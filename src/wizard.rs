use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use ratatui::layout::{Layout, Direction, Constraint, Rect};
use ratatui::style::{Style, Color, Modifier};
use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute,
};
use crate::state::{WizardState, Step};
use crate::event::{Event, EventHandler};
use crate::logo::{LogoAnimation, render_logo};
use crate::error::render_error_modal;
use crate::generator::{self, UserConfig};

/// Trait for wizard steps - each step implements this
pub trait WizardStep {
    /// Get the step title
    fn title(&self) -> &'static str;

    /// Render the step content
    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect);

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
}

impl WelcomeStep {
    pub fn new() -> Self {
        Self {
            animation: LogoAnimation::new(),
        }
    }
}

impl WizardStep for WelcomeStep {
    fn title(&self) -> &'static str {
        "Welcome"
    }

    fn render(&self, frame: &mut Frame, _state: &WizardState, area: Rect) {
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
                .block(Block::new().borders(Borders::NONE));

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

    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect) {
        // Show filter input at top
        let filter_text = format!("Filter: {}", self.filter);
        let filter_cursor = if !self.filter.is_empty() {
            " ←"
        } else {
            ""
        };
        let filter_para = Paragraph::new(format!("{}{}", filter_text, filter_cursor))
            .style(Style::default().fg(Color::Cyan))
            .block(Block::new().title("Type to filter"));

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
            .block(Block::new().title("Select Timezone"))
            .style(Style::default().fg(Color::White));

        frame.render_widget(filter_para, Rect::new(area.x, area.y, area.width, 2));
        frame.render_widget(list, list_area);

        // Show match count if filter is active
        if !self.filter.is_empty() {
            let match_count = format!("Showing {} of {} timezones", self.filtered_timezones.len(), TIMEZONES.len());
            let count_para = Paragraph::new(match_count)
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::new().borders(Borders::NONE));

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
                        state.go_back();
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

    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect) {
        // Show filter input at top
        let filter_text = format!("Filter: {}", self.filter);
        let filter_cursor = if !self.filter.is_empty() {
            " ←"
        } else {
            ""
        };
        let filter_para = Paragraph::new(format!("{}{}", filter_text, filter_cursor))
            .style(Style::default().fg(Color::Cyan))
            .block(Block::new().title("Type to filter"));

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
            .block(Block::new().title("Select Keyboard Layout"))
            .style(Style::default().fg(Color::White));

        frame.render_widget(filter_para, Rect::new(area.x, area.y, area.width, 2));
        frame.render_widget(list, list_area);

        // Show match count if filter is active
        if !self.filter.is_empty() {
            let match_count = format!("Showing {} of {} layouts", self.filtered_layouts.len(), KEYBOARD_LAYOUTS.len());
            let count_para = Paragraph::new(match_count)
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::new().borders(Borders::NONE));

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
                        state.go_back();
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

    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect) {
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

        let text: Vec<String> = vec![
            "Configure your account settings:".to_string(),
            "".to_string(),
            format!("{}Hostname: {}", hostname_prefix, hostname),
            format!("{}Username: {}", username_prefix, username),
            format!("{}Full Name: {}", full_name_prefix, full_name),
            format!("{}Git Username: {}", git_username_prefix, git_username),
            format!("{}Git Email: {}", git_email_prefix, git_email),
            "".to_string(),
            "Use arrow keys or Tab to navigate between fields.".to_string(),
            "Press Enter to save each field and advance.".to_string(),
        ];

        let paragraph = Paragraph::new(text.join("\n"))
            .style(Style::default().fg(Color::White))
            .block(Block::new().title("Account"));

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
                crossterm::event::KeyCode::Char('u') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
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
                        0 => { let _ = self.hostname_buffer.pop(); }
                        1 => { let _ = self.username_buffer.pop(); }
                        2 => { let _ = self.full_name_buffer.pop(); }
                        3 => { let _ = self.git_username_buffer.pop(); }
                        4 => { let _ = self.git_email_buffer.pop(); }
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

    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect) {
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

        let text: Vec<String> = vec![
            "Configure paths for Noctalia:".to_string(),
            "".to_string(),
            format!("{}Wallpaper Directory: {}", wallpaper_prefix, wallpaper),
            format!("{}Avatar Image: {} (optional)", avatar_prefix, avatar_display),
            format!("{}Screenshot Directory: {}", screenshot_prefix, screenshot),
            "".to_string(),
            "These paths are used by Noctalia for:".to_string(),
            "  • Finding wallpaper images".to_string(),
            "  • Displaying your avatar in UI".to_string(),
            "  • Saving screenshots".to_string(),
            "".to_string(),
            "Use arrow keys or Tab to navigate between fields.".to_string(),
            "Press Enter to save each field and advance.".to_string(),
        ];

        let paragraph = Paragraph::new(text.join("\n"))
            .style(Style::default().fg(Color::White))
            .block(Block::new().title("Paths & Directories"));

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
                crossterm::event::KeyCode::Char('u') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                    // Clear current field
                    match self.focus_field {
                        0 => self.wallpaper_buffer.clear(),
                        1 => self.avatar_buffer.clear(),
                        2 => self.screenshot_buffer.clear(),
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
                        0 => { let _ = self.wallpaper_buffer.pop(); }
                        1 => { let _ = self.avatar_buffer.pop(); }
                        2 => { let _ = self.screenshot_buffer.pop(); }
                        _ => {}
                    };
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

/// Generation status for tracking progress
#[derive(Clone, Debug, PartialEq)]
pub enum GenerationStatus {
    Pending,
    Generating,
    Success,
    Error,
}

/// Generate step - generates configuration files and displays results
pub struct GenerateStep {
    /// Current state of generation
    pub status: GenerationStatus,
    /// Generated file paths
    pub generated_files: Vec<PathBuf>,
    /// Any error that occurred
    pub error: Option<String>,
}

impl GenerateStep {
    pub fn new() -> Self {
        Self {
            status: GenerationStatus::Pending,
            generated_files: Vec::new(),
            error: None,
        }
    }

    /// Generate configuration files
    pub fn generate(&mut self, state: &WizardState) {
        self.status = GenerationStatus::Generating;
        self.error = None;

        // Convert WizardState to UserConfig
        let config = UserConfig::from_wizard_state(state);

        // Create output directory
        let output_dir = PathBuf::from("milos-output");

        // Generate all configs
        match generator::generate_all(&config, &output_dir) {
            Ok(files) => {
                self.generated_files = files;
                self.status = GenerationStatus::Success;
            }
            Err(e) => {
                self.error = Some(e.to_string());
                self.status = GenerationStatus::Error;
            }
        }
    }
}

impl WizardStep for GenerateStep {
    fn title(&self) -> &'static str {
        "Generate Configuration"
    }

    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect) {
        match self.status {
            GenerationStatus::Pending => {
                let hostname = state.hostname.as_deref().unwrap_or("(not set)");
                let username = state.username.as_deref().unwrap_or("(not set)");

                let text: Vec<String> = vec![
                    "Generate your NixOS configuration:".to_string(),
                    "".to_string(),
                    format!("  Hostname:  {}", hostname),
                    format!("  Username:  {}", username),
                    "".to_string(),
                    "The following files will be generated:".to_string(),
                    "  • flake.nix (NixOS flake configuration)".to_string(),
                    "  • users.nix (User account configuration)".to_string(),
                    "  • git.nix (Git configuration)".to_string(),
                    "  • locale.nix (Locale settings)".to_string(),
                    "  • noctalia.nix (Noctalia shell config)".to_string(),
                    "  • niri/config.kdl (Niri compositor config)".to_string(),
                    "  • nix.conf (Nix configuration)".to_string(),
                    "".to_string(),
                    "Press ENTER to generate configuration files.".to_string(),
                    "Press ESC to go back.".to_string(),
                ];

                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::White))
                    .block(Block::new().title("Generate Configuration"));

                frame.render_widget(paragraph, area);
            }
            GenerationStatus::Generating => {
                let text: Vec<String> = vec![
                    "Generating configuration files...".to_string(),
                    "".to_string(),
                    "Please wait while your NixOS configuration is being generated.".to_string(),
                    "".to_string(),
                    "This may take a moment...".to_string(),
                ];

                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::new().title("Generating..."));

                frame.render_widget(paragraph, area);
            }
            GenerationStatus::Success => {
                let file_count = self.generated_files.len();

                let file_list: Vec<String> = self.generated_files
                    .iter()
                    .map(|p| format!("  • {}", p.display()))
                    .collect();

                let text: Vec<String> = vec![
                    "✓ Configuration generated successfully!".to_string(),
                    "".to_string(),
                    format!("Generated {} files:", file_count),
                    "".to_string(),
                    file_list.join("\n"),
                    "".to_string(),
                    "Your NixOS configuration is ready in the 'milos-output' directory.".to_string(),
                    "".to_string(),
                    "Press ENTER to continue.".to_string(),
                ];

                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::Green))
                    .block(Block::new().title("Success"));

                frame.render_widget(paragraph, area);
            }
            GenerationStatus::Error => {
                let error_msg = self.error.as_deref().unwrap_or("Unknown error");

                let text: Vec<String> = vec![
                    "✗ Error generating configuration".to_string(),
                    "".to_string(),
                    format!("Error: {}", error_msg),
                    "".to_string(),
                    "Press ENTER to retry.".to_string(),
                    "Press ESC to go back.".to_string(),
                ];

                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::Red))
                    .block(Block::new().title("Error"));

                frame.render_widget(paragraph, area);
            }
        }
    }

    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            match key.code {
                crossterm::event::KeyCode::Enter => {
                    match self.status {
                        GenerationStatus::Pending => {
                            // Start generation
                            self.generate(state);
                        }
                        GenerationStatus::Success => {
                            // Mark step complete and proceed
                            state.mark_step_complete();
                        }
                        GenerationStatus::Error => {
                            // Retry generation
                            self.generate(state);
                        }
                        GenerationStatus::Generating => {
                            // Ignore during generation
                        }
                    }
                }
                crossterm::event::KeyCode::Esc => {
                    // Go back
                    state.go_back();
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate(&self, _state: &WizardState) -> Result<(), String> {
        match self.status {
            GenerationStatus::Success => Ok(()),
            GenerationStatus::Error => Ok(()), // Allow retry
            _ => Err("Generation not complete".to_string()),
        }
    }

    fn is_complete(&self, state: &WizardState) -> bool {
        self.status == GenerationStatus::Success && state.is_current_step_complete()
    }
}

/// Summary step - review before install
pub struct SummaryStep;

impl WizardStep for SummaryStep {
    fn title(&self) -> &'static str {
        "Review & Install"
    }

    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect) {
        let hostname = state.hostname.as_deref().unwrap_or("(not set)");
        let username = state.username.as_deref().unwrap_or("(not set)");
        let locale = state.locale.as_deref().unwrap_or("(not set)");
        let keyboard = state.keyboard_layout.as_deref().unwrap_or("(not set)");

        let text: Vec<String> = vec![
            "Review your configuration:".to_string(),
            "".to_string(),
            format!("  Hostname:        {}", hostname),
            format!("  Username:        {}", username),
            format!("  Locale:          {}", locale),
            format!("  Keyboard:        {}", keyboard),
            "".to_string(),
            "Press Enter to begin installation.".to_string(),
            "Press Escape to go back and make changes.".to_string(),
        ];

        let paragraph = Paragraph::new(text.join("\n"))
            .style(Style::default().fg(Color::White))
            .block(Block::new().title("Review Configuration"));

        frame.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            if key.code == crossterm::event::KeyCode::Enter {
                state.mark_step_complete();
                state.go_next()?;
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
pub fn render_sidebar(frame: &mut Frame, state: &WizardState, area: Rect) {
    let steps = Step::all_steps();
    let _current_step = state.current_step();

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
        .block(Block::new().title("Steps"))
        .style(Style::default().bg(Color::Rgb(30, 30, 30)));

    frame.render_widget(list, area);
}

/// Create the current step based on state
pub fn create_current_step(step: Step) -> Box<dyn WizardStep> {
    match step {
        Step::Welcome => Box::new(WelcomeStep::new()),
        Step::Timezone => Box::new(TimezoneStep::new()),
        Step::Keyboard => Box::new(KeyboardStep::new()),
        Step::Account => Box::new(AccountStep::new()),
        Step::Paths => Box::new(PathsStep::new()),
        Step::Summary => Box::new(SummaryStep),
        Step::Generate => Box::new(GenerateStep::new()),
        Step::Execution => Box::new(ExecutionStep::new()),
        Step::Completion => Box::new(CompletionStep::new()),
    }
}

/// Run the wizard loop
pub fn run_wizard() -> Result<(), String> {
    let state = Arc::new(RwLock::new(WizardState::new()));

    // Enter raw mode
    enable_raw_mode().map_err(|e| e.to_string())?;

    // Create terminal
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend).map_err(|e| e.to_string())?;

    // Enable bracketed paste
    execute!(terminal.backend_mut(), EnableBracketedPaste).map_err(|e| e.to_string())?;

    // Main render loop
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                ])
                .split(frame.area());

            let state_guard = state.read().unwrap();
            let current_step = state_guard.current_step();
            let step = create_current_step(current_step);

            // Render sidebar
            render_sidebar(frame, &state_guard, chunks[0]);

            // Render main content
            step.render(frame, &state_guard, chunks[1]);

            // Render error modal if there's an error
            if let Some(ref error_modal) = state_guard.error_mode {
                render_error_modal(frame, error_modal, frame.area());
            }
        }).map_err(|e| e.to_string())?;

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
            if let Some(ref mut error_modal) = state_guard.error_mode {
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
    execute!(terminal.backend_mut(), DisableBracketedPaste).ok();
    execute!(terminal.backend_mut(), Clear(ClearType::All)).ok();
    disable_raw_mode().map_err(|e| e.to_string())?;

    Ok(())
}

/// Execution status for tracking git commit and nixos-rebuild progress
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutionStatus {
    Idle,
    GitCommitting,
    GitComplete,
    Rebuilding { phase: String, progress: Option<String> },
    Success,
    Failed { error: String, can_rollback: bool, previous_generation: Option<i32> },
}

/// Execution step - runs git commit and nixos-rebuild
pub struct ExecutionStep {
    pub status: ExecutionStatus,
    pub output_lines: Vec<String>,
    pub error_lines: Vec<String>,
}

impl ExecutionStep {
    pub fn new() -> Self {
        Self {
            status: ExecutionStatus::Idle,
            output_lines: Vec::new(),
            error_lines: Vec::new(),
        }
    }
}

impl WizardStep for ExecutionStep {
    fn title(&self) -> &'static str {
        "Apply Configuration"
    }
    
    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect) {
        let hostname = state.hostname.as_deref().unwrap_or("unknown");
        
        match &self.status {
            ExecutionStatus::Idle => {
                let text: Vec<String> = vec![
                    format!("Ready to apply configuration to '{}':", hostname),
                    "".to_string(),
                    "This will:".to_string(),
                    "  1. Commit generated configs to git".to_string(),
                    "  2. Run nixos-rebuild switch --flake".to_string(),
                    "".to_string(),
                    "Press ENTER to begin.".to_string(),
                    "Press ESC to go back.".to_string(),
                ];
                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::White))
                    .block(Block::new().title("Apply Configuration"));
                frame.render_widget(paragraph, area);
            }
            ExecutionStatus::GitCommitting => {
                let text: Vec<String> = vec![
                    "Committing configuration to git...".to_string(),
                    "".to_string(),
                    "Running: git add . && git commit".to_string(),
                ];
                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::new().title("Git Commit"));
                frame.render_widget(paragraph, area);
            }
            ExecutionStatus::GitComplete => {
                let text: Vec<String> = vec![
                    "✓ Git commit successful".to_string(),
                    "".to_string(),
                    "Starting NixOS rebuild...".to_string(),
                ];
                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::Green))
                    .block(Block::new().title("Git Complete"));
                frame.render_widget(paragraph, area);
            }
            ExecutionStatus::Rebuilding { phase, progress } => {
                let phase_str = match phase.as_str() {
                    "Downloading" => "📥 Downloading packages",
                    "Building" => "🔨 Building configuration",
                    "Activating" => "⚡ Activating configuration",
                    _ => &phase,
                };
                
                let progress_str = progress.as_ref().map(|s| s.as_str()).unwrap_or("");
                
                let text: Vec<String> = vec![
                    "Applying system configuration...".to_string(),
                    "".to_string(),
                    format!("Phase: {}", phase_str),
                    progress_str.to_string(),
                    "".to_string(),
                    "This may take several minutes.".to_string(),
                ];
                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::new().title("Rebuilding"));
                frame.render_widget(paragraph, area);
            }
            ExecutionStatus::Success => {
                let text: Vec<String> = vec![
                    "✓ Configuration applied successfully!".to_string(),
                    "".to_string(),
                    format!("System '{}' has been configured.", hostname),
                    "".to_string(),
                    "Press ENTER to continue.".to_string(),
                ];
                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::Green))
                    .block(Block::new().title("Success"));
                frame.render_widget(paragraph, area);
            }
            ExecutionStatus::Failed { error, can_rollback, previous_generation } => {
                let rollback_text = if *can_rollback {
                    format!("Press 'r' to rollback to generation {}.", previous_generation.unwrap_or(0))
                } else {
                    "Rollback not available (no previous generation).".to_string()
                };
                
                let text: Vec<String> = vec![
                    "✗ Configuration failed".to_string(),
                    "".to_string(),
                    format!("Error: {}", error),
                    "".to_string(),
                    rollback_text,
                    "".to_string(),
                    "Press 'c' to continue anyway (stay on current system).".to_string(),
                    "Press ESC to go back.".to_string(),
                ];
                let paragraph = Paragraph::new(text.join("\n"))
                    .style(Style::default().fg(Color::Red))
                    .block(Block::new().title("Failed"));
                frame.render_widget(paragraph, area);
            }
        }
    }
    
    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            match key.code {
                crossterm::event::KeyCode::Enter => {
                    if self.status == ExecutionStatus::Idle {
                        // Start execution
                        self.status = ExecutionStatus::GitCommitting;
                        // TODO: Execute git commit then nixos-rebuild
                    } else if self.status == ExecutionStatus::Success {
                        state.mark_step_complete();
                        state.go_next()?; // Go to Completion step
                    }
                }
                crossterm::event::KeyCode::Esc => {
                    if self.status == ExecutionStatus::Idle {
                        state.go_back();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn validate(&self, _state: &WizardState) -> Result<(), String> {
        Ok(())
    }
    
    fn is_complete(&self, state: &WizardState) -> bool {
        self.status == ExecutionStatus::Success && state.is_current_step_complete()
    }
}

/// Completion step - shows success message and next steps
pub struct CompletionStep;

impl CompletionStep {
    pub fn new() -> Self {
        Self
    }
}

impl WizardStep for CompletionStep {
    fn title(&self) -> &'static str {
        "Complete"
    }
    
    fn render(&self, frame: &mut Frame, state: &WizardState, area: Rect) {
        let hostname = state.hostname.as_deref().unwrap_or("unknown");
        let username = state.username.as_deref().unwrap_or("unknown");
        
        let text: Vec<String> = vec![
            "✓ Installation Complete!".to_string(),
            "".to_string(),
            format!("Your NixOS system '{}' has been configured.", hostname),
            format!("User '{}' has been set up with Niri and Noctalia.", username),
            "".to_string(),
            "Next Steps:".to_string(),
            "1. Log out and log back in (or restart)".to_string(),
            "2. Select 'Niri' from the display manager".to_string(),
            "3. Your Noctalia shell will be ready!".to_string(),
            "".to_string(),
            "Configuration is tracked in git at:".to_string(),
            "  ./milos-output".to_string(),
            "".to_string(),
            "Press Enter to exit the installer.".to_string(),
        ];
        
        let paragraph = Paragraph::new(text.join("\n"))
            .style(Style::default().fg(Color::Green))
            .block(Block::new().title("Installation Complete"));
        
        frame.render_widget(paragraph, area);
    }
    
    fn handle_input(&mut self, event: Event, state: &mut WizardState) -> Result<(), String> {
        if let Event::Key(key) = event {
            if key.code == crossterm::event::KeyCode::Enter {
                state.mark_step_complete();
                // Wizard loop will exit since all steps complete
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
