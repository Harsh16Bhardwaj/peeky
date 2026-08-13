# AGENT.md — Model Operating Instructions for Peeky

This file governs how the model should behave, reason, and communicate
throughout the development of Peeky. Read this file first on every session
before touching any code or any other file.

---

## Project File Map

| File | Purpose | When to Read |
|---|---|---|
| `AGENT.md` | This file. Model behavior rules and file navigation guide | Every session, first |
| `architecture.md` | Full system design, module layout, data flow, threading model | Before writing any new module or making structural decisions |
| `instructions.md` | Coding standards, style rules, language conventions, tooling | Before writing any code |
| `focus.md` | The active isolated feature being built right now | Before starting any implementation work |
| `updates.md` | Approved deviations, plan changes, and amendments to architecture | After reading architecture.md, to check for overrides |
| `deviations.md` | Running log of what was changed from the original plan and why | Append to this whenever something deviates from architecture.md |

---

## Session Start Protocol

Every session must follow this sequence before writing a single line of code:

1. Read `AGENT.md` (this file)
2. Read `focus.md` to understand what is in scope today
3. Read `architecture.md` for structural context
4. Read `updates.md` for any amendments that override architecture
5. Read `instructions.md` for coding standards
6. Check `deviations.md` to understand what has already diverged

Do not skip steps. Do not assume prior context carries over between sessions.
Each session starts cold. The files are the memory.

---

## focus.md — How to Use It

`focus.md` defines the **one feature or module currently being built**.
It is a hard scope boundary.

Rules:
- Only work on what is described in `focus.md`
- Do not refactor, improve, or touch code outside the focused scope
  unless it is a direct blocker to the focused feature
- If work outside scope is genuinely necessary, note it in `deviations.md`
  and ask for confirmation before proceeding
- If `focus.md` is empty or absent, stop and ask what to focus on
  before writing any code

Example `focus.md` content:
