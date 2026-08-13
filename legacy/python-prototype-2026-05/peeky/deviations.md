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

### [2026-05-05] [11:38] — Placeholder icon instead of real .ico
**File affected:** assets/peeky.ico
**Original plan:** Real application icon
**What was done instead:** 32x32 single-color teal PNG saved as .ico via Pillow
**Reason:** No real icon asset available yet
**Follow-up required:** Replace with a proper designed icon

### [2026-05-05] [11:38] — Generated placeholder chime WAV
**File affected:** assets/sounds/chime.wav
**Original plan:** Real chime sound file
**What was done instead:** 0.5 second sine tone at 440Hz generated via wave + struct
**Reason:** No real sound asset available yet; needed a valid WAV so future sound code won't error
**Follow-up required:** Replace with a pleasant chime sound
