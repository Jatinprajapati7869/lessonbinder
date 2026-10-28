//! Study queue counts and in-process review entry for Lessonbinder decks.

use std::collections::HashSet;

use anki::prelude::*;
use anki::scheduler::answering::CardAnswer;
use anki::scheduler::answering::Rating;
use anki::scheduler::states::CardState;
use anki::scheduler::states::FilteredState;
use anki::services::CardsService;
use anki::services::NotesService;
use anki_proto::cards::CardId as ProtoCardId;
use anki_proto::decks::DeckTreeNode;
use anki_proto::notes::NoteId as ProtoNoteId;
use thiserror::Error;

use crate::clip::{LESSONBINDER_CLIP_FIELDS, LESSONBINDER_DECK_PREFIX};

/// New / learning / review counts for all `Lessonbinder::<course>` decks combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DueSummary {
    pub new: u32,
    pub learning: u32,
    pub review: u32,
}

/// Grades the reviewer may offer for the current card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewAnswerButtons {
    pub again: bool,
    pub hard: bool,
    pub good: bool,
    pub easy: bool,
}

/// Answer button chosen by the learner (maps to Anki `Rating`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewEase {
    Again,
    Hard,
    Good,
    Easy,
}

/// Next due card in the Lessonbinder deck subtree, ready to show in-app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewCard {
    pub card_id: CardId,
    pub note_id: NoteId,
    /// Question-side field texts for the card template (Front, then Screenshot for clip notes).
    pub front_fields: Vec<String>,
    pub buttons: ReviewAnswerButtons,
}

/// One graded card during an in-memory review session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewSessionAnswer {
    pub card_id: CardId,
    pub ease: ReviewEase,
}

/// In-memory review session (answer log + operations on an open collection).
#[derive(Debug, Default)]
pub struct ReviewSession {
    answers: Vec<ReviewSessionAnswer>,
}

/// Begin a review session. Does not create notes or cards.
pub fn start_review_session(_col: &Collection) -> ReviewSession {
    ReviewSession::default()
}

impl ReviewSession {
    pub fn answers(&self) -> &[ReviewSessionAnswer] {
        &self.answers
    }

    /// Grade `card_id` and append to the session log. Fails if the card is not in a Lessonbinder deck.
    pub fn answer(
        &mut self,
        col: &mut Collection,
        card_id: CardId,
        ease: ReviewEase,
    ) -> Result<(), AnswerCardError> {
        ensure_card_in_lessonbinder_decks(col, card_id)?;
        answer_card(col, card_id, ease)?;
        self.answers.push(ReviewSessionAnswer { card_id, ease });
        Ok(())
    }

    /// End the session. Does not bury, suspend, or otherwise alter cards beyond prior answers.
    pub fn close(self) -> Vec<ReviewSessionAnswer> {
        self.answers
    }
}

#[derive(Debug, Error)]
pub enum AnswerCardError {
    #[error("no Lessonbinder deck in collection")]
    NoLessonbinderDeck,
    #[error("no cards due in Lessonbinder study queue")]
    EmptyQueue,
    #[error("card is not at the head of the Lessonbinder study queue")]
    CardNotAtQueueHead,
    #[error("card is not in a Lessonbinder deck")]
    CardNotInLessonbinderDecks,
    #[error("scheduler does not offer {0:?} for this card")]
    EaseNotOffered(ReviewEase),
    #[error(transparent)]
    Anki(#[from] anki::error::AnkiError),
}

/// Due counts from Anki's deck tree for the `Lessonbinder::` subtree only.
pub fn due_summary(col: &mut Collection) -> Result<DueSummary> {
    let tree = col.deck_tree(Some(TimestampSecs::now()))?;
    Ok(due_summary_from_tree(&tree))
}

/// Next card due in decks under the top-level `Lessonbinder` deck, or `None` when no Lessonbinder
/// card appears in the next slice of the scoped study queue.
///
/// Foreign due cards ahead in the scheduler order are skipped (not answered or buried).
/// Uses `set_current_deck` + `get_queued_cards`; does not query SQL directly.
pub fn next_review_card(col: &mut Collection) -> Result<Option<ReviewCard>> {
    let Some(root_id) = lessonbinder_root_deck_id(col)? else {
        return Ok(None);
    };
    next_lessonbinder_review_card(col, root_id, None)
}

/// Next due Lessonbinder card whose note id is in `allowed_note_ids`.
pub fn next_review_card_for_course(
    col: &mut Collection,
    allowed_note_ids: &HashSet<NoteId>,
) -> Result<Option<ReviewCard>> {
    let Some(root_id) = lessonbinder_root_deck_id(col)? else {
        return Ok(None);
    };
    next_lessonbinder_review_card(col, root_id, Some(allowed_note_ids))
}

/// Same as [`next_review_card_for_course`] with vault link note ids (`i64`).
pub fn next_review_card_for_course_note_ids(
    col: &mut Collection,
    allowed_note_ids: &HashSet<i64>,
) -> Result<Option<ReviewCard>> {
    let allowed: HashSet<NoteId> = allowed_note_ids.iter().copied().map(NoteId).collect();
    next_review_card_for_course(col, &allowed)
}

/// Grade the card at the head of the Lessonbinder study queue.
///
/// `card_id` must match the card returned by the most recent `next_review_card` on this collection.
/// `answer_card` with primitive ids for IPC boundaries.
pub fn answer_card_by_id(
    col: &mut Collection,
    card_id: i64,
    ease: ReviewEase,
) -> Result<(), AnswerCardError> {
    answer_card(col, CardId(card_id), ease)
}

pub fn answer_card(
    col: &mut Collection,
    card_id: CardId,
    ease: ReviewEase,
) -> Result<(), AnswerCardError> {
    let Some(root_id) = lessonbinder_root_deck_id(col)? else {
        return Err(AnswerCardError::NoLessonbinderDeck);
    };
    col.set_current_deck(root_id)?;
    let queue_head = col.get_next_card()?;
    let Some(review) = next_lessonbinder_review_card(col, root_id, None)? else {
        return Err(AnswerCardError::EmptyQueue);
    };
    if review.card_id != card_id {
        return Err(AnswerCardError::CardNotAtQueueHead);
    }
    ensure_card_in_lessonbinder_decks(col, card_id)?;
    col.set_current_deck(root_id)?;
    let Some(queued) = col
        .get_queued_cards(64, false)?
        .cards
        .into_iter()
        .find(|q| q.card.id() == card_id)
    else {
        return Err(AnswerCardError::EmptyQueue);
    };
    let buttons = answer_buttons_for_state(&queued.states.current);
    ensure_ease_offered(buttons, ease)?;

    let (rating, new_state) = match ease {
        ReviewEase::Again => (Rating::Again, queued.states.again),
        ReviewEase::Hard => (Rating::Hard, queued.states.hard),
        ReviewEase::Good => (Rating::Good, queued.states.good),
        ReviewEase::Easy => (Rating::Easy, queued.states.easy),
    };

    let mut answer = CardAnswer {
        card_id,
        current_state: queued.states.current,
        new_state,
        rating,
        answered_at: TimestampMillis::now(),
        milliseconds_taken: 0,
        custom_data: None,
        from_queue: queue_head
            .as_ref()
            .is_some_and(|head| head.card.id() == card_id),
    };
    col.answer_card(&mut answer)?;
    Ok(())
}

fn next_lessonbinder_review_card(
    col: &mut Collection,
    root_id: DeckId,
    allowed_note_ids: Option<&HashSet<NoteId>>,
) -> Result<Option<ReviewCard>> {
    col.set_current_deck(root_id)?;
    for queued in col.get_queued_cards(64, false)?.cards {
        if !card_in_lessonbinder_decks(col, &queued.card)? {
            continue;
        }
        let note_id = queued.card.note_id();
        if let Some(allowed) = allowed_note_ids {
            if !allowed.contains(&note_id) {
                continue;
            }
        }
        let note: Note = NotesService::get_note(
            col,
            ProtoNoteId {
                nid: queued.card.note_id().0,
            },
        )?
        .into();
        let front_fields = front_fields_for_note(&note)?;
        return Ok(Some(ReviewCard {
            card_id: queued.card.id(),
            note_id: queued.card.note_id(),
            front_fields,
            buttons: answer_buttons_for_state(&queued.states.current),
        }));
    }
    Ok(None)
}

fn ensure_card_in_lessonbinder_decks(
    col: &mut Collection,
    card_id: CardId,
) -> Result<(), AnswerCardError> {
    let proto = CardsService::get_card(col, ProtoCardId { cid: card_id.0 })?;
    let deck_id = DeckId(proto.deck_id);
    let deck = col.get_deck(deck_id)?.or_not_found(deck_id)?;
    let name = deck.human_name();
    if name.starts_with(LESSONBINDER_DECK_PREFIX) || name == lessonbinder_root_name() {
        Ok(())
    } else {
        Err(AnswerCardError::CardNotInLessonbinderDecks)
    }
}

fn ensure_ease_offered(
    buttons: ReviewAnswerButtons,
    ease: ReviewEase,
) -> Result<(), AnswerCardError> {
    let offered = match ease {
        ReviewEase::Again => buttons.again,
        ReviewEase::Hard => buttons.hard,
        ReviewEase::Good => buttons.good,
        ReviewEase::Easy => buttons.easy,
    };
    if offered {
        Ok(())
    } else {
        Err(AnswerCardError::EaseNotOffered(ease))
    }
}

fn front_fields_for_note(note: &Note) -> Result<Vec<String>> {
    let fields = note.fields();
    require!(
        fields.len() == LESSONBINDER_CLIP_FIELDS.len(),
        "unexpected field count on review note"
    );
    Ok(vec![fields[2].clone(), fields[6].clone()])
}

pub(crate) fn answer_buttons_for_state(current: &CardState) -> ReviewAnswerButtons {
    match current {
        CardState::Filtered(FilteredState::Preview(_)) => ReviewAnswerButtons {
            again: true,
            hard: true,
            good: true,
            easy: false,
        },
        _ => ReviewAnswerButtons {
            again: true,
            hard: true,
            good: true,
            easy: true,
        },
    }
}

fn card_in_lessonbinder_decks(col: &mut Collection, card: &Card) -> Result<bool> {
    let deck = col.get_deck(card.deck_id())?.or_not_found(card.deck_id())?;
    let name = deck.human_name();
    Ok(name.starts_with(LESSONBINDER_DECK_PREFIX) || name == lessonbinder_root_name())
}

fn lessonbinder_root_name() -> &'static str {
    LESSONBINDER_DECK_PREFIX.trim_end_matches("::")
}

fn lessonbinder_root_deck_id(col: &Collection) -> Result<Option<DeckId>> {
    col.get_deck_id(lessonbinder_root_name())
}

fn due_summary_from_tree(tree: &DeckTreeNode) -> DueSummary {
    let node = tree
        .children
        .iter()
        .find(|child| lessonbinder_root_deck(child));
    match node {
        Some(n) => DueSummary {
            new: n.new_count,
            learning: n.learn_count,
            review: n.review_count,
        },
        None => DueSummary::default(),
    }
}

/// Top-level deck node named `Lessonbinder` (parent of `Lessonbinder::<course>` leaves).
fn lessonbinder_root_deck(node: &DeckTreeNode) -> bool {
    node.level == 1 && node.name == lessonbinder_root_name()
}

#[cfg(test)]
mod tests {
    use anki::scheduler::states::PreviewState;

    use super::*;

    #[test]
    fn rejects_unoffered_easy_on_preview_card() {
        let current = CardState::Filtered(FilteredState::Preview(PreviewState {
            scheduled_secs: 60,
            finished: false,
        }));
        let buttons = answer_buttons_for_state(&current);
        assert!(matches!(
            ensure_ease_offered(buttons, ReviewEase::Easy),
            Err(AnswerCardError::EaseNotOffered(ReviewEase::Easy))
        ));
    }
}
