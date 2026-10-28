import type { ReactNode } from "react";

import type { LibraryCommandPort } from "../api/libraryCommands";
import { tauriLibraryCommands } from "../api/libraryCommands";
import { LibraryCommandsContext } from "./libraryCommandsContext";

export function LibraryCommandsProvider({
  commands,
  children,
}: {
  commands?: LibraryCommandPort;
  children: ReactNode;
}) {
  return (
    <LibraryCommandsContext.Provider value={commands ?? tauriLibraryCommands}>
      {children}
    </LibraryCommandsContext.Provider>
  );
}
