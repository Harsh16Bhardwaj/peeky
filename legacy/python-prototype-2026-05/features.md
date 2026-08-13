1. 👁️ Eye Break System (20-20-20)
What it does: Every 20 minutes, triggers a fullscreen overlay with a 20-second countdown asking you to look 20 feet away. At the end of each break, auto-dismisses and resets the timer.

Why it exists: The 20-20-20 rule is ophthalmologist-recommended for reducing CVS (Computer Vision Syndrome). Harsh spends long uninterrupted coding sessions and the screen is literally the only thing in his field of view for hours. Most existing Windows tools either don't exist, are paid, or are ugly abandonware. This is the core product — everything else orbits it.

Key behaviours: Fullscreen overlay (all monitors), animated circular countdown, fade in/out animation, Skip button, Postpone (2 min / 5 min) buttons, soft chime alert before overlay appears (10-second heads-up toast).

2. 🍅 Dual Timer Mode — Pomodoro
What it does: A parallel timer mode where work sessions are 25 minutes (configurable) followed by a 5-minute break. Long breaks (15 min) auto-trigger after every 4 Pomodoros. Can be toggled from tray.

Why it exists: The eye break interval and the focus interval are different things — 20 min for eyes, 25 min for focus. Both are valid simultaneously. A student prepping for placements, grinding DSA or working on a project, needs structured focus blocks, not just eye reminders. The Pomodoro mode is the work rhythm layer on top of the eye health layer. Having both in one tray app means zero context-switch cost.

Key behaviours: Independent countdown from eye break timer, visual indicator in tray tooltip showing which mode is active, Pomodoro count (🍅🍅🍅) shown in dashboard, configurable work/break durations.

3. 📊 Session Dashboard
What it does: A clean, minimal window (accessible from tray) showing today's stats: total screen time, breaks taken vs skipped, compliance percentage, hourly activity bar, Pomodoro count, and a 7-day compliance trend bar chart.

Why it exists: You can't improve what you don't measure. After a day of coding, Harsh has no idea if he actually took breaks or just kept skipping them. The dashboard closes the feedback loop — it's the mirror the app holds up. It's also the most visually impressive part of a portfolio demo: a real-time analytics panel for your own body. The 7-day trend is what turns a utility into a habit tool.

Key behaviours: Opens as a non-blocking side panel or small window, all data from local JSON log, today's timeline shown as coloured blocks (green = break taken, red = skipped, grey = idle), streak counter ("4-day compliance streak 🔥"), longest unbroken screen session highlighted in red if > 2 hrs.

4. 📌 Quick Capture / Handoff
What it does: Global hotkey (Ctrl+Shift+Space) opens a tiny always-on-top floating input box. You type a task, paste a URL, write a note, and optionally set a reminder time ("remind me in 2 hrs" or "at 4:30 PM"). It stores to a local list. At the set time, a toast notification fires with a clickable action. Items auto-archive after 24 hours without interaction.

Why it exists: Context switching is the silent productivity killer. Mid-flow, Harsh remembers he needs to send something, check something, or revisit a link. The options are: break flow to handle it now, or forget it. Neither is good. This capture box is the third option — offload it in 3 seconds, trust the app to ping you back. It's not a todo app. It has no projects, no priorities, no categories. Just a capture net with a timer. During the eye break overlay, pending items surface at the bottom — the forced break becomes a natural review moment.

Key behaviours: Input box appears at screen centre, auto-focuses, dismisses on Esc or Enter, reminder field is optional with natural language parsing ("in 2 hrs", "at 5pm", "tomorrow 9am" via dateparser library), items shown in a scrollable list in tray menu, break overlay shows count of pending items.

5. 🗂️ Tab Graveyard
What it does: Two global hotkeys. Ctrl+Shift+S silently reads clipboard and saves it as a tab entry (URL + timestamp). Ctrl+Shift+T opens a floating panel showing all saved tabs with title, age, and two actions: Open (launches in last-used Chrome window) and Delete. "Open All" button at top. Tabs older than 24 hrs are visually greyed out. No automatic deletion.

Why it exists: Harsh's exact problem — open 10 YouTube tabs, plan to watch them, forget, they sit eating RAM. The real issue is the tab intention — "I meant to watch this" — gets lost. This is a manual clipboard-powered tab queue. The UX is deliberately minimal: no form, no dialog, just copy + keystroke. The "last used Chrome window" targeting means it doesn't open a new Chrome instance, it slots tabs into your existing session. The 24-hr visual decay is guilt-free: greyed out = probably stale, but you decide when to delete.

Key behaviours: pygetwindow + win32gui to find last Chrome HWND, subprocess call to chrome.exe --new-tab <url>, panel is a slim always-on-top window, tabs stored in tabs.json, favicon fetched async and cached locally for visual richness in the panel.

6. 🧘 Eye Exercise Carousel on Overlay
What it does: During each eye break, the overlay doesn't just show a blank countdown — it cycles through a different eye exercise prompt each time: "Blink rapidly 10 times", "Focus on something 20 feet away", "Roll your eyes slowly in a circle", "Alternate focus: thumb close, object far", "Close eyes, palm them gently for 10 seconds".

Why it exists: A blank overlay with a countdown gets ignored and mentally skipped even when taken. The exercise prompt gives the break purpose — you're not just waiting, you're doing something specific. It also subtly educates the user over time. Rotating prompts means it never feels repetitive. This costs almost nothing to implement (a list of strings + an index counter) but dramatically improves the quality of each break.

Key behaviours: 5–8 exercises in a rotating list, shown as large readable text with a small icon/emoji, random shuffle order, displayed prominently on overlay above the countdown ring.

7. ⚙️ Profiles System
What it does: Pre-defined named configs — "Work" (20-20-20 + Pomodoro both active, strict mode on), "Study" (25/5 Pomodoro only, no strict mode), "Casual" (60-min reminders, soft toast only, no fullscreen). One-click switch from tray right-click menu. Custom profiles can be created and saved.

Why it exists: Harsh's usage context changes across the day — deep coding sessions need strict enforcement, casual browsing doesn't. Switching the entire app behaviour with one tray click removes the friction of going into settings. Profiles also make the app demonstrably smarter in a portfolio context: it's not just a timer, it has contextual awareness baked into the UX.

Key behaviours: Active profile shown in tray tooltip, profiles stored as named entries in config.json, each profile stores all timer settings + strict mode + notification style + break type, switching profile restarts the active timers.

8. 🔕 Smart Skip Logic (Idle + Presentation Detection)
What it does: Before showing any overlay, the app checks: (a) was the system idle for > 5 minutes (user stepped away), (b) is a fullscreen exclusive process running (game, VLC, PowerPoint), (c) is Windows Focus Assist / DND mode active. If any are true, the break is silently skipped and logged as "auto-skipped" in dashboard.

Why it exists: The single most-cited complaint about break reminder apps is false positives — the overlay fires when you're in a meeting, watching a movie, or away from the desk. Every false positive trains the user to distrust and eventually disable the app. Smart skip is what separates a tool people keep installed vs one they uninstall in a week. This is invisible when working correctly, which is exactly the point.

Key behaviours: ctypes.GetLastInputInfo for idle detection, GetForegroundWindow + IsZoomed + process name check for fullscreen detection, SHQueryUserNotificationState for DND/presentation mode, all auto-skips logged with reason in session data.

9. 🔒 Startup, Mutex & Persistence
What it does: On first launch, registers itself to Windows startup via winreg. On every launch, checks for an existing running instance via a named Windows Mutex — if one exists, exits silently. Timer state (last_break_time, session stats) is persisted to config.json using atomic writes (write-to-temp, rename). On wake from sleep, detects elapsed idle time and adjusts timer accordingly.

Why it exists: A background utility that doesn't survive reboots, crashes on double-launch, or resets its timer every time Windows wakes from sleep is unusable. These are the "invisible quality" features — users never notice them when they work, but they destroy trust immediately when they don't. Building this correctly from day one means Harsh never has to patch these in later. It also demonstrates production-level thinking in a portfolio context.

Key behaviours: winreg for startup key (always rewritten with current .exe path on launch), CreateMutex via ctypes.windll.kernel32, time.monotonic() for all elapsed time calculations, config.json.tmp atomic write pattern, WM_POWERBROADCAST or tick-count delta for sleep/wake correction.

10. 📦 Packaging & Distribution
What it does: A single peeky.exe built with PyInstaller — no installer needed, just drop and run. Includes a tray icon, a proper app icon, and a README.md. Hosted on GitHub with releases. Optional: a simple landing page (GitHub Pages).

Why it exists: A Python app that requires pip install and running from terminal is a hobby project. An .exe you can hand to a non-technical person is a product. For portfolio purposes, the packaging is the last mile that makes it real. The GitHub releases page with a download link and a clean README is what a recruiter or interviewer actually clicks on.

Key behaviours: PyInstaller --onefile --windowed --icon=peeky.ico, hidden imports for pystray._win32, version string in binary (--version-file), CHANGELOG.md maintained, GitHub Actions CI to auto-build .exe on tag push.

🗺️ Build Priority Order
text
Phase 1 (Day 1): Smart Skip Logic → Break Overlay → Dual Timer → Tray Icon
Phase 2 (Day 2): Dashboard → Profiles → Persistence + Mutex + Startup
Phase 3 (Day 3): Quick Capture → Tab Graveyard → Eye Exercise Carousel
Phase 4 (Day 4): Polish → Packaging → GitHub Release → README