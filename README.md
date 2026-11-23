# VRC Toolbox

this project was almost AI generated.

👇**AI**💻**Generated**👇

VRC Toolbox is a small Windows desktop app that helps you control common VRChat features without leaving the desktop.

It uses VRChat's OSC connection to send chatbox messages and avatar eye-height changes from one simple interface.

## What you can do

- **Send chatbox messages** to VRChat using OSC.
- **Reuse recent messages** and keep frequently used text as templates.
- **Repeat saved messages** in order, then stop them at any time.
- **Change avatar eye height** and save your preferred height values for quick reuse.
- **Check the OSC connection** and adjust the local OSC target if needed.

Your messages, templates, settings, and saved height values stay on this device.

## Download and install

1. Open the [latest release](https://github.com/RichardLexJiang/vrc-toolbox/releases/latest).
2. Download the Windows MSI or NSIS installer from the release's **Assets** section.
3. Run the installer and open VRC Toolbox.
4. Start VRChat and make sure OSC is enabled.

The app uses the standard local OSC address, `127.0.0.1:9000`, by default. If your VRChat setup uses another address or port, open **Settings** in VRC Toolbox and update the OSC target.

## Using VRC Toolbox

### Chatbox

Write a source message of up to 200 characters and click **Send message**. Press **Shift+Enter** as a shortcut when composing. The 200-character limit applies to the message text before dynamic variables are rendered; rendered/send behavior is unchanged.

Dynamic variables can be added to messages using `${...}` tokens. The supported variables are:

| Token | Description |
| --- | --- |
| `${cpu_usage}` | Overall CPU usage as a percentage. |
| `${memory_usage}` | Used physical memory as a percentage of total RAM. |
| `${total_ram}` | Total physical RAM in GiB. |
| `${used_ram}` | Used physical RAM in GiB. |
| `${local_time}` | Current local time in `HH:MM` format. |
| `${local_time("%Y-%m-%d %H:%M:%S")}` | Current local time in a custom [Chrono `strftime` format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html). |
| `${dots}` | Three dots by default; while repeating, cycles from one to three dots. |
| `${dots(6)}` | Up to six dots; while repeating, cycles from one to six dots. The count must be from 1 to 6. |

Use the **Dynamic variables** list above the composer to see supported tokens and examples. Click a token or drag it into your message to insert it. The preview renders dynamic values in the desktop app.

Recent messages appear in the history panel. Selecting one puts it back in the composer without sending it automatically.

To repeat messages:

1. Create or save one or more templates.
2. Select the templates you want to repeat.
3. Click **Repeat selected**.
4. Click **Stop** when you are finished.

### Avatar height

Enter an avatar eye-height value from `0.01` to `100`, then click **Apply**. You can also save a value and apply it again later from your saved shortcuts.

### Settings

Use **Settings** to view the OSC connection status and change the OSC target. The default VRChat address is usually correct and does not need to be changed.

## Troubleshooting

- **Messages are not appearing in VRChat:** Confirm that VRChat is running, OSC is enabled, and VRC Toolbox shows a connection. Check that the OSC target in **Settings** matches your VRChat setup.
- **The Avatar page does not respond:** Make sure an avatar is loaded and OSC is enabled in VRChat.
- **Nothing works in the browser preview:** The browser preview can show the interface, but OSC commands require the Windows desktop app.

## For developers

VRC Toolbox is built with Tauri 2, React, TypeScript, Rust, Vite, and pnpm.

### Requirements

- Windows 10 or later
- Node.js 22
- pnpm 10
- Rust stable
- Microsoft C++ Build Tools
- WebView2 Runtime

### Set up the project

```powershell
pnpm install
pnpm tauri dev
```

To preview the interface in a browser:

```powershell
pnpm dev
```

The browser preview is useful for UI work, but it cannot send OSC commands.

### Build the app

Build and type-check the frontend:

```powershell
pnpm build
```

Build the Windows desktop app and installers:

```powershell
pnpm tauri build
```

The Windows MSI and NSIS installers are generated under `src-tauri/target/release/bundle/`.

## Project documentation

- [Avatar page specification](docs/specifications/avatar-page.md)
- [Chatbox page specification](docs/specifications/chatbox-page.md)

## App icon

The source icon is [`public/app-icon.svg`](public/app-icon.svg). After editing it, regenerate the Tauri icons with:

```powershell
pnpm tauri icon public/app-icon.svg
```
