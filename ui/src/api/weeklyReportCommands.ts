import { invoke } from "@tauri-apps/api/core";

import type { WeeklyReport } from "./weeklyReportTypes";

export type WeeklyReportCommandPort = {
  getWeeklyReport(): Promise<WeeklyReport>;
};

export const tauriWeeklyReportCommands: WeeklyReportCommandPort = {
  getWeeklyReport: () => invoke<WeeklyReport>("get_weekly_report"),
};
