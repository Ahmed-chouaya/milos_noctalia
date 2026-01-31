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

/// Timezone selection step with region/city hierarchy and type-to-filter
pub struct TimezoneStep {
    selected: usize,
    filter: String,
    filtered_timezones: Vec<&'static str>,
}

static TIMEZONES: &[&str] = &[
    // America
    "America/New_York", "America/Los_Angeles", "America/Chicago",
    "America/Denver", "America/Sao_Paulo",
    // Europe
    "Europe/London", "Europe/Berlin", "Europe/Paris", "Europe/Rome",
    "Europe/Madrid", "Europe/Amsterdam", "Europe/Moscow",
    // Asia
    "Asia/Tokyo", "Asia/Shanghai", "Asia/Dubai", "Asia/Singapore",
    "Asia/Kolkata", "Asia/Seoul",
    // Oceania
    "Australia/Sydney", "Australia/Melbourne", "Pacific/Auckland",
];

impl TimezoneStep {
    pub fn new() -> Self {
        Self {
            selected: 0,
            filter: String::new(),
            filtered_timezones: TIMEZONES.to_vec(),
        }
    }

    fn apply_filter(&mut self) {
        if self.filter.is_empty() {
            self.filtered_timezones = TIMEZONES.to_vec();
        } else {
            self.filtered_timezones = TIMEZONES
                .iter()
                .filter(|&&tz| tz.to_lowercase().contains(&self.filter.to_lowercase()))
                .copied()
                .collect();
        }
        // Reset selection to top when filter changes
        if self.selected >= self.filtered_timezones.len() {
            self.selected = 0;
        }
    }
}

impl WizardStep for TimezoneStep {
    fn title(&self) -> &'static str {
        "Timezone"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        // Show filter input at top
        let filter_text = format!("Filter: {}", self.filter);
        let filter_cursor = if !self.filter.is_empty() {
            " ←"
        } else {
            ""
        };
        let filter_para = Paragraph::new(format!("{}{}", filter_text, filter_cursor))
            .style(Style::default().fg(Color::Cyan))
            .block(Block::borders().title("Type to filter"));

        // Calculate list area (below filter)
        let list_height = area.height.saturating_sub(3);
        let list_area = Rect::new(area.x, area.y + 2, area.width, list_height);

        let items: Vec<ListItem> = self.filtered_timezones
            .iter()
            .enumerate()
            .map(|(i, &tz)| {
                let prefix = if i == self.selected { "▶ " } else { "  " };
                let suffix = if Some(tz) == state.timezone.as_deref() { " ◀" } else { "" };
                ListItem::new(format!("{}{}{}", prefix, tz, suffix))
            })
            .collect();

        let list = List::new(items)
            .block(Block::borders().title("Select Timezone"))
            .style(Style::default().fg(Color::White));

        frame.render_widget(filter_para, Rect::new(area.x, area.y, area.width, 2));
        frame.render_widget(list, list_area);

        // Show match count if filter is active
        if !self.filter.is_empty() {
            let match_count = format!("Showing {} of {} timezones", self.filtered_timezones.len(), TIMEZONES.len());
            let count_para = Paragraph::new(match_count)
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::borders().borders(Borders::NONE));

            let count_area = Rect::new(area.x, area.y + area.height - 1, area.width, 1);
            frame.render_widget(count_para, count_area);
        }
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
                    if self.selected < self.filtered_timezones.len() - 1 {
                        self.selected += 1;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    if !self.filtered_timezones.is_empty() {
                        state.timezone = Some(self.filtered_timezones[self.selected].to_string());
                        state.mark_step_complete();
                        state.go_next()?;
                    }
                }
                crossterm::event::KeyCode::Char(c) => {
                    self.filter.push(c);
                    self.apply_filter();
                }
                crossterm::event::KeyCode::Backspace => {
                    self.filter.pop();
                    self.apply_filter();
                }
                crossterm::event::KeyCode::Esc => {
                    if self.filter.is_empty() {
                        state.go_back()?;
                    } else {
                        self.filter.clear();
                        self.apply_filter();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate(&self, state: &WizardState) -> Result<(), String> {
        if state.timezone.is_some() {
            Ok(())
        } else {
            Err("Please select a timezone".to_string())
        }
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        state.timezone.is_some()
    }
}



/// Keyboard layout selection step with type-to-filter
pub struct KeyboardStep {
    selected: usize,
    filter: String,
    filtered_layouts: Vec<(&'static str, &'static str)>,
}

static KEYBOARD_LAYOUTS: &[(&'static str, &'static str)] = &[
    ("us", "US (QWERTY)"),
    ("gb", "UK (QWERTY)"),
    ("de", "German (QWERTZ)"),
    ("fr", "French (AZERTY)"),
    ("es", "Spanish"),
    ("it", "Italian"),
    ("jp", "Japanese"),
    ("ru", "Russian"),
    ("br", "Brazilian (ABNT2)"),
    ("sv", "Swedish"),
    ("no", "Norwegian"),
    ("dk", "Danish"),
    ("fi", "Finnish"),
    ("pt", "Portuguese"),
    ("pl", "Polish"),
    ("cz", "Czech"),
    ("hu", "Hungarian"),
    ("tr", "Turkish"),
];

impl KeyboardStep {
    pub fn new() -> Self {
        Self {
            selected: 0,
            filter: String::new(),
            filtered_layouts: KEYBOARD_LAYOUTS.to_vec(),
        }
    }

    fn apply_filter(&mut self) {
        if self.filter.is_empty() {
            self.filtered_layouts = KEYBOARD_LAYOUTS.to_vec();
        } else {
            self.filtered_layouts = KEYBOARD_LAYOUTS
                .iter()
                .filter(|(code, name)| {
                    code.to_lowercase().contains(&self.filter.to_lowercase())
                        || name.to_lowercase().contains(&self.filter.to_lowercase())
                })
                .copied()
                .collect();
        }
        // Reset selection to top when filter changes
        if self.selected >= self.filtered_layouts.len() {
            self.selected = 0;
        }
    }
}

impl WizardStep for KeyboardStep {
    fn title(&self) -> &'static str {
        "Keyboard Layout"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        // Show filter input at top
        let filter_text = format!("Filter: {}", self.filter);
        let filter_cursor = if !self.filter.is_empty() {
            " ←"
        } else {
            ""
        };
        let filter_para = Paragraph::new(format!("{}{}", filter_text, filter_cursor))
            .style(Style::default().fg(Color::Cyan))
            .block(Block::borders().title("Type to filter"));

        // Calculate list area (below filter)
        let list_height = area.height.saturating_sub(3);
        let list_area = Rect::new(area.x, area.y + 2, area.width, list_height);

        let items: Vec<ListItem> = self.filtered_layouts
            .iter()
            .enumerate()
            .map(|(i, &(code, name))| {
                let prefix = if i == self.selected { "▶ " } else { "  " };
                let suffix = if Some(code) == state.keyboard_layout.as_deref() { " ◀" } else { "" };
                ListItem::new(format!("{}{}{}", prefix, name, suffix))
            })
            .collect();

        let list = List::new(items)
            .block(Block::borders().title("Select Keyboard Layout"))
            .style(Style::default().fg(Color::White));

        frame.render_widget(filter_para, Rect::new(area.x, area.y, area.width, 2));
        frame.render_widget(list, list_area);

        // Show match count if filter is active
        if !self.filter.is_empty() {
            let match_count = format!("Showing {} of {} layouts", self.filtered_layouts.len(), KEYBOARD_LAYOUTS.len());
            let count_para = Paragraph::new(match_count)
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::borders().borders(Borders::NONE));

            let count_area = Rect::new(area.x, area.y + area.height - 1, area.width, 1);
            frame.render_widget(count_para, count_area);
        }
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
                    if self.selected < self.filtered_layouts.len() - 1 {
                        self.selected += 1;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    if !self.filtered_layouts.is_empty() {
                        state.keyboard_layout = Some(self.filtered_layouts[self.selected].0.to_string());
                        state.mark_step_complete();
                        state.go_next()?;
                    }
                }
                crossterm::event::KeyCode::Char(c) => {
                    self.filter.push(c);
                    self.apply_filter();
                }
                crossterm::event::KeyCode::Backspace => {
                    self.filter.pop();
                    self.apply_filter();
                }
                crossterm::event::KeyCode::Esc => {
                    if self.filter.is_empty() {
                        state.go_back()?;
                    } else {
                        self.filter.clear();
                        self.apply_filter();
                    }
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

/// Account creation step with hostname, username, full name, git config
pub struct AccountStep {
    focus_field: usize, // 0: hostname, 1: username, 2: full_name, 3: git_username, 4: git_email
    hostname_buffer: String,
    username_buffer: String,
    full_name_buffer: String,
    git_username_buffer: String,
    git_email_buffer: String,
}

impl AccountStep {
    pub fn new() -> Self {
        Self {
            focus_field: 0,
            hostname_buffer: String::new(),
            username_buffer: String::new(),
            full_name_buffer: String::new(),
            git_username_buffer: String::new(),
            git_email_buffer: String::new(),
        }
    }

    /// Initialize buffers from state (for returning to step)
    pub fn from_state(state: &WizardState) -> Self {
        Self {
            focus_field: 0,
            hostname_buffer: state.hostname.clone().unwrap_or_default(),
            username_buffer: state.username.clone().unwrap_or_default(),
            full_name_buffer: state.full_name.clone().unwrap_or_default(),
            git_username_buffer: state.git_username.clone().unwrap_or_default(),
            git_email_buffer: state.git_email.clone().unwrap_or_default(),
        }
    }
}

impl WizardStep for AccountStep {
    fn title(&self) -> &'static str {
        "Account"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        // Use buffers if available, otherwise fall back to state
        let hostname = if !self.hostname_buffer.is_empty() {
            &self.hostname_buffer
        } else {
            state.hostname.as_deref().unwrap_or("")
        };
        let username = if !self.username_buffer.is_empty() {
            &self.username_buffer
        } else {
            state.username.as_deref().unwrap_or("")
        };
        let full_name = if !self.full_name_buffer.is_empty() {
            &self.full_name_buffer
        } else {
            state.full_name.as_deref().unwrap_or("")
        };
        let git_username = if !self.git_username_buffer.is_empty() {
            &self.git_username_buffer
        } else {
            state.git_username.as_deref().unwrap_or("")
        };
        let git_email = if !self.git_email_buffer.is_empty() {
            &self.git_email_buffer
        } else {
            state.git_email.as_deref().unwrap_or("")
        };

        let hostname_prefix = if self.focus_field == 0 { "▶ " } else { "  " };
        let username_prefix = if self.focus_field == 1 { "▶ " } else { "  " };
        let full_name_prefix = if self.focus_field == 2 { "▶ " } else { "  " };
        let git_username_prefix = if self.focus_field == 3 { "▶ " } else { "  " };
        let git_email_prefix = if self.focus_field == 4 { "▶ " } else { "  " };

        let text = vec![
            "Configure your account settings:",
            "",
            &format!("{}Hostname: {}", hostname_prefix, hostname),
            &format!("{}Username: {}", username_prefix, username),
            &format!("{}Full Name: {}", full_name_prefix, full_name),
            &format!("{}Git Username: {}", git_username_prefix, git_username),
            &format!("{}Git Email: {}", git_email_prefix, git_email),
            "",
            "Use arrow keys or Tab to navigate between fields.",
            "Press Enter to save each field and advance.",
        ];

        let paragraph = Paragraph::new(text.join("\n"))
            .style(Style::default().fg(Color::White))
            .block(Block::borders().title("Account"));

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
                    if self.focus_field < 4 {
                        self.focus_field += 1;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    // Save current field to state
                    match self.focus_field {
                        0 => {
                            state.hostname = Some(self.hostname_buffer.clone());
                            self.focus_field = 1;
                        }
                        1 => {
                            state.username = Some(self.username_buffer.clone());
                            self.focus_field = 2;
                        }
                        2 => {
                            state.full_name = Some(self.full_name_buffer.clone());
                            self.focus_field = 3;
                        }
                        3 => {
                            state.git_username = Some(self.git_username_buffer.clone());
                            self.focus_field = 4;
                        }
                        4 => {
                            state.git_email = Some(self.git_email_buffer.clone());
                            // Check if all required fields are valid
                            if state.hostname.is_some() 
                                && state.username.is_some()
                                && state.full_name.is_some()
                                && state.git_username.is_some()
                                && state.git_email.is_some()
                            {
                                state.mark_step_complete();
                                state.go_next()?;
                            }
                        }
                        _ => {}
                    }
                }
                crossterm::event::KeyCode::Char(c) => {
                    match self.focus_field {
                        0 => self.hostname_buffer.push(c),
                        1 => self.username_buffer.push(c),
                        2 => self.full_name_buffer.push(c),
                        3 => self.git_username_buffer.push(c),
                        4 => self.git_email_buffer.push(c),
                        _ => {}
                    }
                }
                crossterm::event::KeyCode::Backspace => {
                    match self.focus_field {
                        0 => self.hostname_buffer.pop(),
                        1 => self.username_buffer.pop(),
                        2 => self.full_name_buffer.pop(),
                        3 => self.git_username_buffer.pop(),
                        4 => self.git_email_buffer.pop(),
                        _ => {}
                    }
                }
                crossterm::event::KeyCode::Ctrl('u') => {
                    // Clear current field
                    match self.focus_field {
                        0 => self.hostname_buffer.clear(),
                        1 => self.username_buffer.clear(),
                        2 => self.full_name_buffer.clear(),
                        3 => self.git_username_buffer.clear(),
                        4 => self.git_email_buffer.clear(),
                        _ => {}
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
        if state.full_name.is_none() {
            return Err("Full name is required".to_string());
        }
        if state.git_username.is_none() {
            return Err("Git username is required".to_string());
        }
        if state.git_email.is_none() {
            return Err("Git email is required".to_string());
        }
        Ok(())
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        state.hostname.is_some() 
            && state.username.is_some() 
            && state.full_name.is_some()
            && state.git_username.is_some()
            && state.git_email.is_some()
    }
}

/// Path configuration step for Noctalia
pub struct PathsStep {
    focus_field: usize,  // 0: wallpaper_dir, 1: avatar_path, 2: screenshot_dir
    wallpaper_buffer: String,
    avatar_buffer: String,
    screenshot_buffer: String,
}

impl PathsStep {
    pub fn new() -> Self {
        Self {
            focus_field: 0,
            wallpaper_buffer: String::new(),
            avatar_buffer: String::new(),
            screenshot_buffer: String::new(),
        }
    }

    /// Initialize buffers from state (for returning to step)
    pub fn from_state(state: &WizardState) -> Self {
        Self {
            focus_field: 0,
            wallpaper_buffer: state.wallpaper_dir.clone().unwrap_or_default(),
            avatar_buffer: state.avatar_path.clone().unwrap_or_default(),
            screenshot_buffer: state.screenshot_dir.clone().unwrap_or_default(),
        }
    }
}

impl WizardStep for PathsStep {
    fn title(&self) -> &'static str {
        "Paths & Directories"
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, state: &WizardState, area: Rect) {
        // Use buffers if available, otherwise fall back to state defaults
        let wallpaper = if !self.wallpaper_buffer.is_empty() {
            &self.wallpaper_buffer
        } else {
            state.wallpaper_dir.as_deref().unwrap_or("")
        };
        let avatar = if !self.avatar_buffer.is_empty() {
            &self.avatar_buffer
        } else {
            state.avatar_path.as_deref().unwrap_or("")
        };
        let screenshot = if !self.screenshot_buffer.is_empty() {
            &self.screenshot_buffer
        } else {
            state.screenshot_dir.as_deref().unwrap_or("")
        };

        let wallpaper_prefix = if self.focus_field == 0 { "▶ " } else { "  " };
        let avatar_prefix = if self.focus_field == 1 { "▶ " } else { "  " };
        let screenshot_prefix = if self.focus_field == 2 { "▶ " } else { "  " };

        // Check if avatar is optional (empty or None)
        let avatar_display = if avatar.is_empty() {
            "(none)".to_string()
        } else {
            avatar.to_string()
        };

        let text = vec![
            "Configure paths for Noctalia:",
            "",
            &format!("{}Wallpaper Directory: {}", wallpaper_prefix, wallpaper),
            &format!("{}Avatar Image: {} (optional)", avatar_prefix, avatar_display),
            &format!("{}Screenshot Directory: {}", screenshot_prefix, screenshot),
            "",
            "These paths are used by Noctalia for:",
            "  • Finding wallpaper images",
            "  • Displaying your avatar in UI",
            "  • Saving screenshots",
            "",
            "Use arrow keys or Tab to navigate between fields.",
            "Press Enter to save each field and advance.",
        ];

        let paragraph = Paragraph::new(text.join("\n"))
            .style(Style::default().fg(Color::White))
            .block(Block::borders().title("Paths & Directories"));

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
                    if self.focus_field < 2 {
                        self.focus_field += 1;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    // Save current field to state
                    match self.focus_field {
                        0 => {
                            state.wallpaper_dir = Some(self.wallpaper_buffer.clone());
                            self.focus_field = 1;
                        }
                        1 => {
                            // Avatar is optional - can be empty/None
                            if self.avatar_buffer.is_empty() {
                                state.avatar_path = None;
                            } else {
                                state.avatar_path = Some(self.avatar_buffer.clone());
                            }
                            self.focus_field = 2;
                        }
                        2 => {
                            state.screenshot_dir = Some(self.screenshot_buffer.clone());
                            // Check if required fields are valid
                            if state.wallpaper_dir.as_ref().map_or(false, |s| !s.is_empty())
                                && state.screenshot_dir.as_ref().map_or(false, |s| !s.is_empty())
                            {
                                state.mark_step_complete();
                                state.go_next()?;
                            }
                        }
                        _ => {}
                    }
                }
                crossterm::event::KeyCode::Char(c) => {
                    match self.focus_field {
                        0 => self.wallpaper_buffer.push(c),
                        1 => self.avatar_buffer.push(c),
                        2 => self.screenshot_buffer.push(c),
                        _ => {}
                    }
                }
                crossterm::event::KeyCode::Backspace => {
                    match self.focus_field {
                        0 => self.wallpaper_buffer.pop(),
                        1 => self.avatar_buffer.pop(),
                        2 => self.screenshot_buffer.pop(),
                        _ => {}
                    };
                }
                crossterm::event::KeyCode::Ctrl('u') => {
                    // Clear current field
                    match self.focus_field {
                        0 => self.wallpaper_buffer.clear(),
                        1 => self.avatar_buffer.clear(),
                        2 => self.screenshot_buffer.clear(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate(&self, state: &WizardState) -> Result<(), String> {
        // Check required fields
        if state.wallpaper_dir.is_none() || state.wallpaper_dir.as_ref().map_or(true, |s| s.is_empty()) {
            return Err("Wallpaper directory is required".to_string());
        }
        if state.screenshot_dir.is_none() || state.screenshot_dir.as_ref().map_or(true, |s| s.is_empty()) {
            return Err("Screenshot directory is required".to_string());
        }
        Ok(())
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        state.wallpaper_dir.is_some()
            && state.wallpaper_dir.as_ref().map_or(false, |s| !s.is_empty())
            && state.screenshot_dir.is_some()
            && state.screenshot_dir.as_ref().map_or(false, |s| !s.is_empty())
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
        Step::Timezone => Box::new(TimezoneStep::new()),
        Step::Keyboard => Box::new(KeyboardStep::new()),
        Step::Account => Box::new(AccountStep::new()),
        Step::Paths => Box::new(PathsStep::new()),
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
