//! This module defines the `Selection` struct, which represents a text selection in a text editor.
//! It consists of an anchor and a head, which can be used to determine the range of the selection.

/// Represents a text selection in a text editor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Selection {
    /// The anchor is the fixed point of the selection.
    pub anchor: usize,
    /// The head is the movable point of the selection.
    pub head: usize,
}

impl Selection {
    /// Creates a new selection with the given anchor and head positions.
    ///
    /// # Arguments
    /// * `anchor` - The position of the anchor.
    /// * `head` - The position of the head.
    pub fn new(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    /// Creates a new selection that is a single point at the given position.
    ///
    /// # Arguments
    /// * `position` - The position of the point selection.
    pub fn point(position: usize) -> Self {
        Self { anchor: position, head: position }
    }

    /// Returns the range of the selection as a tuple (start, end).
    pub fn range(&self) -> (usize, usize) {
        if self.anchor < self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    /// Collapses the selection to a single point at the head position.
    pub fn collapse(&mut self) {
        self.anchor = self.head;
    }

    /// Returns the minimum of the anchor and head positions, which is the start of the selection.
    pub fn min(&self) -> usize {
        self.anchor.min(self.head)
    }
    
    /// Returns the maximum of the anchor and head positions, which is the end of the selection.
    pub fn max(&self) -> usize {
        self.anchor.max(self.head)
    }
}
