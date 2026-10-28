import { useContext } from "react";

import type { LibraryCommandPort } from "../api/libraryCommands";
import { LibraryCommandsContext } from "./libraryCommandsContext";

export function useLibraryCommands(): LibraryCommandPort {
  const ctx = useContext(LibraryCommandsContext);
  if (!ctx) {
    throw new Error("useLibraryCommands requires LibraryCommandsProvider");
  }
  return ctx;
}
