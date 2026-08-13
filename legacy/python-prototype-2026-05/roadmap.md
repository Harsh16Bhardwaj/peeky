🗺️ Peeky — Phased Build Roadmap
🟦 FOUNDATION LAYER
Node 1 — Project Skeleton + Dev Environment
Concerns: folder structure, requirements.txt, config.json schema, AGENT.md + instructions.md + architecture.md + focus.md + updates.md + deviations.md all created with initial content
Deliverable: repo is cloned, venv works, all files exist, app runs and exits cleanly

Node 2 — Event Log + Append Writer
Concerns: events.py — newline-delimited JSON append writer, daily log rotation, event schema definitions
Deliverable: calling log_event(type, payload) writes a line to logs/events-YYYY-MM-DD.log

Node 3 — Persistence + Atomic Writes
Concerns: persistence.py — safe write protocol (write tmp → fsync → copy to prev → os.replace), snapshot flush on interval, backup rotation to data/backups/
Deliverable: save_snapshot() never corrupts a file even if interrupted mid-write

Node 4 — Recovery Engine
Concerns: recovery.py — three-step recovery: try current snapshot → try prev → rebuild from event log
Deliverable: deleting app_state.json mid-run and restarting recovers all state correctly

Node 5 — Thread-Safe State Model
Concerns: state.py — AppState dataclass, threading.Lock, getters/setters, today counters, timer states
Deliverable: multiple threads can read/write state without race conditions, verified with a simple stress test

🟨 CORE ENGINE
Node 6 — Supervisor + Thread Lifecycle
Concerns: supervisor.py — starts all worker threads, heartbeat monitoring, auto-restart on thread death, logs restarts
Deliverable: kill a worker thread manually, supervisor detects and restarts it within 5 seconds

Node 7 — Idle Detection
Concerns: idle.py — GetLastInputInfo via ctypes, idle vs non-idle transition events, idle threshold from config, logs idle_started / idle_ended events
Deliverable: walking away from PC for 5 min logs idle segment correctly

Node 8 — Multi-Break Timer Engine
Concerns: timers.py — independent timer objects loaded from config, monotonic elapsed time, due evaluation loop, snooze/skip/complete actions, merge window for coalescing simultaneous reminders
Deliverable: all three timers (blink/lookaway/walk) fire independently and correctly after restart

Node 9 — Reminder Dispatch + UX
Concerns: reminders.py + ui/notifier.py — popup or toast per reminder type, Done/Snooze/Skip buttons, logs action taken, respects idle suppression
Deliverable: reminders fire at correct times, actions are logged, idle suppression works

Node 10 — Pomodoro Engine
Concerns: add Pomodoro timer to timers.py, work/break session tracking, Pomodoro count per day, log events per session
Deliverable: Pomodoro runs independently alongside break timers, sessions are counted and logged

🟧 TRAY + CONTROLS
Node 11 — Tray Shell
Concerns: tray.py — pystray icon, static right-click menu with all items, tray thread starts cleanly, app exits on Quit
Deliverable: tray icon appears, menu opens, Quit works

Node 12 — Tray Live State + Controls
Concerns: dynamic tray menu labels (next break ETA, active profile, paused state), Pause/Resume wired to AppState, menu refresh on state change, open browser pages from tray
Deliverable: pausing from tray suppresses all reminders, tray label updates correctly

Node 13 — Startup + Mutex + Autostart
Concerns: main.py — named Windows Mutex to prevent double launch, winreg autostart key written on first run and refreshed on every launch, graceful single-instance exit
Deliverable: launching app twice silently kills second instance, app survives reboot

🟩 BROWSER SURFACES — BACKEND
Node 14 — Local Web Server Shell
Concerns: web/server.py — Flask app on localhost:41741, health endpoint, static file serving, thread-safe access to AppState, server starts silently in background
Deliverable: GET /api/health returns app status, server runs without blocking tray or timers

Node 15 — Tabs Backend
Concerns: web/tabs.py — CRUD endpoints for tabs, clipboard read on hotkey saves tab, stale logic, extend-by-1-day logic, open via subprocess chrome.exe
Deliverable: hotkey saves clipboard URL, all CRUD endpoints work correctly

Node 16 — Handoffs Backend
Concerns: web/handoffs.py — CRUD endpoints for handoffs, deadline + lead-time reminder scheduler, reminder fires via reminders.py, auto-archive after retention period
Deliverable: create handoff with deadline, reminder fires at correct lead time

Node 17 — Dashboard Data API
Concerns: web/dashboard.py — aggregate events from log files, compute KPIs (non-idle hours, breaks taken/skipped, Pomodoro count, compliance rate, attention span segments), frequency data for 24h/7d/30d range
Deliverable: GET /api/dashboard/summary and GET /api/dashboard/frequency?range=24h return correct aggregated data

🟪 BROWSER SURFACES — FRONTEND
Node 18 — Tabs Page UI
Concerns: web/tabs.html + JS — teal theme, list sorted by recency, per-tab open/delete/extend buttons, open-all button, stale tabs visually muted, manual add URL field
Deliverable: full tabs UI works end-to-end in browser

Node 19 — Handoffs Page UI
Concerns: web/handoffs.html + JS — teal theme, sorted upcoming first, create form (name, deadline, lead time, context, link), status chips, done/archive actions
Deliverable: full handoffs UI works end-to-end in browser

Node 20 — Dashboard UI
Concerns: web/dashboard.html + Chart.js — teal theme, KPI strip (6–8 metrics), line chart for work frequency, 24h/7d/30d toggle, attention span section, consistency section
Deliverable: dashboard loads, KPIs populate from real logged data, chart renders and toggles correctly

🟥 PACKAGING + RELEASE
Node 21 — Settings Window
Concerns: ui/settings.py — PyQt5 settings window, edit timer intervals/durations, enable/disable each timer, profile management (save/load), config persisted on save
Deliverable: changing blink interval in settings takes effect on next timer cycle without restart

Node 22 — Eye Exercise Overlay
Concerns: ui/overlay.py — PyQt5 fullscreen overlay, animated countdown ring, exercise prompt carousel, Skip/Snooze/Done buttons, multi-monitor support, DPI-aware, fade in/out
Deliverable: overlay fires on lookaway timer, looks polished, works on 1080p and HiDPI

Node 23 — Build + Installer
Concerns: build.py + installer/peeky.nsi — PyInstaller onefile windowed build, all hidden imports, NSIS installer with shortcuts/uninstaller/Start Menu, version string in binary
Deliverable: .exe installer runs on a clean Windows machine, app starts and works fully

Node 24 — Auto-Update + GitHub Release
Concerns: on-startup GitHub Releases API ping, version comparison, tray notification if update available, CHANGELOG.md, GitHub Actions CI that auto-builds .exe on version tag push
Deliverable: tagging v1.0.0 on GitHub triggers a CI build and publishes the installer as a release artifact