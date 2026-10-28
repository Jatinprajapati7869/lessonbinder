import { createContext } from "react";

import type { SettingsCommandPort } from "../api/settingsCommands";

export const SettingsCommandsContext =
  createContext<SettingsCommandPort | null>(null);
