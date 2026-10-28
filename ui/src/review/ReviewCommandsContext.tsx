import type { ReactNode } from "react";

import type { ReviewCommandPort } from "../api/reviewCommands";
import { tauriReviewCommands } from "../api/reviewCommands";
import { ReviewCommandsContext } from "./reviewCommandsContext";

export function ReviewCommandsProvider({
  commands,
  children,
}: {
  commands?: ReviewCommandPort;
  children: ReactNode;
}) {
  return (
    <ReviewCommandsContext.Provider value={commands ?? tauriReviewCommands}>
      {children}
    </ReviewCommandsContext.Provider>
  );
}
