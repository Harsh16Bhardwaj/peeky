# Peeky

## Which folder is which?

`D:\Peeky` is the project folder. It contains the source code, frontend, Rust/Tauri desktop wrapper, tests, and release scripts.

- `src\` is the Peeky interface: quick panel, settings, dashboard, break warning, break scenes, and styling.
- `src-tauri\` is the Windows application layer: tray menu, scheduler, persistence, activity tracking, and installer configuration.
- `src-tauri\crates\peeky-core\` contains the timer and activity logic with deterministic tests.
- `public\` contains static frontend assets such as icons and images.
- `dist-web\` is generated frontend output. Do not edit it; it is bundled into the desktop app during a release build.
- `dist\Peeky.exe` is a portable build for trying the app directly. It is not an installed application and will not appear in Control Panel.
- `dist\Peeky-Setup-x64.exe` is the installer. Use this when you want Start Menu and uninstall registration.
- `D:\Peeky-build\` is only the Rust/Tauri build cache and intermediate release output. Do not run or edit files there.
- `%LOCALAPPDATA%\Peeky\` is user data: settings, runtime state, activity database, exports, and logs. It is intentionally separate from the application.

## Try Peeky

For a quick portable trial, run `D:\Peeky\dist\Peeky.exe`. It runs in the tray and stores data under `%LOCALAPPDATA%\Peeky`.

For a normal installation, run `D:\Peeky\dist\Peeky-Setup-x64.exe`, finish the installer, and launch Peeky from the Start Menu. The installer is current-user only and does not require administrator access.

To uninstall, use Windows Settings > Apps > Installed apps > Peeky, or the Peeky uninstall entry in the Start Menu. The uninstaller asks whether `%LOCALAPPDATA%\Peeky` should be removed. Existing running Peeky processes are stopped before that question so locked files do not make uninstall fail.

If Peeky is absent from Installed apps, it was not installed with the setup executable; close any portable copy and run `Peeky-Setup-x64.exe` again.

## Developer commands

From `D:\Peeky`:

```powershell
npm.cmd install
npm.cmd run build
.\scripts\build-release.ps1 -SkipTests
```

The release script refreshes `dist\Peeky.exe`, `dist\Peeky-Setup-x64.exe`, the portable zip, and `dist\SHA256SUMS.txt`.

Peeky is a Windows-first break companion and optional local activity journal. It schedules blink, look-away, posture, and walking breaks using active computer time, tracks one foreground application at a time, and groups credited activity into two-hour sessions for review.

## Use

Install `Peeky-Setup-x64.exe`, then launch Peeky from the Start Menu. Peeky starts with Windows by default and remains in the notification area when its windows are closed.

For portable use, extract `Peeky-Portable-x64.zip` and run `Peeky.exe`. Keep the included `WebView2Loader.dll` beside the executable on GNU-built releases.

Portable and installed editions both track foreground applications. Google Chrome is recorded as one application; Peeky does not inspect tabs, titles, URLs, or browsing content.

## Development

Windows 10/11 x64 and Node.js 20+ are required. `setup-dev.ps1` configures Rust and uses the Visual Studio C++ tools when available, with an isolated MINGW64 fallback for non-admin machines.

```powershell
.\scripts\setup-dev.ps1
.\run-dev.bat
```

Build the application and installer with `build-installer.bat`. Release artifacts are copied to `dist/`.

The release build creates `Peeky.exe`, the NSIS installer, the portable archive, and SHA-256 checksums.

## Privacy

Peeky has no accounts, telemetry, cloud storage, screenshots, keystroke capture, browser-tab inspection, page-content access, or network API. Settings and break state remain JSON. Activity data is stored in a local WAL-mode SQLite database under `%LOCALAPPDATA%\Peeky`; application names, timestamps, and category totals remain readable to software running as the same Windows user.
