# Chatbox page specification

## Purpose

Provide a focused place to compose and preview a VRChat chatbox message, send it through the desktop app, and reuse a recent message.

## Page layout

- Keep the page within the existing VRC Toolbox shell and visual language.
- Introduce the page with a short explanation of composing and revisiting messages.
- Use a two-column workspace on wide screens:
  - **Compose a message:** a multiline editor, character count, keyboard shortcut hint, live preview, and send action.
  - **Recent messages:** the local chatbox history.
- Place the supported dynamic-variable list above the workspace.
- Collapse the workspace into a single column on narrower screens.

## Composer behavior

- Limit source messages to 200 characters and display the current count. This limit applies before dynamic variables are rendered; rendered/send behavior remains unchanged.
- List the supported dynamic variables with their usage descriptions and total count.
- Allow inserting a variable token into the editor by dragging it from the list or activating its insert control.
- Render the live preview with the `preview_message` command so dynamic values match the message that will be sent.
- Support the following dynamic variables:
  - `${cpu_usage}`: overall CPU usage as a percentage.
  - `${memory_usage}`: used physical memory as a percentage of total RAM.
  - `${total_ram}` and `${used_ram}`: total and used physical memory in GiB.
  - `${local_time}`: local time in `HH:MM` format by default; an optional quoted Chrono `strftime` format can be provided, such as `${local_time("%Y-%m-%d %H:%M:%S")}`.
  - `${dots}`: three dots by default; while repeating, cycles from one to three dots. An optional maximum count from 1 to 6 can be provided, such as `${dots(6)}`.
- Keep Enter available for adding a newline.
- Pressing **Shift+Enter** submits the same form as the Send message button.
- Do not trigger the shortcut while an input method editor is composing text.
- An emoji picker offers categorized choices; selecting one inserts it at the current cursor or replaces selected text.
- Emojis can be combined with text and sent using either the send button or **Shift+Enter**.
- Emoji and dynamic-variable insertion respect the 200-character source limit.
- Disable editing and history selection while a message is being sent.
- Selecting a history entry fills the composer; it does not send automatically.

## History behavior

- Load history from the existing `get_chatbox_history` Tauri command when the Chatbox page opens.
- Refresh the list after a successful send.
- Display the newest entry first, showing its message and local date/time.
- Show distinct loading, empty, browser-preview, and error states.
- Reuse a selected message after trimming it to the composer’s 200-character limit.

## Template repeating behavior

- Allow editing the name and message content of a saved template.
- Limit template names to 40 characters and template content to 200 characters.
- Allow selecting multiple saved templates for a repeating sequence.
- Repeat selected templates in their saved-list order, with no user-configurable interval.
- Show the messages in the active sequence and provide a stop action while repeating.

## Accessibility

- Keep a visible shortcut hint beside the character count.
- Use semantic form controls and buttons so keyboard users can compose, submit, and select history entries.
- Give each repeat-selection checkbox an accessible name that identifies its template.
- Announce history loading and error states to assistive technology.
