Absolutely — below is a clean architecture.md style spec you can hand to the model building the app. I’ve kept the scraper/automation system out of scope and optimized this around: always-on tray app, multi-break timers, browser-based utilities, file-based persistence, and crash robustness.

Peeky Architecture
Peeky is a Windows-first background productivity app that lives in the system tray, runs continuously, and provides three core capabilities: break timers, tab parking, and handoffs. The app must stay lightweight, survive crashes and restarts without losing state, and keep all long-term data in plain files rather than a database.

Product Scope
The app has four product surfaces:

System tray application, always running.

Break engine with multiple independent reminder timers.

Browser-based utilities: Dashboard, Tabs, Handoffs.

File-based persistence and recovery.

Out of scope for now:

Scraper automation and cron-style jobs.

Per-window or per-app activity tracking.

Cloud sync, accounts, and multi-device state.

SQLite or any heavyweight storage layer.

Core Principles
The app should be designed around these principles:

Always available: it should boot fast, run quietly, and be controlled primarily from the tray.

Never lose data: no exception, crash, or bad write should wipe timers, logs, handoffs, or tabs.

Simple storage: use JSON and append-only logs, not SQLite.

Background first: browser pages are views over local app state, not the source of truth.

Separation of concerns: timer engine, persistence, tray actions, and browser UI should be isolated modules.

Graceful degradation: if dashboard or browser UI fails, the tray and timer engine must keep working.

Runtime Model
Use a single process, multi-threaded architecture. This avoids IPC complexity while still allowing the tray loop, timer engine, reminder checks, and lightweight local web server to run independently.

Recommended runtime threads:

Main thread: process bootstrap, lifecycle manager, exception hooks.

Tray thread: owns the system tray icon and menu loop.

Timer thread: evaluates all break timers and fires due reminders.

Reminder thread: checks due handoffs and deadline notifications.

Persistence thread: flushes in-memory state to disk, rotates logs, maintains backups.

Web thread: serves the dashboard, tabs page, and handoffs page on localhost.

This split is justified because the tray loop is effectively its own event loop in pystray, and tray menus are built around that icon/menu lifecycle rather than a normal request-response model.

Tray Model
Peeky is fundamentally a tray app. Right-click on the tray icon should expose:

Dashboard

Settings

Tabs

Handoffs

Pause all reminders

Resume reminders

Quit

The tray menu should be dynamic, so labels can reflect live state such as “Paused”, active profile, or next break ETA; pystray supports dynamic menu properties, and external state changes require calling Icon.update_menu to refresh the menu. The tray event loop itself is blocking once started, so it should run in its own thread and never be mixed with timer logic.

Break Engine
The break system must support multiple concurrent reminder streams, not a single timer. Each timer is independently configurable and logs its own events.

Initial built-in timers:

Blink timer: every 10 minutes.

Look away timer: every 20 minutes for 20 seconds.

Walk timer: every 30 minutes for 1 minute.

Each timer should support:

Name

Interval

Break duration

Enabled/disabled

Reminder text

Severity/type

Last fired timestamp

Last completed timestamp

Snoozed-until timestamp

Behavior rules:

All timers run off a shared scheduler loop.

Timers are evaluated against monotonic elapsed time, not wall-clock arithmetic.

If the user is idle, timers should not produce noisy reminders.

If multiple reminders become due together, they should be coalesced into one ordered reminder queue.

A more important break can defer a less important one by a small merge window, e.g. 60–120 seconds.

After wake-from-sleep or crash recovery, timers should resume from persisted state rather than restart from zero.

Reminder Priority
Use this priority order:

Walk

Look away

Blink

If two reminders are due within a merge window:

Show the highest-priority reminder first.

Record the others as delayed, not lost.

Re-evaluate delayed reminders after the active break finishes.

This avoids stacked reminder spam.

Reminder UX
For v1, keep reminder delivery simple:

Blink: small unobtrusive popup or toast.

Look away: focused reminder with countdown.

Walk: more prominent reminder with one-minute countdown.

Avoid complex enforcement in v1. Support only:

Done

Snooze

Skip

Every action must be logged.

Idle Handling
The app should log idle time because idle is useful both for break accuracy and for later dashboard KPIs. For now:

Track idle vs non-idle time globally.

Do not track active app or window.

Use idle time to suppress or delay reminders.

Store idle segments in logs so dashboard analytics can be added later.

This gives you “hours worked” and “hours non-idle” without invasive window tracking.

Browser Surfaces
Three sections open in the browser via localhost pages:

Dashboard

Tabs

Handoffs

These are read/write UI shells backed by local files and the running app, not standalone apps. If the browser page is closed, the underlying feature remains active.

Dashboard
Dashboard is lowest priority for implementation, but the architecture should support it from day one. Its data source is the app’s event logs and persisted state snapshots.

Top KPI strip:

Total non-idle hours today

Total idle hours today

Pomodoro sessions today

Break compliance rate

Number of breaks taken

Number of breaks skipped

Current streak or consistency score

Average attention span

Main chart:

Teal-themed line chart for work frequency over the last 24 hours.

Toggle below chart for:

24 hours

7 days

30 days

Below that:

Consistency section

Average attention span

Categories based on attention span

Trend notes later, not needed now

This KPI-plus-chart layout is directionally aligned with productivity dashboards like Rize, which emphasize focus metrics, summary scores, and dashboard views across different time ranges.

Attention Span Definition
For now, define attention span as:

A contiguous non-idle work segment until either:

idle threshold exceeded, or

a break reminder is completed/skipped, or

a manual pause occurs.

Store these segments in logs so the dashboard can later compute:

Average attention span

Best focus block

Number of deep-work blocks

Consistency buckets

Suggested buckets:

Fragmented: under 10 min

Recovering: 10–25 min

Steady: 25–45 min

Deep: 45–90 min

Extended: 90+ min

Tabs
Tabs are intentionally simple and browser-driven.

Desired Flow
User copies a link.

User presses a hotkey.

App reads clipboard and saves the link as a parked tab.

User opens Tabs from tray.

Browser page shows tabs sorted by recency.

Each tab has:

title or raw URL

saved time

age

open button

delete button

“watch tomorrow” / extend life by +1 day button

Open behavior:

Clicking a tab opens it in Chrome.

“Open all” opens all visible tabs.

No need to target the exact last window.

Stale time defaults to 24 hours.

Stale tabs are visually muted, not auto-deleted.

Suggested Extras
Useful additions that still keep it simple:

Manual quick-add URL field on the page.

Notes field per tab, optional.

Filter chips:

fresh

stale

extended

“Clear opened tabs” bulk action.

“Copy all links” bulk action.

Handoffs
Handoffs are lightweight reminder objects, not a todo system.

Each handoff has:

Name

Deadline datetime

Optional reminder lead time

Context

Optional link

Status

Supported statuses:

Upcoming

Due soon

Due

Done

Archived

Behavior:

Handoffs are created either from tray action or browser page.

Reminder lead time can be explicit, e.g. remind 15 min before.

Due handoffs should trigger a local reminder.

Browser page should sort upcoming first.

Old completed items should archive automatically after a retention period.

Persistence
Use plain files plus an append-only event log. No SQLite.

Files
Recommended file layout:

text
data/
  app_state.json
  app_state.prev.json
  app_state.tmp.json

  config.json
  config.prev.json
  config.tmp.json

  logs/
    events-2026-05-03.log
    events-2026-05-04.log

  tabs.json
  tabs.prev.json
  tabs.tmp.json

  handoffs.json
  handoffs.prev.json
  handoffs.tmp.json

  backups/
    app_state-2026-05-03T12-00-00.json
    tabs-2026-05-03T12-00-00.json
    handoffs-2026-05-03T12-00-00.json
Persistence Strategy
Use a two-layer persistence strategy:

Layer 1: append-only event log
Every important event is appended immediately:

timer fired

break completed

break skipped

break snoozed

idle started

idle ended

pomodoro started/completed

tab added/opened/deleted/extended

handoff created/edited/completed/archived

app paused/resumed

app started/stopped/recovered

This is the first recovery source.

Layer 2: periodic state snapshots
At regular intervals and on significant changes, serialize current in-memory state to snapshot files:

app_state.json

tabs.json

handoffs.json

config.json

This is the fast restore source.

Safe Write Protocol
Every file write should follow this sequence:

Serialize content to *.tmp.json

Flush and fsync the temp file

Copy current stable file to *.prev.json

Atomically replace stable file with temp file

Verify readable JSON

Only then consider write successful

Writing to a temp file in the same directory and then replacing the destination with os.replace is the correct atomic replacement pattern on the same filesystem.

Recovery Order
On startup or crash recovery:

Try current stable snapshot file.

If corrupt, try prev snapshot.

If both fail, rebuild state from append-only event logs.

If rebuild partly fails, recover whatever valid events remain and mark degraded recovery in logs.

This gives you the “two-step fallback mechanism” you asked for, with an additional third emergency path.

Robustness
The app should be designed to stay up even when one surface fails.

Failure Rules
If dashboard server fails, tray and timers continue.

If browser page is malformed, underlying data is untouched.

If a reminder popup fails, timer event is still logged.

If snapshot writing fails, event log still contains the truth.

If a single worker thread crashes, supervisor should restart that worker.

If the whole app crashes, restart should recover from snapshots or logs without resetting timers to zero.

Supervisor Model
Within the single process, use a lightweight internal supervisor:

Starts all worker threads.

Monitors health heartbeats.

Restarts failed non-main threads.

Emits restart events into the log.

Each worker thread should periodically update:

last heartbeat

current status

last exception summary

If a thread misses heartbeat beyond threshold, restart it.

Logging
Use append-only newline-delimited JSON logs.

Example event:

json
{"ts":"2026-05-03T13:10:03Z","type":"break_fired","break_id":"lookaway","status":"shown"}
{"ts":"2026-05-03T13:10:25Z","type":"break_completed","break_id":"lookaway","duration_sec":20}
{"ts":"2026-05-03T13:11:00Z","type":"idle_started"}
{"ts":"2026-05-03T13:18:11Z","type":"idle_ended","idle_sec":431}
{"ts":"2026-05-03T13:30:02Z","type":"tab_added","url":"https://youtube.com/..."}
{"ts":"2026-05-03T13:31:02Z","type":"handoff_created","id":"h_102","deadline":"2026-05-03T16:00:00Z"}
Rules:

Never rewrite old log lines.

Rotate logs daily.

On startup, continue with today’s file or create a new one.

Keep logs as the audit trail and analytics source.

Config
Config should be human-readable and explicit.

Suggested top-level config:

json
{
  "theme": "teal",
  "tray_enabled": true,
  "dashboard_port": 41741,
  "idle_threshold_sec": 300,
  "stale_tab_hours": 24,
  "snapshot_interval_sec": 30,
  "backup_interval_min": 30,
  "timers": [
    {
      "id": "blink",
      "name": "Blink",
      "enabled": true,
      "interval_sec": 600,
      "duration_sec": 10,
      "priority": 3
    },
    {
      "id": "lookaway",
      "name": "Look Away",
      "enabled": true,
      "interval_sec": 1200,
      "duration_sec": 20,
      "priority": 2
    },
    {
      "id": "walk",
      "name": "Walk",
      "enabled": true,
      "interval_sec": 1800,
      "duration_sec": 60,
      "priority": 1
    }
  ]
}
Module Layout
Recommended modules:

text
peeky/
  main.py
  supervisor.py
  tray.py
  timers.py
  reminders.py
  idle.py
  hotkeys.py
  persistence.py
  recovery.py
  events.py
  state.py
  config.py
  web/
    server.py
    dashboard.py
    tabs.py
    handoffs.py
  ui/
    notifier.py
  data/
  assets/
Module Responsibilities
main.py
Bootstraps config, state, recovery, supervisor, tray, and workers.

supervisor.py
Starts threads, monitors health, restarts failed workers.

tray.py
Owns tray icon, right-click menu, and menu actions.

timers.py
Multi-timer scheduler, due evaluation, merge logic, snooze handling.

reminders.py
Popup/toast dispatch and action handling.

idle.py
Tracks idle vs non-idle transitions.

hotkeys.py
Handles tab-save hotkey and any future quick actions.

persistence.py
Safe writes, snapshots, backups, and flush cadence.

recovery.py
Restores state from snapshot, prev file, or logs.

events.py
Append-only event writer and event schemas.

state.py
Thread-safe in-memory state model.

web/server.py
Localhost web app host.

web/dashboard.py
Aggregates metrics from logs and snapshots.

web/tabs.py
CRUD for parked tabs.

web/handoffs.py
CRUD for handoffs and reminders.

State Model
The running app should keep a single in-memory state object protected by locks.

High-level state domains:

App lifecycle state

Timer states

Idle state

Today counters

Tabs collection

Handoffs collection

Pending reminders

Last persistence metadata

Worker heartbeats

Keep the browser UI stateless where possible. It should fetch current state from local endpoints and post changes back to the app.

API Shape
Expose a small localhost API for browser pages.

Suggested endpoints:

GET /api/health

GET /api/dashboard/summary

GET /api/dashboard/frequency?range=24h|7d|30d

GET /api/tabs

POST /api/tabs

POST /api/tabs/open

POST /api/tabs/open-all

POST /api/tabs/extend

DELETE /api/tabs/:id

GET /api/handoffs

POST /api/handoffs

PATCH /api/handoffs/:id

POST /api/handoffs/:id/done

Implementation Order
Keep the build order strict so the app is useful early.

Phase 1
Thread-safe state

Event log writer

Snapshot persistence

Recovery logic

Supervisor

Tray app shell

Phase 2
Multi-break timer engine

Idle logging

Reminder actions

Pause/resume from tray

Phase 3
Tabs backend

Tabs browser page

Hotkey to save clipboard URL

Open/open-all/extend/delete flows

Phase 4
Handoffs backend

Handoffs browser page

Deadline reminders

Phase 5
Dashboard aggregation

KPI page

Teal theme

24h / week / month chart toggle

Non-Negotiables
These must be treated as required:

Tray app is the primary control surface.

Logs are written immediately and are the ultimate source of truth.

Snapshots are only a convenience for fast recovery.

No SQLite.

No crash should reset timers, tabs, or handoffs.

Browser pages are optional views; core engine must survive without them.

Dashboard is last priority, but logging must support it from the beginning.

Build Intent
This app should feel like a polished background companion, not a toy timer. Productivity dashboards often stand out because they combine summary KPIs with coherent trend views rather than dumping raw numbers, which is why the dashboard should eventually center on a KPI strip plus a single strong trend chart instead of many disconnected widgets.

