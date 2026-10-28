export type CardDraft = {
  course: string;
  lesson_title: string;
  front: string;
  back: string;
  source_label: string;
  timestamp: string;
};

export type CommittedCard = {
  duplicate: boolean;
  note_id: number;
  course: string;
  lesson_title: string;
  front: string;
  back: string;
  source_label: string;
  timestamp: string;
};

export type CaptureOcrStatus = {
  available: boolean;
};
