import { useContext } from "react";

import type { ReviewCommandPort } from "../api/reviewCommands";
import { ReviewCommandsContext } from "./reviewCommandsContext";

export function useReviewCommands(): ReviewCommandPort {
  const ctx = useContext(ReviewCommandsContext);
  if (!ctx) {
    throw new Error("useReviewCommands requires ReviewCommandsProvider");
  }
  return ctx;
}
