//! This crate defines the data structures for the RPC communication between the editor and the UI.
//!
//! The editor and the UI will use these data structures to communicate with each other. The editor
//! will send `RpcResponse` messages to the UI, and the UI will send `RpcRequest` messages to the
//! editor.

use serde::{Deserialize, Serialize};

/// The `Direction` enum represents the possible directions that the cursor can move in.
/// This enum is used in the `MoveCursor` variant of the `RpcRequest` enum to specify the direction
/// in which the cursor should move.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// The `RpcRequest` enum represents the different types of requests that the UI can send to the
/// editor. Each variant of the enum corresponds to a different type of request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcRequest {
    /// The `InsertNewline` variant represents a request to insert a newline character at the current cursor position.
    InsertNewline,
    /// The `InsertChar` variant represents a request to insert a character at the current cursor position.
    InsertChar(char),
    /// The `DeleteChar` variant represents a request to delete the character at the current cursor position.
    DeleteChar,
    /// The `MoveCursor` variant represents a request to move the cursor in a specific direction.
    MoveCursor(Direction),
    /// The `Quit` variant represents a request to quit the editor.
    Quit,
}

/// The `RpcResponse` enum represents the different types of responses that the editor can send to
/// the UI. Each variant of the enum corresponds to a different type of response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcResponse {
    /// The `Render` variant represents a response that contains the text to be rendered on the UI,
    /// as well as the cursor position.
    Render {
        text: String,
        cursor_x: usize,
        cursor_y: usize,
    },
    /// The `Error` variant represents a response that contains an error message.
    Shutdown,
}
