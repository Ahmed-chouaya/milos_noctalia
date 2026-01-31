# Phase 1: TUI Foundation - Research

**Researched:** 2026-01-31
**Domain:** Rust TUI with Ratatui 0.30.0 and Crossterm 0.29.0
**Confidence:** HIGH

## Summary

Phase 1 establishes the foundational TUI infrastructure for the MILOS installer using Ratatui 0.30.0, Crossterm 0.29.0, and Color-eyre 0.6. The research confirms these versions work well together and provides clear patterns for implementing the required functionality: event loop, state management, logo display, and error handling.

The key insight is that Ratatui 0.30.0 is modularized into multiple crates (core, widgets, crossterm backend) but the main `ratatui` crate re-exports everything for application developers. Color-eyre integration requires careful terminal restoration before displaying errors. The Elm Architecture pattern is the recommended approach for wizard-style state management.

**Primary recommendation:** Use centralized event handling with message passing to match on current step, apply Elm Architecture (Model-Update-View) for wizard state, and use the Canvas widget with custom paint closure for pixel art logo rendering.

## Standard Stack

The established libraries and versions for this phase:

### Core Dependencies
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| ratatui | 0.30.0 | TUI widget library | Modern, actively maintained, modular architecture |
| crossterm | 0.29.0 | Terminal backend | Cross-platform raw mode, event handling |
| color-eyre | 0.6.5 | Error handling | Beautiful backtraces, panic hooks |
| color-eyre | 0.6.5 | eyre trait for error wrapping | Modern error handling with context |

### Feature Flags
```toml
[dependencies]
ratatui = "0.30.0"
crossterm = "0.29.0"
color-eyre = "0.6"
```

### Cargo Configuration
No special features required for basic functionality. The `crossterm` backend is enabled by default in Ratatui.

**Installation:**
```bash
cargo add ratatui@=0.30.0 crossterm@=0.29.0 color-eyre@=0.6
```

## Architecture Patterns

### Recommended Project Structure

Based on the Ratatui component template, structure the TUI crate as:

```
milos-tui/
├── src/
│   ├── main.rs              # Entry point, terminal setup/teardown
│   ├── tui.rs               # Terminal initialization and restoration
│   ├── app.rs               # Main App struct and event loop
│   ├── state.rs             # Wizard state (Model in Elm Architecture)
│   ├── components/
│   │   ├── logo.rs          # Pixel art logo rendering
│   │   ├── sidebar.rs       # Step list navigation sidebar
│   │   ├── modal.rs         # Error modal overlay
│   │   └── welcome.rs       # Welcome screen component
│   └── errors.rs            # Error types and handling
```

### Pattern 1: Elm Architecture for Wizard State

The Elm Architecture (Model-Update-View) is ideal for wizard-style TUI applications:

```rust
// Source: https://ratatui.rs/concepts/application-patterns/the-elm-architecture/

// 1. Model - All application state
#[derive(Debug, Default)]
struct WizardState {
    current_step: usize,
    steps: Vec<WizardStep>,
    is_first_launch: bool,
    error_mode: bool,
    error_message: Option<String>,
    show_backtrace: bool,
}

// 2. Message enum - All possible events
#[derive(PartialEq, Clone)]
enum Message {
    NextStep,
    PreviousStep,
    EnterWelcome,      // Initial "Press Enter to begin"
    ShowError(String), // System error occurred
    DismissError,
    ToggleBacktrace,
    Quit,
}

// 3. Update function - State transitions
fn update(state: &mut WizardState, msg: Message) {
    match msg {
        Message::NextStep if can_advance(state) => {
            state.current_step = (state.current_step + 1).min(state.steps.len() - 1);
        }
        Message::PreviousStep if state.current_step > 0 => {
            state.current_step -= 1;
        }
        Message::EnterWelcome => {
            state.is_first_launch = false;
        }
        Message::ShowError(msg) => {
            state.error_mode = true;
            state.error_message = Some(msg);
        }
        Message::DismissError => {
            state.error_mode = false;
            state.error_message = None;
        }
        Message::ToggleBacktrace => {
            state.show_backtrace = !state.show_backtrace;
        }
        _ => {}
    }
}

// 4. View function - Render UI
fn view(state: &WizardState, frame: &mut Frame) {
    // Layout: sidebar + main content
    let layout = Layout::horizontal([
        Constraint::Length(25),  // Sidebar width
        Constraint::Fill(1),     // Main content
    ]);
    let [sidebar_area, main_area] = layout.areas(frame.area());
    
    // Render sidebar with step progress
    render_sidebar(state, frame, sidebar_area);
    
    // Render main content based on state
    if state.is_first_launch {
        render_welcome(state, frame, main_area);
    } else if state.error_mode {
        render_error_modal(state, frame, main_area);
    } else {
        render_step(state, frame, main_area);
    }
}
```

**Source:** [Ratatui Elm Architecture](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/)

### Pattern 2: Centralized Event Handling with Message Passing

For wizard flows, use centralized event catching with message passing:

```rust
// Source: https://ratatui.rs/concepts/event-handling/

fn event_loop(terminal: &mut DefaultTerminal, state: &mut WizardState) -> color_eyre::Result<()> {
    let tick_rate = Duration::from_millis(100);
    
    loop {
        terminal.draw(|frame| view(state, frame))?;
        
        // Poll for events with timeout
        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key(state, key);
                }
            }
        }
    }
}

fn handle_key(state: &mut WizardState, key: KeyEvent) {
    let msg = match key.code {
        KeyCode::Enter => {
            if state.is_first_launch {
                Message::EnterWelcome
            } else {
                Message::NextStep
            }
        }
        KeyCode::Esc => Message::PreviousStep,
        KeyCode::Char('b') => Message::ToggleBacktrace,
        KeyCode::Char('q') | KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Message::Quit,
        _ => return,
    };
    update(state, msg);
}
```

**Source:** [Ratatui Event Handling](https://ratatui.rs/concepts/event-handling/)

### Pattern 3: Pixel Art Logo with Canvas Widget

The Canvas widget provides a drawing surface for pixel art:

```rust
// Source: https://ratatui.rs/examples/widgets/canvas/

fn render_logo(frame: &mut Frame, area: Rect, animation_state: &LogoAnimation) {
    Canvas::default()
        .block(Block::bordered())
        .x_bounds([0.0, f64::from(area.width)])
        .y_bounds([0.0, f64::from(area.height)])
        .paint(move |ctx| {
            // Draw each character as positioned text with color
            for (i, line) in animation_state.visible_lines.iter().enumerate() {
                // Neon glow effect using layered text
                ctx.print(
                    f64::from(area.left() + 2),
                    f64::from(area.top() + i as u16 + 2),
                    line.clone().with(Color::DarkGray),  // Glow layer
                );
                ctx.print(
                    f64::from(area.left() + 1),
                    f64::from(area.top() + i as u16 + 1),
                    line.clone().with(Color::Yellow),   // Amber glow
                );
                ctx.print(
                    f64::from(area.left()),
                    f64::from(area.top() + i as u16),
                    line.clone().with(Color::Green),    // Main text
                );
            }
            
            // Cursor blink effect
            if animation_state.show_cursor {
                let cursor_x = animation_state.cursor_position;
                let cursor_y = animation_state.cursor_line;
                ctx.print(
                    f64::from(cursor_x),
                    f64::from(cursor_y),
                    "█".green(),
                );
            }
        })
        .render(area, frame.buffer_mut());
}
```

**Source:** [Ratatui Canvas Widget](https://ratatui.rs/examples/widgets/canvas/)

### Pattern 4: Color-eyre with Terminal Restoration

Critical for displaying errors correctly:

```rust
// Source: https://ratatui.rs/recipes/apps/color-eyre/

fn main() -> color_eyre::Result<()> {
    // Install color-eyre hooks FIRST
    color_eyre::install()?;
    
    // Initialize terminal
    let mut terminal = tui::init()?;
    
    // Run the app
    let result = run_app(&mut terminal).wrap_err("app failed");
    
    // ALWAYS restore terminal before showing errors
    tui::restore()?;
    
    result
}

fn set_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = tui::restore();  // Ignore errors - we're already failing
        original_hook(panic_info);
    }));
}
```

**Source:** [Color-eyre Recipe](https://ratatui.rs/recipes/apps/color-eyre/)

### Pattern 5: Layout for Sidebar + Main Content

Use Constraint-based layout for wizard interface:

```rust
// Source: https://ratatui.rs/concepts/layout/

fn render_wizard_layout(state: &WizardState, frame: &mut Frame) {
    let area = frame.area();
    
    // Vertical split: sidebar (left) + main content (right)
    let main_layout = Layout::horizontal([
        Constraint::Length(30),  // Fixed-width sidebar
        Constraint::Fill(1),     // Remaining space for content
    ]);
    let [sidebar_area, content_area] = main_layout.areas(area);
    
    // Horizontal split in sidebar: logo + step list
    let sidebar_layout = Layout::vertical([
        Constraint::Length(15),  // Logo area
        Constraint::Fill(1),     // Step list fills rest
    ]);
    let [logo_area, steps_area] = sidebar_layout.areas(sidebar_area);
    
    // Render components
    render_logo(state, frame, logo_area);
    render_step_list(state, frame, steps_area);
    render_content(state, frame, content_area);
}
```

**Source:** [Ratatui Layout](https://ratatui.rs/concepts/layout/)

### Pattern 6: Modal Overlay for Errors

Use overwrite-region rendering for modals:

```rust
// Source: https://ratatui.rs/recipes/render/overwrite-regions/

fn render_error_modal(state: &WizardState, frame: &mut Frame, area: Rect) {
    // Create modal area (centered, 60% width, 40% height)
    let modal_layout = Layout::vertical([
        Constraint::Length(3),  // Header
        Constraint::Length(5),  // Error message
        Constraint::Length(3),  // Buttons
    ]);
    let modal_areas = modal_layout.areas(centered_rect(60, 40, area));
    
    // Darken background behind modal
    frame.render_widget(Clear, area);
    frame.render_widget(Block::bordered().style(Style::default().bg(Color::DarkGray)), area);
    
    // Error title
    frame.render_widget(
        Paragraph::new("ERROR").alignment(Alignment::Center),
        modal_areas[0],
    );
    
    // Error message
    frame.render_widget(
        Paragraph::new(state.error_message.clone().unwrap_or_default())
            .alignment(Alignment::Center),
        modal_areas[1],
    );
    
    // Buttons
    let button_layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Fill(1),
    ]);
    let [_left, retry_btn, cancel_btn, details_btn, _right] = button_layout.areas(modal_areas[2]);
    
    frame.render_widget(
        Block::bordered().title("Retry"),
        retry_btn,
    );
    frame.render_widget(
        Block::bordered().title("Details"),
        details_btn,
    );
}
```

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Terminal setup/teardown | Custom raw mode handling | Ratatui + Crossterm backend | Cross-platform issues, signal handling, panic restoration |
| Error backtraces | Custom formatting | color-eyre | Thread-safe, context-rich, industry standard |
| Event loop timing | Thread::sleep in loop | event::poll() with Duration | Non-blocking, efficient, proper terminal events |
| Layout calculations | Manual Rect math | Layout with Constraints | Responsive, maintainable, handles terminal resize |
| Cross-platform terminals | platform::all() checks | crossterm | Abstracts Windows/Unix differences |
| Color handling | ANSI escape strings | ratatui::style::Color | Type-safe, 256-color, RGB support |

**Key insight:** Terminal handling has subtle cross-platform differences (Windows conpty vs Unix PTY, signal handling, raw mode edge cases). Using the established stack avoids bugs that only appear in specific environments.

## Common Pitfalls

### Pitfall 1: Forgetting Terminal Restoration on Error

**What goes wrong:** Errors display with corrupted terminal state, unreadable output, cursor hidden or wrong position.

**Why it happens:** When an error occurs mid-TUI, the terminal is in raw mode with alternate screen active. Displaying formatted output without restoring state causes visual corruption.

**How to avoid:** Always call terminal restoration in error paths:

```rust
fn risky_operation() -> color_eyre::Result<()> {
    // ... code that might fail ...
    Ok(())
}

fn handle_errors() {
    if let Err(e) = risky_operation() {
        let _ = tui::restore();  // CRITICAL: Restore FIRST
        return Err(e);
    }
}
```

**Warning signs:** ANSI escape codes visible in error output, wrong colors, cursor at wrong position.

**Source:** [Color-eyre Recipe](https://ratatui.rs/recipes/apps/color-eyre/)

### Pitfall 2: Blocking Event Loop

**What goes wrong:** Application freezes, doesn't respond to input, animation jitters.

**Why it happens:** Using blocking read operations or sleep in the event loop prevents timely event processing and rendering.

**How to avoid:** Use non-blocking poll with timeout:

```rust
// BAD: Blocks until key press
let event = event::read()?;  // Blocks forever

// GOOD: Non-blocking poll with timeout
if event::poll(Duration::from_millis(50))? {
    if let Event::Key(key) = event::read()? {
        // Handle key
    }
}
// Continue to render/update even if no event
```

**Warning signs:** CPU at 100%, terminal unresponsive, animations frozen.

**Source:** [Crossterm poll](https://docs.rs/crossterm/0.29.0/crossterm/event/fn.poll.html)

### Pitfall 3: State Cloning for Animation

**What goes wrong:** Performance issues, complex borrow checker errors, animation stuttering.

**Why it happens:** Passing large state objects to animation closures causes ownership issues and unnecessary clones.

**How to avoid:** Use Arc/Rc for shared state or design stateless render functions:

```rust
// For animation state that needs to be shared:
struct LogoAnimation {
    visible_lines: Vec<String>,
    cursor_position: u16,
    cursor_line: u16,
    show_cursor: bool,
}

impl LogoAnimation {
    fn advance(&mut self) {
        // Update internal state
        self.cursor_position += 1;
    }
}

// In render function, take &mut self
fn render_logo(&self, frame: &mut Frame, area: Rect) {
    // Read-only access to current state
    // Animation updates happen separately
}
```

### Pitfall 4: Hardcoded Terminal Size

**What goes wrong:** UI breaks on smaller/larger terminals, content overflows, layout overlaps.

**Why it happens:** Calculating positions from absolute values instead of using Layout constraints.

**How to avoid:** Always use Layout with Constraints:

```rust
// BAD: Hardcoded values
let sidebar_width = 30;
let main_area = Rect::new(30, 0, area.width - 30, area.height);

// GOOD: Constraint-based layout
let layout = Layout::horizontal([
    Constraint::Length(30),
    Constraint::Fill(1),
]);
let [sidebar, main] = layout.areas(area);
```

**Source:** [Ratatui Layout](https://ratatui.rs/concepts/layout/)

### Pitfall 5: Missing KeyEventKind Check

**What goes wrong:** Duplicate events (press + repeat), unexpected behavior on key release.

**Why it happens:** Crossterm distinguishes between KeyPress, KeyRepeat, and KeyRelease events.

**How to avoid:** Check KeyEventKind:

```rust
if key.kind == KeyEventKind::Press {
    // Handle the key press
}
```

**Source:** [Crossterm Event Types](https://docs.rs/crossterm/0.29.0/crossterm/event/enum.KeyEventKind.html)

### Pitfall 6: Color-eyre Installed After Errors

**What goes wrong:** Error reports don't use color-eyre formatting, default Rust errors shown.

**Why it happens:** color-eyre::install() must be called before any errors are constructed.

**How to avoid:** Install as first statement in main:

```rust
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;  // FIRST: Before any error-prone code
    // ... rest of code
}
```

**Source:** [Color-eyre install docs](https://docs.rs/color-eyre/0.6.5/color_eyre/fn.install.html)

## Code Examples

### Event Loop Skeleton

```rust
// Source: Ratatui counter app tutorial
use std::time::Duration;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, DefaultTerminal};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init()?;
    let mut app = WizardState::default();
    
    while !app.should_exit {
        terminal.draw(|frame| view(&app, frame))?;
        
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_input(&mut app, key);
                }
            }
        }
    }
    
    ratatui::restore()?;
    Ok(())
}
```

### Wizard State Definition

```rust
#[derive(Debug, Default)]
struct WizardState {
    current_step: usize,
    steps: &'static [WizardStep],
    is_first_launch: bool,
    error_mode: bool,
    current_error: Option<String>,
    show_backtrace: bool,
}

#[derive(Debug)]
struct WizardStep {
    name: &'static str,
    key: char,  // For sidebar display
}

impl WizardState {
    fn can_advance(&self) -> bool {
        self.current_step < self.steps.len() - 1
    }
    
    fn can_go_back(&self) -> bool {
        self.current_step > 0 && !self.is_first_launch
    }
}
```

### Terminal Setup Module

```rust
// src/tui.rs
use std::io::{self, stdout, Stdout};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    set_panic_hook();
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore();
        hook(panic_info);
    }));
}
```

### Logo Animation State

```rust
struct LogoAnimation {
    lines: Vec<String>,
    visible_count: usize,
    cursor_x: u16,
    cursor_y: u16,
    show_cursor: bool,
    last_cursor_toggle: Instant,
}

impl LogoAnimation {
    fn new(logo_text: &'static str) -> Self {
        let lines: Vec<String> = logo_text.lines().map(|s| s.to_string()).collect();
        Self {
            lines,
            visible_count: 0,
            cursor_x: 0,
            cursor_y: 0,
            show_cursor: true,
            last_cursor_toggle: Instant::now(),
        }
    }
    
    fn tick(&mut self, now: Instant) {
        // Typewriter effect: show next character/line
        if self.visible_count < self.lines.len() * 20 {  // Approximate char count
            self.visible_count += 1;
        }
        
        // Cursor blink (500ms period)
        if now.duration_since(self.last_cursor_toggle) > Duration::from_millis(500) {
            self.show_cursor = !self.show_cursor;
            self.last_cursor_toggle = now;
        }
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| tui-rs (pre-0.22) | Ratatui 0.30.0 | 2023 rename | Modular architecture, better async support |
| Manual terminal restore | Color-eyre with panic hooks | 2024+ | Consistent error display, thread-safe |
| Blocking read | event::poll() pattern | Always best practice | Non-blocking, responsive UI |
| Hardcoded layout | Constraint-based Layout | Ratatui 0.26+ | Responsive to terminal resize |
| Single crate | Workspace with ratatui-core | 0.30.0 | Smaller binaries, better API stability |

**Deprecated/outdated:**
- `tui-rs` crate name (renamed to Ratatui)
- Blocking `event::read()` without poll check
- Hardcoded Rect calculations instead of Layout
- Any non-async event handling patterns (async not required for basic TUI)

## Open Questions

1. **Animation timing precision**
   - What we know: event::poll with 100ms timeout works for UI responsiveness
   - What's unclear: Best approach for smooth 60fps animations in terminal
   - Recommendation: Use 16ms tick rate, accept that terminal refresh rate may vary

2. **Color-eyre backtrace toggle in TUI**
   - What we know: color-eyre supports full backtraces with RUST_BACKTRACE env var
   - What's unclear: How to capture backtrace programmatically for toggle display
   - Recommendation: Use std::backtrace::Backtrace, show condensed error by default

3. **Terminal capability detection**
   - What we know: 256-color and truecolor support varies by terminal
   - What's unclear: Auto-detection of color depth
   - Recommendation: Assume 256-color support (most terminals), document assumption

## Sources

### Primary (HIGH confidence)
- [Ratatui 0.30.0 Documentation](https://docs.rs/ratatui/0.30.0/ratatui/) - Official crate docs
- [Ratatui Event Handling](https://ratatui.rs/concepts/event-handling/) - Official pattern guide
- [Ratatui Elm Architecture](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/) - Official architecture guide
- [Ratatui Layout](https://ratatui.rs/concepts/layout/) - Official layout docs
- [Crossterm 0.29.0 Documentation](https://docs.rs/crossterm/0.29.0/crossterm/) - Official backend docs
- [Color-eyre 0.6.5 Documentation](https://docs.rs/color-eyre/0.6.5/color_eyre/) - Official error handling docs
- [Ratatui Canvas Example](https://ratatui.rs/examples/widgets/canvas/) - Official canvas patterns
- [Color-eyre Recipe](https://ratatui.rs/recipes/apps/color-eyre/) - Official integration guide

### Secondary (MEDIUM confidence)
- [Ratatui Component Template](https://ratatui.rs/templates/component/) - Official project structure
- [Ratatui Project Structure](https://ratatui.rs/templates/component/project-structure/) - Official file organization

### Tertiary (LOW confidence)
- [Canvas Widget Reddit Discussion](https://www.reddit.com/r/rust/comments/1d8k2uj/) - Community confirmation of Canvas usage for pixel art
- [Wizard Pattern in Other Frameworks](https://docs.retool.com/apps/web/guides/components/wizard) - Conceptual guidance for wizard UX

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Official docs, stable versions, well-documented
- Architecture: HIGH - Elm Architecture is official recommendation, well-proven
- Event handling: HIGH - Crossterm docs are comprehensive, patterns are standard
- Pixel art/Canvas: MEDIUM - Canvas works for this, but not common use case
- Color-eyre integration: HIGH - Official recipe exists with full example

**Research date:** 2026-01-31
**Valid until:** 2026-07-31 (6 months - Ratatui releases monthly but stable API)
