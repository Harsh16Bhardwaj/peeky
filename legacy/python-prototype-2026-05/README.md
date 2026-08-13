# Legacy Python Prototype

This directory preserves the May 2026 Python prototype for reference only. Nothing here is used by the current application.

## What failed

- The implementation stopped at stub modules; the entrypoint loaded JSON and exited immediately.
- PyQt5 and pystray were selected before a coherent window and event-loop lifecycle was proven.
- Product scope expanded into dashboards, tabs, handoffs, web servers, and recovery before the basic tray timer worked.
- Governance files were duplicated, wrapped in pasted conversational text, and contained encoding corruption.
- Placeholder icon and sound assets were treated as completion instead of temporary scaffolding.
- No installer, executable smoke test, background longevity test, or real tray workflow was delivered.

The replacement starts with the actual product contract: a reliable Windows tray process, one scheduler, focused break UX, local diagnostics, and verified packaging.
