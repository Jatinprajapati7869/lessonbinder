import {
  formatShortcutKeysForSettings,
  IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS,
} from "../keyboard/implementedShortcuts";

export function SettingsKeyboardShortcutsList() {
  return (
    <div className="settings-field" data-testid="settings-keyboard-shortcuts">
      <span className="settings-field__label">Keyboard shortcuts</span>
      <ul className="settings-shortcuts">
        {IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS.map((row, index) => (
          <li
            key={`${row.scope}-${row.action}-${index}`}
            className="settings-shortcuts__row"
            data-testid={`settings-shortcut-row-${index}`}
          >
            <span className="settings-shortcuts__key">
              {formatShortcutKeysForSettings(row.keys)}
            </span>
            <span className="settings-shortcuts__action">{row.action}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}
