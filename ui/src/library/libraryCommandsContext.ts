import { createContext } from "react";

import type { LibraryCommandPort } from "../api/libraryCommands";

export const LibraryCommandsContext = createContext<LibraryCommandPort | null>(
  null
);
