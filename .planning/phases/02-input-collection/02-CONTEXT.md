# Phase 2: Input Collection - Context

**Gathered:** 2026-01-31
**Status:** Ready for planning

<domain>
## Phase Boundary

Complete user data gathering wizard with 6 success criteria: hostname, username, personal info, timezone, keyboard layout, and path inputs. Each input has validation. User can navigate forward/backward through steps without losing data.

</domain>

<decisions>
## Implementation Decisions

### Validation feedback
- Summary at top of screen when user tries to proceed with errors
- Hybrid validation: real-time inline state (red/green) as they type, summary on submit attempt
- Detailed error messages with examples: "Hostname cannot contain spaces or special characters"
- Allow navigation with validation errors (not blocking)

### Selection UI
- List with search for timezone and keyboard layout selection
- Type-to-filter: type characters to filter the list, arrow keys to navigate
- Keyboard layout shows name only, no visual preview
- Timezones organized by region: Continent/City hierarchy (America/New_York, Europe/Berlin)

### Progress indication
- Left sidebar navigator showing all wizard steps
- Status icons per step: checkmark for completed, dot for current, empty for pending
- Review-only sidebar: can view past steps but not jump to future steps
- Not interactive for navigation, just informational

### Field navigation
- Both Tab and arrow keys move between form fields
- Auto-advance to next field when current field is valid
- Default field order: top to bottom within each step

### Claude's Discretion
- Active field indicator design (border highlight or cursor styling)
- Exact spacing, colors, and typography
- Keyboard shortcut details (Enter vs Ctrl+N for next, etc.)

</decisions>

<specifics>
## Specific Ideas

- Error summary appears only when user attempts to proceed with invalid data
- Type-to-filter starts immediately when user types any character in the list
- Sidebar shows: ✓ Hostname → ○ Username → ○ Full Name → ○ Git → ○ Timezone → ○ Keyboard → ○ Paths

</specifics>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope

</deferred>

---

*Phase: 02-input-collection*
*Context gathered: 2026-01-31*
