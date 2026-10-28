import { createContext, useContext, type ReactNode } from "react";

import type { WeeklyReportCommandPort } from "../api/weeklyReportCommands";
import { tauriWeeklyReportCommands } from "../api/weeklyReportCommands";

const WeeklyReportCommandsContext =
  createContext<WeeklyReportCommandPort | null>(null);

export type WeeklyReportCommandsProviderProps = {
  children: ReactNode;
  commands?: WeeklyReportCommandPort;
};

export function WeeklyReportCommandsProvider({
  children,
  commands = tauriWeeklyReportCommands,
}: WeeklyReportCommandsProviderProps) {
  return (
    <WeeklyReportCommandsContext.Provider value={commands}>
      {children}
    </WeeklyReportCommandsContext.Provider>
  );
}

export function useWeeklyReportCommands(): WeeklyReportCommandPort {
  const commands = useContext(WeeklyReportCommandsContext);
  if (commands === null) {
    throw new Error("WeeklyReportCommandsProvider is missing");
  }
  return commands;
}
