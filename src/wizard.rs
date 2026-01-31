use std::io::Stdout;
use std::time::Instant;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
    Frame,
};
use ratatui::layout::{Layout, Direction, Constraint, Rect};
use ratatui::style::{Style, Color, Modifier};
use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute,
};
use crate::state::{WizardState, Step, SharedState};
use crate::event::{Event, EventHandler, run_event_loop};
use crate::logo::{LogoAnimation, render_logo};
use crate::error::{ErrorModal, render_error_modal};

/// Trait for wizard steps - each step implements this
pub trait WizardStep {
    /// Get the step title
    fn title(&self) -> &'static str;

    /// Render the step content
    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect);

    /// Handle user input
    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String>;

    /// Validate the step is complete
    fn validate(&self, state: &WizardState) -> Result<(), String>;

    /// Check if step is complete
    fn is_complete(&self, state: &WizardState) -> bool;
}

/// Welcome step - shows animated logo and "Press Enter to begin"
pub struct WelcomeStep {
    animation: LogoAnimation,
    start_time: Instant,
}

impl WelcomeStep {
    pub fn new() -> Self {
        Self {
            animation: LogoAnimation::new(),
            start_time: Instant::now(),
        }
    }
}

impl WizardStep for WelcomeStep {
    fn title(&self) -> &'static str {
        "Welcome"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        // Update animation
        let mut animation = self.animation.clone();
        animation.update();

        // Render the logo
        render_logo(frame, area, &animation);

        // If animation is complete, show "Press Enter to begin" below
        if animation.complete {
            let prompt = "Press ENTER to begin";
            let prompt_area = Rect::new(
                area.x + area.width / 2 - prompt.len() as u16 / 2,
                area.y + area.height - 3,
                prompt.len() as u16,
                1,
            );

            let paragraph = Paragraph::new(prompt)
                .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                .block(Block::borders().borders(Borders::NONE));

            frame.render_widget(paragraph, prompt_area);
        }
    }

    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            if key.code == crossterm::event::KeyCode::Enter {
                // Only proceed if animation is complete
                if self.animation.complete {
                    state.mark_step_complete();
                    state.go_next()?;
                }
            }
        }
        Ok(())
    }

    fn validate(&self, _state: &WizardState) -> Result<(), String> {
        Ok(())
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        state.is_current_step_complete()
    }
}

/// Locale selection step
pub struct LocaleStep {
    selected: usize,
    locales: Vec<&'static str>,
}

impl LocaleStep {
    pub fn new() -> Self {
        Self {
            selected: 0,
            locales: vec!["en_US.UTF-8", "en_GB.UTF-8", "de_DE.UTF-8", "fr_FR.UTF-8", "es_ES.UTF-8"],
        }
    }
}

impl WizardStep for LocaleStep {
    fn title(&self) -> &'static str {
        "Language & Locale"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        let items: Vec<ListItem> = self.locales
            .iter()
            .enumerate()
            .map(|(i, &locale)| {
                let prefix = if i == self.selected { "▶ " } else { "  " };
                let suffix = if locale == state.locale.as_deref().unwrap_or("") { " ◀" } else { "" };
                ListItem::new(format!("{}{}{}", prefix, locale, suffix))
            })
            .collect();

        let list = List::new(items)
            .block(Block::borders().title("Select Language & Locale"))
            .style(Style::default().fg(Color::White));

        frame.render_widget(list, area);
    }

    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            match key.code {
                crossterm::event::KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                crossterm::event::KeyCode::Down => {
                    if self.selected < self.locales.len() - 1 {
                        self.selected += 1;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    state.locale = Some(self.locales[self.selected].to_string());
                    state.mark_step_complete();
                    state.go_next()?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate(&self, state: &WizardState) -> Result<(), String> {
        if state.locale.is_some() {
            Ok(())
        } else {
            Err("Please select a locale".to_string())
        }
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        state.locale.is_some()
    }
}

/// Keyboard layout selection step
pub struct KeyboardStep {
    selected: usize,
    layouts: Vec<&'static str>,
}

impl KeyboardStep {
    pub fn new() -> Self {
        Self {
            selected: 0,
            layouts: vec!["us", "gb", "de", "fr", "es", "it", "jp"],
        }
    }
}

impl WizardStep for KeyboardStep {
    fn title(&self) -> &'static str {
        "Keyboard Layout"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        let items: Vec<ListItem> = self.layouts
            .iter()
            .enumerate()
            .map(|(i, &layout)| {
                let prefix = if i == self.selected { "▶ " } else { "  " };
                let suffix = if Some(layout) == state.keyboard_layout.as_deref() { " ◀" } else { "" };
                let name = match layout {
                    "us" => "US (QWERTY)",
                    "gb" => "UK (QWERTY)",
                    "de" => "German (QWERTZ)",
                    "fr" => "French (AZERTY)",
                    "es" => "Spanish",
                    "it" => "Italian",
                    "jp" => "Japanese",
                    _ => layout,
                };
                ListItem::new(format!("{}{}{}", prefix, name, suffix))
            })
            .collect();

        let list = List::new(items)
            .block(Block::borders().title("Select Keyboard Layout"))
            .style(Style::default().fg(Color::White));

        frame.render_widget(list, area);
    }

    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            match key.code {
                crossterm::event::KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                crossterm::event::KeyCode::Down => {
                    if self.selected < self.layouts.len() - 1 {
                        self.selected += 1;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    state.keyboard_layout = Some(self.layouts[self.selected].to_string());
                    state.mark_step_complete();
                    state.go_next()?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate(&self, state: &WizardState) -> Result<(), String> {
        if state.keyboard_layout.is_some() {
            Ok(())
        } else {
            Err("Please select a keyboard layout".to_string())
        }
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        state.keyboard_layout.is_some()
    }
}

/// User creation step
pub struct UserStep {
    focus_field: usize, // 0: hostname, 1: username
    hostname_buffer: String,
    username_buffer: String,
}

impl UserStep {
    pub fn new() -> Self {
        Self {
            focus_field: 0,
            hostname_buffer: String::new(),
            username_buffer: String::new(),
        }
    }
}

impl WizardStep for UserStep {
    fn title(&self) -> &'static str {
        "User Account"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        let hostname = state.hostname.as_deref().unwrap_or("");
        let username = state.username.as_deref().unwrap_or("");

        let hostname_prefix = if self.focus_field == 0 { "▶ " } else { "  " };
        let username_prefix = if self.focus_field == 1 { "▶ " } else { "  " };

        let text = vec![
            "",
            &format!("{}Hostname: {}", hostname_prefix, hostname),
            &format!("{}Username: {}", username_prefix, username),
            "",
            "  Use arrow keys to select fields, type to enter values,",
            "  and press Enter when both fields are complete.",
        ];

        let paragraph = Paragraph::new(text.join("\n"))
            .style(Style::default().fg(Color::White))
            .block(Block::borders().title("User Account"));

        frame.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            match key.code {
                crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Tab => {
                    if self.focus_field > 0 {
                        self.focus_field -= 1;
                    }
                }
                crossterm::event::KeyCode::Down | crossterm::event::KeyCode::BackTab => {
                    if self.focus_field < 1 {
                        self.focus_field += 1;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    if self.focus_field == 0 {
                        state.hostname = Some(self.hostname_buffer.clone());
                        self.focus_field = 1;
                    } else {
                        state.username = Some(self.username_buffer.clone());
                        state.mark_step_complete();
                        state.go_next()?;
                    }
                }
                crossterm::event::KeyCode::Char(c) => {
                    if self.focus_field == 0 {
                        self.hostname_buffer.push(c);
                    } else {
                        self.username_buffer.push(c);
                    }
                }
                crossterm::event::KeyCode::Backspace => {
                    if self.focus_field == 0 {
                        self.hostname_buffer.pop();
                    } else {
                        self.username_buffer.pop();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate(&self, state: &WizardState) -> Result<(), String> {
        if state.hostname.is_none() {
            return Err("Hostname is required".to_string());
        }
        if state.username.is_none() {
            return Err("Username is required".to_string());
        }

        if let Some(hostname) = &state.hostname {
            if hostname.contains(' ') {
                return Err("Hostname cannot contain spaces".to_string());
            }
        }

        if let Some(username) = &state.username {
            if !username.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
                return Err("Username must be lowercase alphanumeric".to_string());
            }
        }

        Ok(())
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        state.hostname.is_some() && state.username.is_some()
    }
}

/// Summary step - review before install
pub struct SummaryStep;

impl WizardStep for SummaryStep {
    fn title(&self) -> &'static str {
        "Review & Install"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        let hostname = state.hostname.as_deref().unwrap_or("(not set)");
        let username = state.username.as_deref().unwrap_or("(not set)");
        let locale = state.locale.as_deref().unwrap_or("(not set)");
        let keyboard = state.keyboard_layout.as_deref().unwrap_or("(not set)");

        let text = vec![
            "Review your configuration:",
            "",
            &format!("  Hostname:        {}", hostname),
            &format!("  Username:        {}", username),
            &format!("  Locale:          {}", locale),
            &format!("  Keyboard:        {}", keyboard),
            "",
            "Press Enter to begin installation.",
            "Press Escape to go back and make changes.",
        ];

        let paragraph = Paragraph::new(text.join("\n"))
            .style(Style::default().fg(Color::White))
            .block(Block::borders().title("Review Configuration"));

        frame.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            if key.code == crossterm::event::KeyCode::Enter {
                state.mark_step_complete();
                // Installation would happen here in Phase 4
            }
        }
        Ok(())
    }

    fn validate(&self, _state: &WizardState) -> Result<(), String> {
        Ok(())
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        state.is_current_step_complete()
    }
}

/// Render the sidebar showing all steps with progress
pub fn render_sidebar(frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
    let steps = Step::all_steps();
    let current_step = state.current_step();

    let items: Vec<ListItem> = steps
        .iter()
        .enumerate()
        .map(|(i, step)| {
            let marker = if state.completed_steps[i] {
                "✓"
            } else if i == state.current_step {
                "▶"
            } else {
                "○"
            };

            let style = if state.completed_steps[i] {
                Style::default().fg(Color::Green)
            } else if i == state.current_step {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            ListItem::new(format!("{} {}", marker, step.title()))
                .style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::borders().title("Steps"))
        .style(Style::default().bg(Color::Rgb(30, 30, 30)));

    frame.render_widget(list, area);
}

/// Create the current step based on state
pub fn create_current_step(step: Step) -> Box<dyn WizardStep> {
    match step {
        Step::Welcome => Box::new(WelcomeStep),
        Step::Locale => Box::new(LocaleStep::new()),
        Step::Keyboard => Box::new(KeyboardStep::new()),
        Step::Network => Box::new(WelcomeStep), // Placeholder
        Step::Disk => Box::new(WelcomeStep),    // Placeholder
        Step::User => Box::new(UserStep::new()),
        Step::Packages => Box::new(WelcomeStep), // Placeholder
        Step::Summary => Box::new(SummaryStep),
    }
}

/// Run the wizard loop
pub fn run_wizard() -> Result<(), String> {
    let state = Arc::new(RwLock::new(WizardState::new()));

    // Enter raw mode
    enable_raw_mode()?;

    // Create terminal
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Enable bracketed paste
    execute!(terminal.backend_mut(), EnableBracketedPaste)?;

    // Main render loop
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                ])
                .split(frame.size());

            let state_guard = state.read().unwrap();
            let current_step = state_guard.current_step();
            let mut step = create_current_step(current_step);

            // Render sidebar
            render_sidebar(frame, &state_guard, chunks[0]);

            // Render main content
            step.render(frame, &state_guard, chunks[1]);

            // Render error modal if there's an error
            if let Some(ref error_modal) = state_guard.error_mode {
                render_error_modal(frame, error_modal, frame.size());
            }
        })?;

        // Handle one event
        let event = {
            let mut handler = EventHandler::new();
            loop {
                if let Some(e) = handler.next_event() {
                    break e;
                }
            }
        };

        // Process event
        {
            let mut state_guard = state.write().unwrap();

            // Check for error mode
            if let Some(error_modal) = &state_guard.error_mode {
                // Handle error state - pass to error modal
                if let Event::Key(key) = event {
                    if let Some(action) = error_modal.handle_input(key.code, key.modifiers) {
                        match action {
                            crate::error::ErrorAction::Dismiss | crate::error::ErrorAction::Cancel => {
                                state_guard.clear_error();
                            }
                            crate::error::ErrorAction::Exit => {
                                // Restore terminal and exit
                                execute!(terminal.backend_mut(), DisableBracketedPaste).ok();
                                execute!(terminal.backend_mut(), Clear(ClearType::All)).ok();
                                disable_raw_mode().ok();
                                return Ok(());
                            }
                            crate::error::ErrorAction::Retry => {
                                state_guard.clear_error();
                                // Retry logic would go here
                            }
                            crate::error::ErrorAction::ToggleBacktrace => {
                                // Handled by handle_input
                            }
                        }
                    }
                    continue;
                }
            }

            // Normal event processing
            if let Event::Key(key) = event {
                // Handle Ctrl+C
                if key.code == crossterm::event::KeyCode::Char('c')
                    && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                    break;
                }

                // Handle Escape for back
                if key.code == crossterm::event::KeyCode::Esc {
                    state_guard.go_back();
                    continue;
                }

                // Pass to current step
                let current_step = state_guard.current_step();
                drop(state_guard); // Release lock

                let mut step = create_current_step(current_step);
                let mut state_guard = state.write().unwrap();
                step.handle_input(event, &mut state_guard)?;

                // Validate step completion
                if step.validate(&state_guard).is_ok() {
                    // Auto-advance if complete
                }
            }
        }

        // Check if wizard is complete
        {
            let state_guard = state.read().unwrap();
            if state_guard.is_complete() {
                break;
            }
        }
    }

    // Cleanup
    execute!(terminal.backend_mut(), DisableBranketedPaste).ok();
    execute!(terminal.backend_mut(), Clear(ClearType::All)).ok();
    disable_raw_mode()?;

    Ok(())
}
