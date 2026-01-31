//! # MILOS Pixel Art Logo
//!
//! This module contains the pixel art logo for the MILOS installer.
//! The logo is defined as Rust code for portability.
//!
//! Design:
//! - 5 letters: M-I-L-O-S
//! - Neon green (#00FF41) with amber (#FFB000) glow
//! - Typewriter animation effect

use std::time::{Duration, Instant};
use ratatui::{
    widgets::canvas::{Canvas, Painter},
    layout::Rect,
    Frame,
};
use ratatui::backend::CrosstermBackend;
use std::io::Stdout;

/// Color scheme for the logo (Noctalia theme)
const NEON_GREEN: (u8, u8, u8) = (0x00, 0xFF, 0x65); // Matrix/screen green
const AMBER_GLOW: (u8, u8, u8) = (0xFF, 0xB0, 0x00); // Amber glow
const DARK_BG: (u8, u8, u8) = (0x10, 0x10, 0x10);    // Near black

/// Pixel representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pixel {
    Empty,
    Letter,
    Glow,
}

/// Letter dimensions
const LETTER_WIDTH: usize = 6;
const LETTER_HEIGHT: usize = 7;
const LETTER_SPACING: usize = 2;
const TOTAL_WIDTH: usize = (LETTER_WIDTH + LETTER_SPACING) * 5 - LETTER_SPACING;
const TOTAL_HEIGHT: usize = LETTER_HEIGHT;

/// Letter M - 6x7 pixel art
const M_PIXELS: [[Pixel; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Letter, Pixel::Empty, Pixel::Letter, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Letter, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
];

/// Letter I - 6x7 pixel art
const I_PIXELS: [[Pixel; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [Pixel::Empty, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Empty],
    [Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Empty, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Empty],
];

/// Letter L - 6x7 pixel art
const L_PIXELS: [[Pixel; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Empty],
];

/// Letter O - 6x7 pixel art
const O_PIXELS: [[Pixel; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [Pixel::Empty, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Empty, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Empty, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Empty, Pixel::Empty],
];

/// Letter S - 6x7 pixel art
const S_PIXELS: [[Pixel; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [Pixel::Empty, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Letter, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty],
    [Pixel::Empty, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Empty, Pixel::Empty],
    [Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Empty, Pixel::Letter, Pixel::Empty],
    [Pixel::Empty, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Letter, Pixel::Empty],
];

/// All letters as a const array
const LETTERS: [&[[Pixel; LETTER_WIDTH]; LETTER_HEIGHT]; 5] = [
    &M_PIXELS,
    &I_PIXELS,
    &L_PIXELS,
    &O_PIXELS,
    &S_PIXELS,
];

/// Get all letters' pixel data
pub fn get_letters() -> [&[[Pixel; LETTER_WIDTH]; LETTER_HEIGHT]; 5] {
    LETTERS
}

/// Animation state for the logo
#[derive(Clone, Debug)]
pub struct LogoAnimation {
    /// How many letters to show (0-5)
    pub letters_shown: usize,
    /// Position within current letter (0-letter_width * letter_height)
    pub letter_progress: usize,
    /// Whether cursor is visible
    pub cursor_visible: bool,
    /// Last time animation progressed
    pub last_update: Instant,
    /// Whether animation is complete
    pub complete: bool,
    /// Letter-by-letter mode (vs pixel-by-pixel)
    pub letter_mode: bool,
    /// Animation speed (per letter or per pixel)
    pub letter_speed: Duration,
    /// Blink rate for cursor
    pub blink_rate: Duration,
}

impl LogoAnimation {
    /// Create new animation with defaults
    pub fn new() -> Self {
        Self {
            letters_shown: 0,
            letter_progress: 0,
            cursor_visible: true,
            last_update: Instant::now(),
            complete: false,
            letter_mode: true, // Letter-by-letter typewriter effect
            letter_speed: Duration::from_millis(150), // 150ms per letter
            blink_rate: Duration::from_millis(500),
        }
    }

    /// Update animation state based on elapsed time
    pub fn update(&mut self) {
        if self.complete {
            // Just blink cursor
            if self.last_update.elapsed() >= self.blink_rate {
                self.cursor_visible = !self.cursor_visible;
                self.last_update = Instant::now();
            }
            return;
        }

        if self.letter_mode {
            // Letter-by-letter mode
            if self.last_update.elapsed() >= self.letter_speed {
                if self.letters_shown < 5 {
                    self.letters_shown += 1;
                } else {
                    self.complete = true;
                }
                self.last_update = Instant::now();
            }
        }
    }

    /// Check if animation is still running
    pub fn is_animating(&self) -> bool {
        !self.complete
    }

    /// Reset animation to start
    pub fn reset(&mut self) {
        self.letters_shown = 0;
        self.letter_progress = 0;
        self.cursor_visible = true;
        self.last_update = Instant::now();
        self.complete = false;
    }
}

impl Default for LogoAnimation {
    fn default() -> Self {
        Self::new()
    }
}

/// Render the logo with animation
pub fn render_logo(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    area: Rect,
    animation: &LogoAnimation,
) {
    let letters = get_letters();
    let total_width = (LETTER_WIDTH + LETTER_SPACING) * 5 - LETTER_SPACING;

    // Calculate centering offset
    let start_x = (area.width as usize / 2).saturating_sub(total_width / 2);
    let start_y = (area.height as usize / 2).saturating_sub(TOTAL_HEIGHT / 2);

    let x = start_x as f64;
    let y = start_y as f64;

    // Create canvas for pixel art
    let canvas = Canvas::default()
        .x_bounds(0.0, area.width as f64)
        .y_bounds(0.0, area.height as f64)
        .paint(move |ctx| {
            // Draw each letter up to animation progress
            for (letter_idx, &letter_pixels) in letters.iter().enumerate() {
                if letter_idx >= animation.letters_shown {
                    break;
                }

                let letter_start_x = x + (letter_idx * (LETTER_WIDTH + LETTER_SPACING)) as f64;

                for (row, pixel_row) in letter_pixels.iter().enumerate() {
                    for (col, pixel) in pixel_row.iter() {
                        let pixel_x = letter_start_x + col as f64;
                        let pixel_y = y + row as f64;

                        match pixel {
                            Pixel::Letter => {
                                // Draw letter pixel in neon green
                                ctx.print(
                                    pixel_x,
                                    pixel_y,
                                    "█",
                                    NEON_GLOW.0,
                                    NEON_GLOW.1,
                                    NEON_GLOW.2,
                                );
                            }
                            Pixel::Glow => {
                                // Draw glow pixel in amber
                                ctx.print(
                                    pixel_x,
                                    pixel_y,
                                    "▒",
                                    AMBER_GLOW.0,
                                    AMBER_GLOW.1,
                                    AMBER_GLOW.2,
                                );
                            }
                            Pixel::Empty => {}
                        }
                    }
                }
            }

            // Draw blinking cursor after last letter
            if animation.cursor_visible {
                let cursor_x = x + (animation.letters_shown * (LETTER_WIDTH + LETTER_SPACING)) as f64 - 1.0;
                let cursor_y = y + TOTAL_HEIGHT as f64 / 2.0;
                ctx.print(
                    cursor_x,
                    cursor_y,
                    "█",
                    NEON_GLOW.0,
                    NEON_GLOW.1,
                    NEON_GLOW.2,
                );
            }
        });

    frame.render_widget(canvas, area);
}

/// Alternative: Simple ASCII logo for terminals that don't support Canvas well
pub fn render_logo_ascii(animation: &LogoAnimation) -> Vec<String> {
    let letters = get_letters();
    let mut lines: Vec<String> = Vec::with_capacity(TOTAL_HEIGHT);

    for row in 0..TOTAL_HEIGHT {
        let mut line = String::new();

        for (letter_idx, &letter_pixels) in letters.iter().enumerate() {
            if letter_idx >= animation.letters_shown {
                break;
            }

            for col in 0..LETTER_WIDTH {
                match letter_pixels[row][col] {
                    Pixel::Letter => line.push('█'),
                    Pixel::Glow => line.push('▒'),
                    Pixel::Empty => line.push(' '),
                }
            }
            if letter_idx < 4 {
                line.push(' ');
                line.push(' ');
            }
        }

        if animation.cursor_visible && row == TOTAL_HEIGHT / 2 {
            line.push('█');
        }

        lines.push(line);
    }

    lines
}

/// Complete logo rendering with subtitle
pub fn render_full_logo(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    area: Rect,
    animation: &LogoAnimation,
) {
    // Logo area (top 70% of space)
    let logo_area = Rect::new(
        area.x,
        area.y,
        area.width,
        area.height.saturating_sub(4),
    );

    render_logo(frame, logo_area, animation);

    // Subtitle area (bottom)
    if animation.complete {
        // Show "Press Enter to begin" when animation is done
        // This would be rendered by the WelcomeStep
    }
}
