# Lessonbinder review queue vs Anki scheduler

Lessonbinder in-app review only grades cards in the **`Lessonbinder::`** deck subtree. Other due cards in the same profile must not block the session and must not be answered, buried, or suspended by Lessonbinder.

## Public API used (no raw SQL)

| Method                                                              | Role                                                                                                                                                                                                 |
| ------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Collection::set_current_deck`                                      | Scope the built study queue to the top-level **`Lessonbinder`** deck (includes `Lessonbinder::<course>` children).                                                                                   |
| `Collection::get_queued_cards(fetch_limit, intraday_learning_only)` | Returns the next cards in scheduler order with template states for answering. Implemented in Anki rslib as the same pipeline as `get_next_card()` (which is `get_queued_cards(1, …).cards.first()`). |
| `Collection::answer_card`                                           | Grade a card the learner chose; only called for ids returned by `next_review_card`.                                                                                                                  |
| `Collection::get_deck` / deck `human_name()`                        | Decide whether a queued card belongs to the Lessonbinder prefix.                                                                                                                                     |

**Skip foreign dues:** `next_review_card` walks the queued slice, **continues** past cards whose deck name is not `Lessonbinder` or `Lessonbinder::…`, and returns the first matching card. Foreign cards are never graded and never buried by this path.

**Stop condition:** If no Lessonbinder card appears within the fetched window (`fetch_limit`, currently 64), the function returns `Ok(None)` even if `due_summary` still shows Lessonbinder dues deeper in the queue. Raising the limit or paging the queue would be a follow-up; it does not require SQL.

## Not used

- Direct SQL on `cards` / `revlog`.
- Bury, suspend, or requeue helpers to “get past” foreign cards.
- Answering foreign cards to drain the global queue.

## Evidence tests

- `anki_bridge::study::next_card_from_lessonbinder_decks` — due **Other** deck card before due Lessonbinder clip; `next_review_card` returns the Lessonbinder front; foreign card stays on the **New** queue.
- `review_session_rejects_card_outside_lessonbinder_prefix` — session `answer` refuses non-prefix ids without mutating the log.
