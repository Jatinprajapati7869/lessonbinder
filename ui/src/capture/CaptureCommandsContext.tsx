import type { ReactNode } from "react";

import type { CaptureCommandPort } from "../api/captureCommands";
import { tauriCaptureCommands } from "../api/captureCommands";
import { CaptureCommandsContext } from "./captureCommandsContext";

export function CaptureCommandsProvider({
  commands,
  children,
}: {
  commands?: CaptureCommandPort;
  children: ReactNode;
}) {
  return (
    <CaptureCommandsContext.Provider value={commands ?? tauriCaptureCommands}>
      {children}
    </CaptureCommandsContext.Provider>
  );
}
