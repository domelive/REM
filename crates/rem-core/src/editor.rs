//! The `editor` module contains the implementation of the `Editor` struct, which is responsible
//! for managing the text buffer and processing requests from the UI. The `Editor` struct uses a
//! `PieceTable` to efficiently manage the text buffer, allowing for fast insertions and deletions.
//! The `Editor` struct also handles the cursor position and communicates with the UI through RPC
//! messages.

use crate::{piece_table::PieceTable, selection::Selection};
use rem_rpc::{RpcRequest, RpcResponse};
use std::sync::mpsc::{Receiver, Sender};

/// The `Editor` struct represents the state of the text editor, including the text buffer, cursor
/// position, and text selections. It provides methods for processing requests from the UI and
/// updating the editor state accordingly.
pub struct Editor {
    /// The `buffer` field is an instance of the `PieceTable` struct.
    buffer: PieceTable,
    /// The `selections` field is a vector of `Selection` structs, which represent the current text
    selections: Vec<Selection>,
    /// The `main_selection_idx` field is an index into the `selections` vector that indicates which selection is currently active or primary.
    main_selection_idx: usize,
}

impl Editor {
    pub fn new(initial_text: String) -> Self {
        Self {
            buffer: PieceTable::new(initial_text),
            selections: vec![Selection::point(0)],
            main_selection_idx: 0,
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
                RpcRequest::InsertNewline => {
                    // NOTE: For now we only support the main selection. In the future, we just iterate
                    // over all selections and apply the insertions to each of them.
                    let main_selection = &mut self.selections[self.main_selection_idx];
                    let (start, end) = main_selection.range();

                    // If the selection is not a point, we delete the selected text first.
                    if start != end {
                        self.buffer.delete(start, end);
                        main_selection.head = start;
                        main_selection.collapse();
                    }

                    self.buffer.insert(main_selection.head, "\n");

                    // TODO: We need to move the cursor to the beginning of the next line after
                    // inserting a newline.
                }

                // When an `InsertChar` request is received, the editor inserts the specified
                // character at the current cursor position in the text buffer and moves the cursor
                // to the right.
                RpcRequest::InsertChar(c) => {
                    // NOTE: For now we only support the main selection. In the future, we just iterate
                    // over all selections and apply the insertions to each of them.
                    let main_selection = &mut self.selections[self.main_selection_idx];
                    let (start, end) = main_selection.range();

                    // If the selection is not a point, we delete the selected text first.
                    if start != end {
                        self.buffer.delete(start, end);
                        main_selection.head = start;
                        main_selection.collapse();
                    }

                    self.buffer.insert(main_selection.head, &c.to_string());

                    // Move the cursor to the right after insertion.
                    main_selection.head += 1;
                    main_selection.collapse();
                }

                // When a `DeleteChar` request is received, the editor deletes the character at the
                // current cursor position in the text buffer and moves the cursor to the left if
                // it's not at the beginning of the buffer.
                RpcRequest::DeleteChar => {
                    // NOTE: For now we only support the main selection. In the future, we just iterate
                    // over all selections and apply the deletion to each of them.
                    let main_selection = &mut self.selections[self.main_selection_idx];
                    let (start, end) = main_selection.range();

                    // If the selection is not a point, we delete the selected text first.
                    if start != end {
                        self.buffer.delete(start, end - start);
                        main_selection.head = start;
                        main_selection.collapse();
                    } else if main_selection.head > 0 {
                        // Delete the character before the cursor and move the cursor to the left.
                        self.buffer.delete(main_selection.head - 1, 1);
                        main_selection.head -= 1;
                        main_selection.collapse();

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
    /// The `calculate_cursor_coords` method calculates the (x, y) coordinates of the cursor based
    /// on the current cursor position in the text buffer.
    ///
    /// Basically transforms the linear cursor position (which is an index into the text buffer)
    /// into 2D coordinates (x, y) where x is the column number and y is the line number.
    fn calculate_cursor_coords(&self) -> (usize, usize) {
        let text = self.buffer.get_text();
        
        // We take the minimum of the cursor position and the text length to avoid out-of-bounds
        // access.
        let offset = self.selections[self.main_selection_idx].head.min(text.len());
        // We take the substring of the text up to the cursor position and split it into lines to
        // calculate the line and column numbers.
        let prefix = &text[..offset];

        // We split the prefix into lines and count the number of lines to get the y coordinate,
        // and the length of the last line to get the x coordinate.
        let lines: Vec<&str> = prefix.split('\n').collect();

        // The y coordinate is the number of lines minus one (since line numbers are zero-based),
        // and the x coordinate is the length of the last line (which is the column number).
        let cursor_y = lines.len() - 1;
        let cursor_x = lines.last().map_or(0, |line| line.len());

        (cursor_x, cursor_y)
    }

    /// The `broadcast_state` method is responsible for sending the current state of the editor
    /// (the text buffer and the cursor position) to the UI. It constructs an `RpcResponse::Render`
    /// message containing the current text and cursor position, and sends it through the provided
    /// `Sender<RpcResponse>`.
    fn broadcast_state(&self, tx: &Sender<RpcResponse>) {
        let (cursor_x, cursor_y) = self.calculate_cursor_coords();

        let response = RpcResponse::Render {
            text: self.buffer.get_text(),
            cursor_x,
            cursor_y,
        };

        let _ = tx.send(response);
    }
}
