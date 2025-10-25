// src/tui/event.rs
//! Event handling and main TUI loop

use crate::tui::app::TuiApp;
use crate::tui::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::time::Duration;

/// Run the TUI application
pub fn run_tui<F>(mut app: TuiApp, mut on_message: F) -> io::Result<()>
where
    F: FnMut(String),
{
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main event loop
    let result = run_app(&mut terminal, &mut app, &mut on_message);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Main application loop
fn run_app<B, F>(
    terminal: &mut Terminal<B>,
    app: &mut TuiApp,
    on_message: &mut F,
) -> io::Result<()>
where
    B: ratatui::backend::Backend,
    F: FnMut(String),
{
    loop {
        // Render UI
        terminal.draw(|f| ui::render(f, app))?;

        // Handle events with timeout to allow rendering at ~60 FPS
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Quit commands
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.quit();
                    }
                    KeyCode::Esc => {
                        app.quit();
                    }

                    // Input handling
                    KeyCode::Char(c) => {
                        app.enter_char(c);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Enter => {
                        let input = app.take_input();
                        if !input.is_empty() {
                            // Check for quit commands
                            if input == "/quit" || input == "/exit" {
                                app.quit();
                            } else {
                                // Send message via callback
                                on_message(input);
                            }
                        }
                    }

                    // Arrow keys for scrolling and cursor movement
                    KeyCode::Up => {
                        app.scroll_up(1);
                    }
                    KeyCode::Down => {
                        app.scroll_down(1);
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }

                    // Page keys for scrolling
                    KeyCode::PageUp => {
                        app.scroll_up(10);
                    }
                    KeyCode::PageDown => {
                        app.scroll_down(10);
                    }

                    // Home/End keys
                    KeyCode::Home if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Ctrl+Home: scroll to top of messages
                        app.scroll_up(usize::MAX);
                    }
                    KeyCode::End if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Ctrl+End: scroll to bottom and re-enable auto-scroll
                        app.scroll_to_bottom();
                    }
                    KeyCode::Home => {
                        // Home: move cursor to start of input
                        app.cursor_position = 0;
                    }
                    KeyCode::End => {
                        // End: move cursor to end of input
                        app.cursor_position = app.input.len();
                    }

                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
