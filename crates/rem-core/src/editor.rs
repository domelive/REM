//! The `editor` module contains the implementation of the `Editor` struct, which is responsible
//! for managing the text buffer and processing requests from the UI. The `Editor` struct uses a
//! `PieceTable` to efficiently manage the text buffer, allowing for fast insertions and deletions.
//! The `Editor` struct also handles the cursor position and communicates with the UI through RPC
//! messages.

use crate::{EditorAction, Mode, StateMachine, piece_table::PieceTable, selection::Selection};
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
    fsm: StateMachine,
}

impl Editor {
    pub fn new(initial_text: String) -> Self {
        Self {
            buffer: PieceTable::new(initial_text),
            selections: vec![Selection::point(0)],
            main_selection_idx: 0,
            fsm: StateMachine::new(),
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
            let action = self.fsm.process_input(req.clone());

            match action {
                EditorAction::Quit => {
                    let _ = tx.send(RpcResponse::Shutdown);
                    break;
                }
                EditorAction::ChangeMode(_) => {}
                EditorAction::InsertText(text) => self.insert_text(&text),
                EditorAction::DeleteBackwards => self.delete_backwards(),
                EditorAction::MoveLeft => {
                    let sel = &mut self.selections[self.main_selection_idx];
                    if sel.head > 0 {
                        sel.head -= 1;
                        sel.anchor = sel.head;
                    }
                }
                EditorAction::MoveRight => {
                    let len = self.buffer.get_text().len();
                    let sel = &mut self.selections[self.main_selection_idx];
                    if sel.head < len {
                        sel.head += 1;
                        sel.anchor = sel.head;
                    }
                }
                EditorAction::DoNothing => {}
            }

            self.broadcast_state(&tx);
        }
    }
}

impl Editor {
    /// The `insert_text` method handles the insertion of text at the current cursor position.
    ///
    /// If there is a selection (i.e., the anchor and head are at different positions), it first deletes the
    /// selected text before inserting the new text.
    /// After inserting the text, it updates the cursor position to be after the newly inserted text.
    ///
    /// # Arguments
    /// - `text`: The text to be inserted at the current cursor position.
    fn insert_text(&mut self, text: &str) {
        let sel = &mut self.selections[self.main_selection_idx];
        let start = sel.min();
        let end = sel.max();

        if start != end {
            self.buffer.delete(start, end - start);
            sel.anchor = start;
            sel.head = start;
        }

        self.buffer.insert(sel.head, text);

        sel.head += text.len();
        sel.anchor = sel.head;
    }

    /// The `delete_backwards` method handles the deletion of text when the user presses the backspace key.
    ///
    /// If there is a selection (i.e., the anchor and head are at different positions), it deletes the selected text.
    ///
    /// If there is no selection and the cursor is not at the beginning of the text,
    /// it deletes the character immediately before the cursor and moves the cursor back by one position.
    fn delete_backwards(&mut self) {
        let sel = &mut self.selections[self.main_selection_idx];
        let start = sel.min();
        let end = sel.max();

        if start != end {
            self.buffer.delete(start, end - start);
            sel.anchor = start;
            sel.head = start;
        } else if sel.head > 0 {
            self.buffer.delete(sel.head - 1, 1);
            sel.head -= 1;
            sel.anchor = sel.head;
        }
    }

    /// The `calculate_cursor_coords` method calculates the (x, y) coordinates of the cursor based
    /// on the current cursor position in the text buffer.
    ///
    /// Basically transforms the linear cursor position (which is an index into the text buffer)
    /// into 2D coordinates (x, y) where x is the column number and y is the line number.
    fn calculate_cursor_coords(&self) -> (usize, usize) {
        let text = self.buffer.get_text();

        // We take the minimum of the cursor position and the text length to avoid out-of-bounds
        // access.
        let offset = self.selections[self.main_selection_idx]
            .head
            .min(text.len());
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

        let mode_name = match self.fsm.current_mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
        }
        .to_string();

        let response = RpcResponse::Render {
            text: self.buffer.get_text(),
            cursor_x,
            cursor_y,
            mode_name,
        };

        let _ = tx.send(response);
    }
}
