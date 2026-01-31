# Phase 1: TUI Foundation - Context

**Gathered:** 2026-01-31
**Status:** Ready for planning

<domain>
## Phase Boundary

Working installer shell with navigation and error handling. Project structure, event loop, state management, logo display, and error handling are implemented. Input collection (Phase 2) is separate.

</domain>

<decisions>
## Implementation Decisions

### Logo Display
- Pixel art defined as Rust code (code-based, not external files)
- Generator script included for creating/editing logo from simple source format
- Neon glow style with green letters + amber glow effect
- Typewriter animation: letters appear one-by-one with cursor blink effect

### Navigation Pattern
- Linear only navigation: must complete each step in order, no skipping ahead
- Back button behavior: persistent until exit (data saved, can navigate freely)
- First launch: Welcome screen with logo + "Press Enter to begin" prompt
- Progress indication: Step list sidebar showing all steps with checkmarks for completed

### Error Handling UX
- Input validation errors: inline message below field, auto-focuses field, shows suggestion
- System errors (disk, network): modal overlay blocking navigation until dismissed
- Modal actions: context-dependent (Retry, Cancel, Exit — varies by error type)
- Error message style: user-friendly text by default, toggle to show color-eyre backtrace

### State Architecture
- Persistence: in-memory only (simplest code, no file I/O)
- Data scope: current step index + all step inputs (hostname, username, timezone, etc.)
- Update pattern: eager save (state updates on every keystroke)
- Structure: flat struct with all fields (no enums or nested structures)

### Claude's Discretion
- Exact pixel art design for the MILOS logo
- Generator script format and implementation
- Step list sidebar visual design (checkmarks, colors, positioning)
- Error modal button ordering and labels
- Color-eyre backtrace formatting in the details toggle

</decisions>

<specifics>
## Specific Ideas

- "Neon glow" aesthetic for MILOS logo
- "Typewriter" effect with cursor blink for logo animation
- Step list sidebar showing all wizard steps with visual completion state
- color-eyre backtraces available via toggle (not hidden entirely)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within Phase 1 scope

</deferred>

---

*Phase: 01-tui-foundation*
*Context gathered: 2026-01-31*
