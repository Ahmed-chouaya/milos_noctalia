use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use crate::state::{WizardState, SharedState};

/// Event types that the wizard responds to
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// Individual key press
    Key(KeyEvent),
    /// Timer tick for animations (60fps)
    Tick,
    /// Terminal resize event
    Resize(u16, u16),
}

/// Navigation actions based on key events
#[derive(Clone, Debug, PartialEq)]
pub enum NavigationAction {
    /// Confirm / proceed to next step
    Confirm,
    /// Go back to previous step
    Back,
    /// Move selection up
    Up,
    /// Move selection down
    Down,
    /// Move to next focusable element
    NextFocus,
    /// Move to previous focusable element
    PrevFocus,
    /// Exit the application
    Exit,
    /// Insert character into input field
    InsertChar(char),
    /// Delete character from input field
    Delete,
    /// No navigation action
    None,
}

/// Event handler configuration
pub struct EventHandler {
    /// Tick rate for animations (16ms = ~60fps)
    tick_rate: Duration,
    /// Polling interval for user input
    poll_rate: Duration,
    /// Last tick timestamp
    last_tick: Instant,
}

impl EventHandler {
    /// Create a new event handler with default rates
    pub fn new() -> Self {
        Self {
            tick_rate: Duration::from_millis(16), // ~60fps
            poll_rate: Duration::from_millis(50), // 20 polls per second
            last_tick: Instant::now(),
        }
    }

    /// Convert Crossterm event to our Event enum
    fn to_event(crossterm_event: CrosstermEvent) -> Option<Event> {
        match crossterm_event {
            CrosstermEvent::Key(key_event) => Some(Event::Key(key_event)),
            CrosstermEvent::Resize(width, height) => Some(Event::Resize(width, height)),
            CrosstermEvent::Paste(_) => None, // Not handling paste for now
        }
    }

    /// Convert key event to navigation action
    pub fn key_to_action(key: KeyEvent) -> NavigationAction {
        match key.code {
            KeyCode::Enter => NavigationAction::Confirm,
            KeyCode::Esc => NavigationAction::Back,
            KeyCode::Up => NavigationAction::Up,
            KeyCode::Down => NavigationAction::Down,
            KeyCode::Left => NavigationAction::Back,
            KeyCode::Right => NavigationAction::Confirm,
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    NavigationAction::PrevFocus
                } else {
                    NavigationAction::NextFocus
                }
            }
            KeyCode::Backspace => NavigationAction::Delete,
            KeyCode::Char(c) => {
                // Handle Ctrl+C for exit
                if c == 'c' && key.modifiers.contains(KeyModifiers::CONTROL) {
                    NavigationAction::Exit
                } else {
                    NavigationAction::InsertChar(c)
                }
            }
            _ => NavigationAction::None,
        }
    }

    /// Poll for next event (non-blocking)
    pub fn next_event(&mut self) -> Option<Event> {
        let now = Instant::now();

        // Check if it's time for a tick
        if now.duration_since(self.last_tick) >= self.tick_rate {
            self.last_tick = now;
            return Some(Event::Tick);
        }

        // Poll for user input (non-blocking)
        if event::poll(self.poll_rate).unwrap_or(false) {
            if let Ok(crossterm_event) = event::read() {
                return Self::to_event(crossterm_event);
            }
        }

        None
    }

    /// Handle navigation action and update state
    pub fn handle_navigation(action: NavigationAction, state: &mut WizardState) {
        match action {
            NavigationAction::Confirm => {
                // Mark current step complete and go forward
                state.mark_step_complete();
                if let Err(e) = state.go_next() {
                    state.set_error(super::error::ErrorMode::SystemError {
                        message: e,
                        recoverable: true,
                    });
                }
            }
            NavigationAction::Back => {
                state.go_back();
                state.clear_error();
            }
            NavigationAction::Up => {
                // For selection-based steps, move selection up
                // This would be handled by the step itself
            }
            NavigationAction::Down => {
                // For selection-based steps, move selection down
            }
            NavigationAction::NextFocus | NavigationAction::PrevFocus => {
                // Cycle focus between UI elements
            }
            NavigationAction::Exit => {
                // Exit the application
                // This would trigger a graceful shutdown
            }
            NavigationAction::InsertChar(_) => {
                // Pass to current step for text input
            }
            NavigationAction::Delete => {
                // Handle backspace in text input
            }
            NavigationAction::None => {}
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the event loop with a shared state
///
/// This function runs the main event loop, processing keyboard input
/// and timer ticks, updating the shared WizardState.
///
/// # Arguments
///
/// * `state` - Shared wizard state wrapped in Arc<RwLock<>>
pub fn run_event_loop(state: SharedState) {
    let mut handler = EventHandler::new();

    loop {
        if let Some(event) = handler.next_event() {
            match event {
                Event::Key(key_event) => {
                    // Handle Ctrl+C for graceful exit
                    if key_event.code == KeyCode::Char('c')
                        && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        break;
                    }

                    let action = EventHandler::key_to_action(key_event);
                    if action == NavigationAction::Exit {
                        break;
                    }

                    let mut state_guard = state.write().unwrap();
                    EventHandler::handle_navigation(action, &mut *state_guard);
                }
                Event::Tick => {
                    // Handle animation ticks
                    // This is where animation state would be updated
                }
                Event::Resize(_, _) => {
                    // Handle terminal resize
                    // The rendering system would use this to recalculate layout
                }
            }
        }
    }
}
