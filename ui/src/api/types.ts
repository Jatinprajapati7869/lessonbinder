export type DueSummary = {
  new: number;
  learning: number;
  review: number;
};

export type AnkiLaunchStatus =
  { kind: "available" } | { kind: "unavailable"; message: string };

export type StudyBarState = {
  due: DueSummary;
  anki: AnkiLaunchStatus;
};

export type VaultBootstrap = {
  ready: boolean;
  path: string;
  error: string | null;
};
