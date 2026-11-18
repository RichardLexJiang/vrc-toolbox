# VRC Toolbox

VRC Toolbox is a Windows desktop app for controlling selected VRChat features through OSC. It is built with Tauri 2, React, TypeScript, and Rust.

## Features

- **Chatbox:** Compose and send messages, add emoji, browse recent messages, and save reusable templates.
- **Repeating messages:** Select saved templates to send as a repeating sequence, and stop the sequence at any time.
- **Avatar height:** Set your avatar's eye-height value and save local shortcuts for frequently used values.
- **OSC settings:** Configure the OSC target and see the connection status from the app.

Chatbox history, templates, settings, and avatar-height shortcuts are stored locally in the app configuration directory. The browser preview is useful for exploring the interface, but OSC actions require the desktop app.

## Requirements

- Windows 10 or later
- Node.js 22
- pnpm 10
- Rust stable toolchain
- Microsoft C++ Build Tools and the WebView2 Runtime (normally available on Windows)

## Development

Install the JavaScript dependencies:

```powershell
pnpm install
```

Run the frontend in a browser:

```powershell
pnpm dev
```

Run the desktop app during development:

```powershell
pnpm tauri dev
```

Build and type-check the frontend:

```powershell
pnpm build
```

Build the Windows desktop app and installers:

```powershell
pnpm tauri build
```

The MSI and NSIS installers are generated under `src-tauri/target/release/bundle/`.

## GitHub Actions

The workflow in `.github/workflows/build.yml` builds the Windows desktop app when manually run or when a version tag starting with `v` is pushed. Manual runs upload the generated MSI and NSIS installers as the `vrc-toolbox-windows-installers` workflow artifact. Pushing a version tag also creates a GitHub Release with generated release notes and attaches both installers.

To publish a release, push a tag such as `v0.1.0`:

```powershell
git tag v0.1.0
git push origin v0.1.0
```

## Project documentation

- [Avatar page specification](docs/specifications/avatar-page.md)
- [Chatbox page specification](docs/specifications/chatbox-page.md)

## App icon

The source icon is `public/app-icon.svg`. After editing it, regenerate the native Tauri icons with:

```powershell
pnpm tauri icon public/app-icon.svg
```
