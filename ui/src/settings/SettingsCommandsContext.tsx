import type { ReactNode } from "react";

import type { SettingsCommandPort } from "../api/settingsCommands";
import { tauriSettingsCommands } from "../api/settingsCommands";
import { SettingsCommandsContext } from "./settingsCommandsContext";

export function SettingsCommandsProvider({
  commands,
  children,
}: {
  commands?: SettingsCommandPort;
  children: ReactNode;
}) {
  return (
    <SettingsCommandsContext.Provider value={commands ?? tauriSettingsCommands}>
      {children}
    </SettingsCommandsContext.Provider>
  );
}
