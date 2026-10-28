import { createContext } from "react";

import type { CaptureCommandPort } from "../api/captureCommands";

export const CaptureCommandsContext = createContext<CaptureCommandPort | null>(
  null
);
