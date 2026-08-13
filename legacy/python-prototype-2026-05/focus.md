Here's the complete `focus.md` for Node 1. Paste this as-is into your `focus.md` file.

***

```markdown
# focus.md — Node 1: Project Skeleton + Dev Environment

## Pre-Work: Read These Files First

Before writing a single file or folder, read the following in this exact order:

1. `AGENT.md` — model behavior rules, when to log deviations, session protocol
2. `architecture.md` — full system design, module layout, threading model, data flow
3. `instructions.md` — coding standards, style, naming conventions, tooling rules
4. `updates.md` — any amendments that override architecture (if file exists)
5. `deviations.md` — what has already diverged from the plan (if file exists)

Do not proceed past this point until all five files above have been read.
If any of these files are missing, stop and report which ones are absent
before doing anything else.

---

## What This Node Is

Node 1 is purely structural. No business logic. No timer code. No UI.
The job is to create the complete project folder layout, all governance files
with their initial content, the dependency manifest, the config schema,
and a minimal `main.py` that boots cleanly and exits cleanly.

At the end of this node the repo must be runnable. `python main.py` must
start without errors and exit cleanly. That is the only acceptance test.

---

## Strict Scope

### In scope
- Create every folder and file listed in this document
- Write initial content for all governance files
- Write `requirements.txt` with pinned versions
- Write `config.json` with the full schema and default values
- Write `main.py` that boots, prints a startup log line, and exits
- Write a `.gitignore` appropriate for a Python + PyQt5 + PyInstaller project

### Out of scope
- Any timer logic
- Any tray icon or pystray code
- Any UI windows
- Any threading beyond what is needed for a clean boot
- Any web server
- Any persistence or recovery logic
- Any hotkey registration
- Any platform API calls

If you find yourself writing anything from the out-of-scope list, stop,
log it in `deviations.md`, and return to scope.

---

## Folder Structure to Create

Create exactly this structure. Do not add extra folders or files unless
they are listed here. Do not rename anything.

```

peeky/
├── main.py
├── supervisor.py              # empty stub
├── tray.py                    # empty stub
├── timers.py                  # empty stub
├── reminders.py               # empty stub
├── idle.py                    # empty stub
├── hotkeys.py                 # empty stub
├── persistence.py             # empty stub
├── recovery.py                # empty stub
├── events.py                  # empty stub
├── state.py                   # empty stub
├── config.py                  # empty stub
│
├── ui/
│   ├── __init__.py
│   ├── notifier.py            # empty stub
│   ├── overlay.py             # empty stub
│   └── settings.py            # empty stub
│
├── web/
│   ├── __init__.py
│   ├── server.py              # empty stub
│   ├── dashboard.py           # empty stub
│   ├── tabs.py                # empty stub
│   ├── handoffs.py            # empty stub
│   └── static/
│       ├── dashboard.html     # empty stub
│       ├── tabs.html          # empty stub
│       └── handoffs.html      # empty stub
│
├── data/
│   ├── config.json            # full schema with defaults (see below)
│   ├── app_state.json         # empty object {}
│   ├── tabs.json              # empty array []
│   ├── handoffs.json          # empty array []
│   └── logs/                  # empty directory, add .gitkeep
│
├── assets/
│   ├── peeky.ico              # placeholder, see note below
│   ├── exercises.json         # eye exercise prompts list (see below)
│   └── sounds/
│       └── chime.wav          # placeholder, see note below
│
├── installer/
│   └── peeky.nsi              # empty stub
│
├── tests/
│   └── .gitkeep
│
├── AGENT.md                   # governance file (already exists)
├── architecture.md            # governance file (already exists)
├── instructions.md            # governance file (already exists)
├── focus.md                   # this file
├── updates.md                 # initialized with empty template
├── deviations.md              # initialized with empty template
├── CHANGELOG.md               # initialized with v0.0.1 entry
├── README.md                  # minimal project description
├── requirements.txt           # full dependency list with pinned versions
├── build.py                   # empty stub
└── .gitignore

```

---

## File Content Specifications

### main.py

Write a minimal entry point that does the following in order:

1. Sets up a basic stdout logger using Python's `logging` module at INFO level.
   Format: `%(asctime)s [%(levelname)s] %(name)s — %(message)s`
2. Logs: `"Peeky starting — version {VERSION}"`
3. Loads `data/config.json` and parses it into a Python dict.
   If the file is missing or malformed, log an error and exit with code 1.
4. Logs: `"Config loaded successfully"`
5. Logs: `"Peeky boot complete — exiting (stub mode)"`
6. Exits cleanly with code 0.

No threading. No tray. No UI. Just boot, load config, log, exit.

Define `VERSION = "0.1.0"` as a module-level constant at the top of the file.

---

### requirements.txt

Pin all versions. Use these exact packages:

```

PyQt5==5.15.11
PyQt5-sip==12.15.0
pystray==0.19.5
Pillow==10.4.0
keyboard==0.13.5
pygetwindow==0.0.9
flask==3.1.0
pywin32==308
pyinstaller==6.11.0
plyer==2.1.0
requests==2.32.3
python-dateutil==2.9.0

```

Add a comment block at the top of the file:

```

# Peeky — Python 3.11 required
# Install: pip install -r requirements.txt
# Do not upgrade versions without testing — pin changes go in updates.md

```

---

### data/config.json

Write the full configuration schema with all default values. Use exactly
this structure and these keys. Do not abbreviate or omit any field.

```json
{
  "version": "0.1.0",
  "theme": "teal",
  "tray_enabled": true,
  "dashboard_port": 41741,
  "idle_threshold_sec": 300,
  "stale_tab_hours": 24,
  "snapshot_interval_sec": 30,
  "backup_interval_min": 30,
  "log_retention_days": 30,
  "active_profile": "work",
  "profiles": {
    "work": {
      "label": "Work",
      "strict_mode": false,
      "pomodoro_enabled": true,
      "pomodoro_work_min": 25,
      "pomodoro_break_min": 5,
      "pomodoro_long_break_min": 15,
      "pomodoro_cycles_before_long": 4
    },
    "study": {
      "label": "Study",
      "strict_mode": false,
      "pomodoro_enabled": true,
      "pomodoro_work_min": 50,
      "pomodoro_break_min": 10,
      "pomodoro_long_break_min": 20,
      "pomodoro_cycles_before_long": 2
    },
    "casual": {
      "label": "Casual",
      "strict_mode": false,
      "pomodoro_enabled": false,
      "pomodoro_work_min": 60,
      "pomodoro_break_min": 10,
      "pomodoro_long_break_min": 20,
      "pomodoro_cycles_before_long": 4
    }
  },
  "timers": [
    {
      "id": "blink",
      "name": "Blink Reminder",
      "enabled": true,
      "interval_sec": 600,
      "duration_sec": 10,
      "priority": 3,
      "message": "Blink your eyes rapidly 10 times."
    },
    {
      "id": "lookaway",
      "name": "Look Away",
      "enabled": true,
      "interval_sec": 1200,
      "duration_sec": 20,
      "priority": 2,
      "message": "Look at something 20 feet away for 20 seconds."
    },
    {
      "id": "walk",
      "name": "Walk Break",
      "enabled": true,
      "interval_sec": 1800,
      "duration_sec": 60,
      "priority": 1,
      "message": "Stand up and take a short walk."
    }
  ],
  "hotkeys": {
    "save_tab": "ctrl+shift+s",
    "open_tabs": "ctrl+shift+t",
    "quick_capture": "ctrl+shift+space"
  },
  "notifications": {
    "sound_enabled": true,
    "sound_file": "assets/sounds/chime.wav",
    "pre_reminder_sec": 10
  },
  "autostart": true,
  "auto_update_check": true,
  "github_releases_url": "https://api.github.com/repos/yourusername/peeky/releases/latest"
}
```

---

### assets/exercises.json

Write a JSON array of eye exercise prompt objects.
Each object has `id`, `prompt`, and `duration_hint`.

Include at minimum these entries:

```json
[
  {
    "id": "blink_rapid",
    "prompt": "Blink your eyes rapidly 10 times.",
    "duration_hint": "10 seconds"
  },
  {
    "id": "look_far",
    "prompt": "Find something at least 20 feet away and focus on it.",
    "duration_hint": "20 seconds"
  },
  {
    "id": "eye_roll",
    "prompt": "Slowly roll your eyes in a full circle, clockwise then counter-clockwise.",
    "duration_hint": "15 seconds"
  },
  {
    "id": "near_far",
    "prompt": "Hold your thumb close to your face, focus on it, then shift focus to a far object. Repeat 5 times.",
    "duration_hint": "20 seconds"
  },
  {
    "id": "palm",
    "prompt": "Close your eyes and gently cup your palms over them. Breathe slowly.",
    "duration_hint": "20 seconds"
  },
  {
    "id": "figure_eight",
    "prompt": "Trace a slow figure-eight with your eyes without moving your head.",
    "duration_hint": "15 seconds"
  },
  {
    "id": "diagonal",
    "prompt": "Look up-left, then down-right. Then up-right, then down-left. Repeat 5 times.",
    "duration_hint": "15 seconds"
  }
]
```

---

### updates.md

Initialize with this exact content:

```markdown
# updates.md — Plan Amendments and Overrides

This file contains approved changes to architecture.md.
Entries here override architecture.md when they conflict.
Read this file after architecture.md every session.

***

_No updates yet. First entry will appear here when a plan change is approved._
```

---

### deviations.md

Initialize with this exact content:

```markdown
# deviations.md — Running Deviation Log

Append-only. Never edit or delete past entries.
Log every deviation from architecture.md or focus.md immediately when it occurs.

Format:
### [Date] [Time] — [Short title]
**File affected:**
**Original plan:**
**What was done instead:**
**Reason:**
**Follow-up required:**

***

_No deviations yet._
```

---

### CHANGELOG.md

Initialize with this exact content:

```markdown
# CHANGELOG

## [0.1.0] — 2026-05-03
### Added
- Initial project skeleton
- Folder structure and all stub modules
- config.json schema with defaults
- Governance files: AGENT.md, architecture.md, focus.md, updates.md, deviations.md
- requirements.txt with pinned dependencies
- Minimal main.py boot sequence
```

---

### README.md

Write a minimal README with these sections only:

- Project name and one-line description
- Requirements (Python 3.11, Windows 10/11)
- Setup instructions (clone, create venv, pip install, run)
- Current status: "Node 1 — Skeleton complete"
- Link placeholder for future docs

---

### Empty stubs

Every `.py` stub file must contain exactly:

```python
# {filename} — stub
# Implemented in Node {N} — see focus.md and architecture.md
```

Replace `{filename}` with the actual filename.
Replace `{N}` with the node number where this file will be implemented,
cross-referenced against the roadmap in architecture.md.

Every `.html` stub file must contain:

```html
<!-- {filename} — stub -->
<!-- Implemented in Node {N} — see focus.md and architecture.md -->
```

---

### Placeholder assets

- `assets/peeky.ico` — if you cannot generate a real icon, create a 32x32
  single-color PNG named `peeky.ico` using Pillow. Log this in deviations.md
  as a placeholder pending a real icon.
- `assets/sounds/chime.wav` — create a minimal valid WAV file (0.5 second
  sine tone at 440Hz) programmatically using Python's `wave` + `struct`
  modules. This avoids a missing file error when sound code is wired up.
  Log this in deviations.md as a generated placeholder.

---

### .gitignore

Include at minimum:

```
__pycache__/
*.pyc
*.pyo
.venv/
venv/
*.egg-info/
dist/
build/
*.spec
data/logs/
data/backups/
data/*.tmp.json
data/*.prev.json
*.exe
installer/*.exe
.env
```

---

## Acceptance Criteria

Node 1 is complete when all of the following are true:

- [ ] Every folder and file listed in the structure above exists
- [ ] `python main.py` runs without any import errors or exceptions
- [ ] `python main.py` prints startup and shutdown log lines to stdout
- [ ] `data/config.json` is valid JSON and loads without errors in `main.py`
- [ ] All stub `.py` files have their stub comment and do not throw on import
- [ ] `requirements.txt` installs cleanly with `pip install -r requirements.txt`
  on a fresh venv using Python 3.11
- [ ] `assets/exercises.json` is valid JSON with at least 7 entries
- [ ] `assets/peeky.ico` exists and is a valid image file
- [ ] `assets/sounds/chime.wav` exists and is a valid WAV file
- [ ] All governance files exist with their initialized content
- [ ] `deviations.md` has an entry for every placeholder asset created
- [ ] `.gitignore` is present and correct

Do not mark Node 1 complete until every checkbox above passes.

---

## What Comes Next

Node 2 will implement `events.py` — the append-only event log writer.
Do not look ahead or pre-implement anything from Node 2 in this session.
When Node 1 is complete, update `focus.md` to Node 2 content.
```