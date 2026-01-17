// src/tui/ui.rs
//! UI rendering logic for the TUI

use crate::tui::app::TuiApp;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use unicode_width::UnicodeWidthStr;

/// Render the TUI
pub fn render(f: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(90), // Message list
            Constraint::Percentage(10), // Input box
        ])
        .split(f.area());

    render_message_list(f, app, chunks[0]);
    render_input_box(f, app, chunks[1]);
}

/// Render the message list with text wrapping support
fn render_message_list(f: &mut Frame, app: &TuiApp, area: Rect) {
    let messages = app.get_messages();
    let inner_width = area.width.saturating_sub(2) as usize; // Account for borders
    let visible_height = area.height.saturating_sub(2) as usize; // Account for borders

    // Create wrapped lines with styles
    let mut all_lines: Vec<Line> = Vec::new();

    for msg in messages.iter() {
        let style = get_message_style(msg);
        let wrapped = wrap_text(msg, inner_width);
        for line_text in wrapped {
            all_lines.push(Line::from(Span::styled(line_text, style)));
        }
    }

    // Calculate which lines to show based on scroll
    let total_lines = all_lines.len();
    let start_line = if app.auto_scroll {
        // Auto-scroll mode: show latest lines
        total_lines.saturating_sub(visible_height)
    } else {
        // Manual scroll mode: show based on offset
        total_lines.saturating_sub(visible_height + app.scroll_offset)
    };

    let visible_lines: Vec<Line> = all_lines
        .into_iter()
        .skip(start_line)
        .take(visible_height)
        .collect();

    // Create title with scroll indicator
    let scroll_indicator = if app.auto_scroll {
        "🔽 Auto-scroll"
    } else {
        "⏸ Paused - Press ↑↓ to scroll"
    };

    let title = format!(" Room {} | {} ", app.room_id, scroll_indicator);

    let paragraph = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::White)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

/// Get the style for a message based on its prefix
fn get_message_style(msg: &str) -> Style {
    if msg.starts_with("[Danmu]") {
        Style::default().fg(Color::Cyan)
    } else if msg.starts_with("[Gift]") {
        Style::default().fg(Color::Yellow)
    } else if msg.starts_with("[Raw]") {
        Style::default().fg(Color::Magenta)
    } else if msg.starts_with("[Unsupported") {
        Style::default().fg(Color::DarkGray)
    } else if msg.starts_with("[System]") {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    }
}

/// Wrap text to fit within the specified width, respecting Unicode character widths
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for ch in text.chars() {
        let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);

        if current_width + char_width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
        }

        current_line.push(ch);
        current_width += char_width;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

/// Render the input box
fn render_input_box(f: &mut Frame, app: &TuiApp, area: Rect) {
    let input_text = format!("> {}", app.input);

    let paragraph = Paragraph::new(input_text.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Input (Enter: send | ↑↓: scroll | ←→: move cursor | Ctrl+C: exit) ")
                .border_style(Style::default().fg(Color::Green)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);

    // Set cursor position
    // Calculate display width up to cursor position (handles multi-byte characters)
    let text_before_cursor: String = app.input.chars().take(app.cursor_position).collect();
    let display_width = text_before_cursor.width();

    // area.x + 1 (left border) + 2 ("> " prefix) + display_width
    let cursor_x = area.x + 1 + 2 + display_width as u16;
    let cursor_y = area.y + 1; // area.y + 1 (top border)

    // Make sure cursor is within bounds
    if cursor_x < area.x + area.width.saturating_sub(1) {
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
