import { createContext } from "react";

import type { ReviewCommandPort } from "../api/reviewCommands";

export const ReviewCommandsContext = createContext<ReviewCommandPort | null>(
  null
);
