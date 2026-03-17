//! This module contains the main entry point for the REM editor TUI application. It sets up the
//! terminal interface using `crossterm` and `ratatui`, and manages communication between the UI
//! and the core editor logic through channels.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
};
use std::{io, sync::mpsc, thread, time::Duration};

use rem_core::editor::Editor;
use rem_rpc::{RpcRequest, RpcResponse};

fn main() -> Result<(), io::Error> {
    // Set up the terminal in raw mode and enter the alternate screen for the TUI application.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up channels for communication between the UI and the core editor logic
    let (ui_tx, core_rx) = mpsc::channel::<RpcRequest>(); // UI sends requests to the core
    let (core_tx, ui_rx) = mpsc::channel::<RpcResponse>(); // Core sends responses to the UI

    // Spawn a thread to run the core editor logic separately from the UI thread.
    thread::spawn(move || {
        let mut editor = Editor::new("".to_string());
        editor.run(core_rx, core_tx);
    });

    let mut render_text = String::new();
    let mut is_running = true;

    let mut current_cursor_x = 0;
    let mut current_cursor_y = 0;

    while is_running {
        // Check for any responses from the core editor logic and update the UI state accordingly.
        while let Ok(response) = ui_rx.try_recv() {
            match response {
                RpcResponse::Render { text, cursor_x, cursor_y } => {
                    render_text = text;
                    current_cursor_x = cursor_x;
                    current_cursor_y = cursor_y;
                }
                RpcResponse::Shutdown => is_running = false,
            }
        }

        if !is_running {
            break;
        }

        // Draw the UI using the latest render text from the core editor logic.
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title(" REM Editor ").borders(Borders::ALL);

            let paragraph = Paragraph::new(render_text.as_str()).block(block);
            f.render_widget(paragraph, size);

            f.set_cursor(
                (current_cursor_x + 1) as u16,
                (current_cursor_y + 1) as u16,
            );
        })?;

        // Poll for user input events with a timeout to allow for UI updates.
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        let _ = ui_tx.send(RpcRequest::Quit);
                    }
                    KeyCode::Backspace => {
                        let _ = ui_tx.send(RpcRequest::DeleteChar);
                    }
                    KeyCode::Enter => {
                        let _ = ui_tx.send(RpcRequest::InsertChar('\n'));
                    }
                    KeyCode::Char(c) => {
                        let _ = ui_tx.send(RpcRequest::InsertChar(c));
                    }
                    _ => {}
                }
            }
        }
    }

    // Clean up the terminal state before exiting.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
