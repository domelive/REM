//! Piece Table implementation for efficient text editing.
//! 
//! This implementation allows for efficient insertions and deletions by maintaining two buffers:
//! - Original Buffer: Contains the initial text.
//! - Add Buffer: Contains all the inserted text.
//!
//! The Piece Table maintains a list of pieces that reference either buffer, allowing for efficient
//! text manipulation without needing to copy large amounts of data.

/// Enum to identify which buffer a piece references.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferID {
    Original, // References the original buffer.
    Add,      // References the add buffer.
}

/// Struct representing a piece of text in the piece table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    /// Identifies which buffer this piece references (Original or Add).
    pub buffer_id: BufferID,
    /// The starting index in the referenced buffer where this piece begins.
    pub start_idx: usize,
    /// The length of the piece, indicating how many characters it spans in the referenced buffer.
    pub length:    usize,
}

/// The main struct for the Piece Table, containing the original buffer, add buffer, and the list
/// of pieces. This struct provides methods for inserting, deleting, and retrieving text
/// efficiently.
pub struct PieceTable {
    // The original text buffer, which remains unchanged after initialization.
    original_buffer: String, 
    // The buffer that accumulates all inserted text, allowing for efficient insertions without modifying the original buffer.
    add_buffer:      String, 
    // A list of pieces that reference either the original buffer or the add buffer, allowing for efficient text manipulation.
    pieces:          Vec<Piece>, 
}

impl PieceTable {
    pub fn new(text: String) -> Self {
        let length = text.len();

        Self { 
            original_buffer: text, 
            add_buffer: String::new(), 
            pieces: vec![Piece {
                buffer_id: BufferID::Original,
                start_idx: 0,
                length,
            }]
        }
    }

    /// Inserts text at the specified offset. This method will add the new text to the add buffer
    /// and update the pieces accordingly.
    ///
    /// # Arguments
    /// - `offset`: The position in the text where the new text should be inserted.
    /// - `text`: The text to be inserted.
    pub fn insert(&mut self, offset: usize, text: &str) {
        if text.is_empty() { return; }

        let new_text_start = self.add_buffer.len();
        self.add_buffer.push_str(text);

        let mut logical_offset = 0;
        let mut relative_offset = 0;
        let mut target_idx = 0;

        for (idx, piece) in self.pieces.iter().enumerate() {
            if logical_offset + piece.length >= offset {
                target_idx = idx;
                relative_offset = offset - logical_offset;
                break;
            }

            logical_offset += piece.length;
        }

        let target_piece = self.pieces[target_idx];

        let left_piece = Piece {
            buffer_id: target_piece.buffer_id,
            start_idx: target_piece.start_idx,
            length: relative_offset,
        };

        let middle_piece = Piece {
            buffer_id: BufferID::Add,
            start_idx: new_text_start,
            length: text.len(),
        };

        let right_piece = Piece {
            buffer_id: target_piece.buffer_id,
            start_idx: target_piece.start_idx + relative_offset,
            length: target_piece.length - relative_offset,
        };

        let mut new_pieces = Vec::new();
        if left_piece.length > 0 { new_pieces.push(left_piece); }
        new_pieces.push(middle_piece);
        if right_piece.length > 0 { new_pieces.push(right_piece); }

        self.pieces.splice(target_idx..=target_idx, new_pieces);
    }

    pub fn delete(&mut self, offset: usize, length: usize) {
        todo!()
    }

    pub fn get_text(&self) -> String {
        let total_length: usize = self.pieces.iter().map(|piece| piece.length).sum();

        let mut text = String::with_capacity(total_length);

        for piece in self.pieces.iter() {
            if piece.length == 0 {
                continue;
            }

            let end_idx = piece.start_idx + piece.length;

            match piece.buffer_id {
                BufferID::Original => {
                    text.push_str(&self.original_buffer[piece.start_idx..end_idx]);
                },
                BufferID::Add => {
                    text.push_str(&self.add_buffer[piece.start_idx..end_idx]);
                }
            }
        }

        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_insertion() {
        let mut pt = PieceTable::new("Hello World".to_string());
        
        pt.insert(5, " Beautiful");
        assert_eq!(pt.get_text(), "Hello Beautiful World", "L'inserimento a metà ha fallito");
    }

    #[test]
    fn test_edge_cases_insertion() {
        let mut pt = PieceTable::new("Code".to_string());

        pt.insert(0, "VS");
        assert_eq!(pt.get_text(), "VSCode");

        let len = pt.get_text().len();
        pt.insert(len, " is slow");
        assert_eq!(pt.get_text(), "VSCode is slow");
    }

    #[test]
    fn test_multiple_overlapping_inserts() {
        let mut pt = PieceTable::new("".to_string());
        
        pt.insert(0, "a");
        pt.insert(1, "c");
        
        pt.insert(1, "b"); 
        
        assert_eq!(pt.get_text(), "abc", "La gestione degli indici relativi ha fallito");
    }
}
