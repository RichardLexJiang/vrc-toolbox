# Chatbox page specification

## Purpose

Provide a focused place to compose and preview a VRChat chatbox message, send it through the desktop app, and reuse a recent message.

## Page layout

- Keep the page within the existing VRC Toolbox shell and visual language.
- Introduce the page with a short explanation of composing and revisiting messages.
- Use a two-column workspace on wide screens:
  - **Compose a message:** a multiline editor, character count, keyboard shortcut hint, live preview, and send action.
  - **Recent messages:** the local chatbox history.
- Collapse the workspace into a single column on narrower screens.

## Composer behavior

- Limit messages to 144 characters and display the current count.
- Keep Enter available for adding a newline.
- Pressing **Shift+Enter** submits the same form as the Send message button.
- Do not trigger the shortcut while an input method editor is composing text.
- An emoji picker offers categorized choices; selecting one inserts it at the current cursor or replaces selected text.
- Emojis can be combined with text and sent using either the send button or **Shift+Enter**.
- Emoji insertion respects the 144-character message limit.
- Disable editing and history selection while a message is being sent.
- Selecting a history entry fills the composer; it does not send automatically.

## History behavior

- Load history from the existing `get_chatbox_history` Tauri command when the Chatbox page opens.
- Refresh the list after a successful send.
- Display the newest entry first, showing its message and local date/time.
- Show distinct loading, empty, browser-preview, and error states.
- Reuse a selected message after trimming it to the composer’s 144-character limit.

## Template repeating behavior

- Allow editing the name and message content of a saved template.
- Limit template names to 40 characters and template content to 144 characters.
- Allow selecting multiple saved templates for a repeating sequence.
- Repeat selected templates in their saved-list order, with no user-configurable interval.
- Show the messages in the active sequence and provide a stop action while repeating.

## Accessibility

- Keep a visible shortcut hint beside the character count.
- Use semantic form controls and buttons so keyboard users can compose, submit, and select history entries.
- Give each repeat-selection checkbox an accessible name that identifies its template.
- Announce history loading and error states to assistive technology.
