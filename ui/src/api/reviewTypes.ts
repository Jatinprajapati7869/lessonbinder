export type ReviewAnswerButtons = {
  again: boolean;
  hard: boolean;
  good: boolean;
  easy: boolean;
};

export type ReviewCard = {
  card_id: number;
  note_id: number;
  front_text: string;
  screenshot_filename: string | null;
  buttons: ReviewAnswerButtons;
};

export type ReviewEase = "again" | "hard" | "good" | "easy";
