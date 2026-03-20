//! This module contains the main entry point for the REM editor TUI application. It sets up the
//! terminal interface using `crossterm` and `ratatui`, and manages communication between the UI
//! and the core editor logic through channels.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
};

use rem_core::editor::Editor;
use rem_rpc::{RpcRequest, RpcResponse};
use std::{io, sync::mpsc, thread, time::Duration};

fn main() -> Result<(), io::Error> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up communication channels
    let (ui_tx, core_rx) = mpsc::channel::<RpcRequest>();
    let (core_tx, ui_rx) = mpsc::channel::<RpcResponse>();

    // Spawn the editor thread
    thread::spawn(move || {
        let mut editor = Editor::new("".to_string());
        editor.run(core_rx, core_tx);
    });

    // Variables to hold the current state of the UI
    let mut render_text = String::new();
    let mut current_cursor_x = 0;
    let mut current_cursor_y = 0;
    let mut current_mode = String::new();
    let mut is_running = true;

    while is_running {
        // First we check for any incoming responses from the editor thread and update our UI state accordingly
        while let Ok(response) = ui_rx.try_recv() {
            match response {
                RpcResponse::Render {
                    text,
                    cursor_x,
                    cursor_y,
                    mode_name,
                } => {
                    render_text = text;
                    current_cursor_x = cursor_x;
                    current_cursor_y = cursor_y;
                    current_mode = mode_name;
                }
                RpcResponse::Shutdown => is_running = false,
            }
        }

        if !is_running {
            break;
        }

        // Then we render the UI based on the current state
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title(format!(" REM Editor | -- {} -- ", current_mode))
                .borders(Borders::ALL);

            let paragraph = Paragraph::new(render_text.as_str()).block(block);
            f.render_widget(paragraph, size);

            f.set_cursor((current_cursor_x + 1) as u16, (current_cursor_y + 1) as u16);
        })?;

        // Finally, we poll for user input and send any relevant requests to the editor thread
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        let _ = ui_tx.send(RpcRequest::Esc);
                    }
                    KeyCode::Backspace => {
                        let _ = ui_tx.send(RpcRequest::Backspace);
                    }
                    KeyCode::Enter => {
                        let _ = ui_tx.send(RpcRequest::Enter);
                    }

                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        let _ = ui_tx.send(RpcRequest::Quit);
                    }

                    KeyCode::Char(c) => {
                        let _ = ui_tx.send(RpcRequest::Char(c));
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
