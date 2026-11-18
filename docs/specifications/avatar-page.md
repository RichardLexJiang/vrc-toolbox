# Avatar page specification

## Purpose

Provide a focused tool to set the current VRChat avatar's eye-height value and keep frequently used values as local shortcuts.

## Page layout

- Keep the page in the existing VRC Toolbox shell and visual language.
- Introduce the control with the supported value range and a reminder that applying a value sends it to the current avatar.
- Show a numeric height input, an apply action, and a list of saved height shortcuts.
- Each saved shortcut can be applied directly or removed.
- Keep the height control and shortcuts in a single full-width column; do not show a separate setup or supporting-information card.

## Height behavior

- Accept finite numeric values from `0.01` through `100`, inclusive, with hundredth precision in the UI.
- Applying the value invokes the `change_avatar_height` Tauri command with `{ height: number }`.
- Do not persist the last applied value as a setting.
- Show success and command errors to the user; do not report success when the OSC command fails.
- In a browser preview, do not attempt to invoke the desktop command; explain that applying requires the desktop app.

## Shortcut behavior

- Load the shortcut list when the Avatar page opens using `get_avatar_height_values`.
- Add the currently entered value with `save_avatar_height_values({ heightValues: number[] })`.
- Reject duplicate values in the UI. The backend also removes duplicates while preserving order.
- Clicking a shortcut applies that value immediately; removing a shortcut updates the persisted list.
- Keep shortcut values in insertion order. An empty list is valid.
- Persist the list in the app's local configuration directory, separate from general settings.
- In a browser preview, shortcut edits are held in memory only and are not persisted.
- Display loading, empty, and error states. If loading fails, do not overwrite the unavailable saved list.
- Preserve the current list if a save fails, and surface the error so the user can retry.

## Tauri commands

- `change_avatar_height(height: number) -> void`: send an eye-height OSC value; reject values outside `0.01..=100` or non-finite values.
- `get_avatar_height_values() -> number[]`: return saved values, or an empty list when no list has been saved yet.
- `save_avatar_height_values(heightValues: number[]) -> number[]`: validate, de-duplicate, persist, and return the saved values. Invalid input returns an error.

## Accessibility

- Associate the numeric input with a visible label and describe its range.
- Use semantic buttons with accessible labels for applying and removing each shortcut.
- Announce loading and saving states and expose errors through alert/status roles.
