import { useContext } from "react";

import type { CaptureCommandPort } from "../api/captureCommands";
import { CaptureCommandsContext } from "./captureCommandsContext";

export function useCaptureCommands(): CaptureCommandPort {
  const ctx = useContext(CaptureCommandsContext);
  if (!ctx) {
    throw new Error("useCaptureCommands requires CaptureCommandsProvider");
  }
  return ctx;
}
