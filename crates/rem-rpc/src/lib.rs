//! This crate defines the data structures for the RPC communication between the editor and the UI.
//!
//! The editor and the UI will use these data structures to communicate with each other. The editor
//! will send `RpcResponse` messages to the UI, and the UI will send `RpcRequest` messages to the
//! editor.

use serde::{Deserialize, Serialize};

/// The `RpcRequest` enum represents the different types of requests that the UI can send to the
/// editor. Each variant of the enum corresponds to a different type of request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcRequest {
    /// The `Char` variant represents a request to insert a character at the current cursor position.
    Char(char),
    /// The `Enter` variant represents a request to insert a newline character at the current cursor position.
    Enter,
    /// The `Backspace` variant represents a request to delete the character before the cursor.
    Backspace,
    /// The `Esc` variant represents a request to exit the current mode and return to normal mode.
    Esc,
    /// The `Quit` variant represents a request to quit the editor.
    Quit,
}

/// The `RpcResponse` enum represents the different types of responses that the editor can send to
/// the UI. Each variant of the enum corresponds to a different type of response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcResponse {
    /// The `Render` variant represents a response that contains the text to be rendered on the UI,
    /// as well as the cursor position and the name of the current mode.
    /// The UI will use this information to update the display accordingly.
    Render {
        text: String,
        cursor_x: usize,
        cursor_y: usize,
        mode_name: String,
    },
    /// The `Error` variant represents a response that contains an error message.
    Shutdown,
}
