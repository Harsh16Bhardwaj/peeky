# Changelog

## 2.0.0 - 2026-08-13

- Added always-on break flood protection with a four-active-minute look-ahead window.
- Superior breaks now bundle and cover lower-priority wellness actions.
- Added persisted active-time quiet periods after completion, snooze, and skip.
- Added shield and bundle status to the Quick Panel, warnings, overlays, and diagnostics logs.
- Migrated runtime state to schema v2 without resetting existing timer progress.

## 1.2.0 - 2026-08-12

- Added opt-in foreground application tracking with idle, lock, sleep, private exclusion, and break-overlay handling.
- Added exact two-active-hour sessions, midnight partial sessions, three-minute review aggregation, local classifications, and future-only rules.
- Added the Session, Today, and Trends dashboard plus Quick Panel and tray tracking controls.
- Added WAL-mode SQLite activity storage, 90-day retention, JSON/CSV export, and complete history deletion.
- Records Google Chrome as one foreground application without inspecting tabs, page titles, URLs, or browsing content.
- Migrated settings schema 2 to schema 3 with activity tracking disabled until explicit consent.

## 1.1.0 - 2026-08-11

- Prevented the release executable from opening a terminal window.
- Refined all four break scenes with quieter, more readable full-screen layouts.
- Changed the default heads-up duration to five seconds and migrated version 1 settings.
- Fixed light-theme pause options and removed transparent window-edge artifacts.

## 1.0.0 - 2026-08-11

- Rebuilt Peeky as a focused Tauri 2 Windows break companion.
- Added active-time timer hierarchy, priority resets, idle-aware scheduling, tray controls, full-screen reminders, settings, diagnostics, and NSIS packaging.
- Archived the nonfunctional Python prototype under `legacy/`.
