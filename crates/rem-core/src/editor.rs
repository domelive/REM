//! The `editor` module contains the implementation of the `Editor` struct, which is responsible
//! for managing the text buffer and processing requests from the UI. The `Editor` struct uses a
//! `PieceTable` to efficiently manage the text buffer, allowing for fast insertions and deletions.
//! The `Editor` struct also handles the cursor position and communicates with the UI through RPC
//! messages.

use crate::piece_table::PieceTable;
use rem_rpc::{RpcRequest, RpcResponse};
use std::sync::mpsc::{Receiver, Sender};

/// The `Editor` struct represents the core of the text editor. It contains the text buffer and the
/// cursor position. The `Editor` struct is responsible for processing the requests received from
/// the UI and updating the text buffer accordingly. It also sends responses back to the UI to
/// update the rendered text and cursor position.
pub struct Editor {
    /// The `buffer` field is an instance of the `PieceTable` struct.
    buffer: PieceTable,
    /// The `cursor_offset` field represents the current position of the cursor in the text buffer.
    cursor_offset: usize,
}

impl Editor {
    pub fn new(initial_text: String) -> Self {
        Self {
            buffer: PieceTable::new(initial_text),
            cursor_offset: 0,
        }
    }

    /// The `run` method is the main loop of the editor. It listens for incoming `RpcRequest`
    /// messages from the UI, processes them, and sends `RpcResponse` messages back to the UI to
    /// update the rendered text and cursor position. The loop continues until a `Quit` request is
    /// received, at which point it sends a `Shutdown` response and breaks the loop.
    ///
    /// # Arguments
    /// - `rx`: A `Receiver<RpcRequest>` that receives requests from the UI.
    /// - `tx`: A `Sender<RpcResponse>` that sends responses back to the UI.
    pub fn run(&mut self, rx: Receiver<RpcRequest>, tx: Sender<RpcResponse>) {
        self.broadcast_state(&tx);

        for req in rx {
            match req {
                // When an `InsertChar` request is received, the editor inserts the specified
                // character at the current cursor position in the text buffer and moves the cursor
                // to the right.
                RpcRequest::InsertChar(c) => {
                    self.buffer.insert(self.cursor_offset, &c.to_string());
                    self.cursor_offset += 1;
                }

                // When a `DeleteChar` request is received, the editor deletes the character at the
                // current cursor position in the text buffer and moves the cursor to the left if
                // it's not at the beginning of the buffer.
                RpcRequest::DeleteChar => {
                    if self.cursor_offset > 0 {
                        self.buffer.delete(self.cursor_offset - 1, 1);
                        self.cursor_offset -= 1;
                    }
                }

                // When a `MoveCursor` request is received, the editor moves the cursor in the
                // specified direction, ensuring that the cursor does not go out of bounds of the
                // text buffer.
                RpcRequest::Quit => {
                    let _ = tx.send(RpcResponse::Shutdown);
                    break;
                }

                _ => {}
            }

            self.broadcast_state(&tx);
        }
    }
}

impl Editor {
    /// The `broadcast_state` method is responsible for sending the current state of the editor
    /// (the text buffer and the cursor position) to the UI. It constructs an `RpcResponse::Render`
    /// message containing the current text and cursor position, and sends it through the provided
    /// `Sender<RpcResponse>`.
    fn broadcast_state(&self, tx: &Sender<RpcResponse>) {
        let response = RpcResponse::Render {
            text: self.buffer.get_text(),
            cursor_x: self.cursor_offset,
            cursor_y: 0,
        };
        let _ = tx.send(response);
    }
}
