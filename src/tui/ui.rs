// src/tui/ui.rs
//! UI rendering logic for the TUI

use crate::tui::app::TuiApp;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

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

/// Render the message list
fn render_message_list(f: &mut Frame, app: &TuiApp, area: Rect) {
    let messages = app.get_messages();

    // Create list items with color coding
    let items: Vec<ListItem> = messages
        .iter()
        .map(|msg| {
            let (style, text) = if msg.starts_with("[Danmu]") {
                (Style::default().fg(Color::Cyan), msg.as_str())
            } else if msg.starts_with("[Gift]") {
                (Style::default().fg(Color::Yellow), msg.as_str())
            } else if msg.starts_with("[Unsupported") {
                (Style::default().fg(Color::DarkGray), msg.as_str())
            } else if msg.starts_with("[System]") {
                (Style::default().fg(Color::Green), msg.as_str())
            } else {
                (Style::default(), msg.as_str())
            };

            ListItem::new(Line::from(Span::styled(text, style)))
        })
        .collect();

    // Calculate scroll state
    let total_messages = messages.len();
    let visible_height = area.height.saturating_sub(2) as usize; // Account for borders

    // Determine which messages to show based on scroll offset
    let start_index = if app.auto_scroll {
        // Auto-scroll mode: show latest messages
        total_messages.saturating_sub(visible_height)
    } else {
        // Manual scroll mode: show based on offset
        total_messages.saturating_sub(visible_height + app.scroll_offset)
    };

    let visible_items: Vec<ListItem> = items
        .into_iter()
        .skip(start_index)
        .take(visible_height)
        .collect();

    // Create title with scroll indicator
    let scroll_indicator = if app.auto_scroll {
        "🔽 Auto-scroll"
    } else {
        "⏸ Paused - Press ↓ to bottom for auto-scroll"
    };

    let title = format!(" Room {} | {} ", app.room_id, scroll_indicator);

    let list = List::new(visible_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::White)),
    );

    f.render_widget(list, area);
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
    use unicode_width::UnicodeWidthStr;
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
