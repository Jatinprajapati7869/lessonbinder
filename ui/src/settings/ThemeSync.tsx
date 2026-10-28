import { useEffect } from "react";

import { applyUiTheme } from "../theme/applyUiTheme";
import { useSettingsCommands } from "./useSettingsCommands";

/** Loads persisted theme from vault settings once the app shell is ready. */
export function ThemeSync() {
  const commands = useSettingsCommands();

  useEffect(() => {
    void commands.getVaultSettings().then((settings) => {
      applyUiTheme(settings.ui_theme);
    });
  }, [commands]);

  return null;
}
