//! This module implements a simple modal editor state machine, inspired by Vim.
//! It defines different modes (Normal, Insert, Visual) and how the editor should
//! respond to various inputs in each mode.

use rem_rpc::RpcRequest;

/// Represents the different modes of the editor.
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

/// Represents the actions that the editor should take in response to user input.
pub enum EditorAction {
    /// Change the current mode of the editor.
    ChangeMode(Mode),
    /// Insert the given text at the current cursor position.
    InsertText(String),
    /// Delete the character before the cursor.
    DeleteBackwards,
    /// Move the cursor left.
    MoveLeft,
    /// Move the cursor right.
    MoveRight,
    /// Quit the editor.
    Quit,
    /// No action needed.
    DoNothing,
}

/// Represents the state of the editor, including the current mode and any other
pub struct StateMachine {
    pub current_mode: Mode,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current_mode: Mode::Normal,
        }
    }

    /// Process an incoming RPC request and determine the appropriate editor action
    ///
    /// # Arguments
    /// * `req` - The incoming RPC request to process
    ///
    /// # Returns
    /// An `EditorAction` that represents the action the editor should take in response to the
    pub fn process_input(&mut self, req: RpcRequest) -> EditorAction {
        match self.current_mode {
            Mode::Normal => self.process_normal(req),
            Mode::Insert => self.process_insert(req),
            Mode::Visual => self.process_visual(req),
        }
    }

    /// Process input in Normal mode
    ///
    /// # Arguments
    /// * `req` - The incoming RPC request to process
    ///
    /// # Returns
    /// An `EditorAction` that represents the action the editor should take in response to the input
    pub fn process_normal(&mut self, req: RpcRequest) -> EditorAction {
        match req {
            RpcRequest::Char('i') => {
                self.current_mode = Mode::Insert;
                EditorAction::ChangeMode(Mode::Insert)
            }
            RpcRequest::Char('v') => {
                self.current_mode = Mode::Visual;
                EditorAction::ChangeMode(Mode::Visual)
            }
            RpcRequest::Char('h') => EditorAction::MoveLeft,
            RpcRequest::Char('l') => EditorAction::MoveRight,
            RpcRequest::Quit => EditorAction::Quit,
            _ => EditorAction::DoNothing,
        }
    }

    /// Process input in Insert mode
    ///
    /// # Arguments
    /// * `req` - The incoming RPC request to process
    ///
    /// # Returns
    /// An `EditorAction` that represents the action the editor should take in response to the input
    pub fn process_insert(&mut self, req: RpcRequest) -> EditorAction {
        match req {
            RpcRequest::Esc => {
                self.current_mode = Mode::Normal;
                EditorAction::ChangeMode(Mode::Normal)
            }
            RpcRequest::Char(c) => EditorAction::InsertText(c.to_string()),
            RpcRequest::Enter => EditorAction::InsertText('\n'.to_string()),
            RpcRequest::Backspace => EditorAction::DeleteBackwards,
            RpcRequest::Quit => EditorAction::Quit,
            _ => EditorAction::DoNothing,
        }
    }

    /// Process input in Visual mode
    ///
    /// # Arguments
    /// * `req` - The incoming RPC request to process
    ///
    /// # Returns
    /// An `EditorAction` that represents the action the editor should take in response to the input
    pub fn process_visual(&mut self, req: RpcRequest) -> EditorAction {
        todo!()
    }
}
