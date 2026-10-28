import { useContext } from "react";

import type { SettingsCommandPort } from "../api/settingsCommands";
import { SettingsCommandsContext } from "./settingsCommandsContext";

export function useSettingsCommands(): SettingsCommandPort {
  const ctx = useContext(SettingsCommandsContext);
  if (!ctx) {
    throw new Error("useSettingsCommands requires SettingsCommandsProvider");
  }
  return ctx;
}
