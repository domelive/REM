//! Piece Table implementation for efficient text editing.
//!
//! This implementation allows for efficient insertions and deletions by maintaining two buffers:
//! - Original Buffer: Contains the initial text.
//! - Add Buffer: Contains all the inserted text.
//!
//! The Piece Table maintains a list of pieces that reference either buffer, allowing for efficient
//! text manipulation without needing to copy large amounts of data.

// ================================================================
// Data structures for the Piece Table implementation.
// ================================================================

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
    pub length: usize,
}

/// The main struct for the Piece Table, containing the original buffer, add buffer, and the list
/// of pieces. This struct provides methods for inserting, deleting, and retrieving text
/// efficiently.
pub struct PieceTable {
    // The original text buffer, which remains unchanged after initialization.
    original_buffer: String,
    // The buffer that accumulates all inserted text, allowing for efficient insertions without modifying the original buffer.
    add_buffer: String,
    // A list of pieces that reference either the original buffer or the add buffer, allowing for efficient text manipulation.
    pieces: Vec<Piece>,
}

// ================================================================
// Public methods for the PieceTable struct.
// ================================================================
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
            }],
        }
    }

    /// Inserts text at the specified offset. This method will add the new text to the add buffer
    /// and update the pieces accordingly.
    ///
    /// # Arguments
    /// - `offset`: The position in the text where the new text should be inserted.
    /// - `text`: The text to be inserted.
    pub fn insert(&mut self, offset: usize, text: &str) {
        if text.is_empty() {
            return;
        }

        // Add the new text to the add buffer and keep track of its length before the addition to
        // calculate the starting index for the new piece.
        let text_len_before_add = self.add_buffer.len();
        self.add_buffer.push_str(text);

        // Find the target piece and the relative offset within that piece where the new text will
        // be inserted. This will help us determine how to split the existing piece to accommodate
        // the new text.
        let (target_idx, relative_offset) = self.find_piece_idx_from_offset(offset);
        let target_piece = self.pieces[target_idx];

        // Split the target piece into three pieces: left, middle, and right.
        let (left_piece, middle_piece, right_piece) = self.split_piece(
            target_piece,
            relative_offset,
            text_len_before_add,
            text.len(),
        );

        // Create a new list of pieces by replacing the target piece with the new pieces resulting
        // from the split.
        let new_pieces = [left_piece, middle_piece, right_piece]
            .into_iter()
            .filter(|p| p.length > 0)
            .collect::<Vec<_>>();

        // Update the pieces by replacing the target piece with the new pieces. This effectively
        // Inserts the new text into the logical view of the text without modifying the underlying
        // buffers.
        self.pieces.splice(target_idx..=target_idx, new_pieces);
    }

    /// Deletes text from the specified offset and length. This method will update the pieces to
    /// reflect the deletion, effectively removing the specified range of text from the logical
    /// view without modifying the underlying buffers.
    ///
    /// # Arguments
    /// - `offset`: The starting position of the text to be deleted.
    /// - `length`: The number of characters to delete from the specified offset.
    pub fn delete(&mut self, offset: usize, length: usize) {
        if length == 0 {
            return;
        }

        // Create a new list of pieces by iterating through the existing pieces and determining how
        // they are affected by the deletion.
        let mut new_pieces = Vec::with_capacity(self.pieces.len());
        let mut current_offset = 0;

        let delete_start = offset;
        let delete_end = offset + length;

        for piece in &self.pieces {
            let piece_start = current_offset;
            let piece_end = current_offset + piece.length;

            // If the piece is completely outside the deletion range, we keep it as is.
            if piece_end <= delete_start || piece_start >= delete_end {
                new_pieces.push(*piece);
            }
            // If the piece overlaps with the deletion range, we need to adjust it accordingly.
            else {
                // If the piece starts before the deletion range, we keep the left part of the
                // piece that is outside the deletion range.
                if piece_start < delete_start {
                    let left_length = delete_start - piece_start;
                    new_pieces.push(Piece {
                        buffer_id: piece.buffer_id,
                        start_idx: piece.start_idx,
                        length: left_length,
                    });
                }

                // If the piece ends after the deletion range, we keep the right part of the piece
                // that is outside the deletion range.
                if piece_end > delete_end {
                    let right_lenght = piece_end - delete_end;
                    let right_start = piece.start_idx + (delete_end - piece_start);
                    new_pieces.push(Piece {
                        buffer_id: piece.buffer_id,
                        start_idx: right_start,
                        length: right_lenght,
                    });
                }
            }

            current_offset += piece.length;
        }

        // Update the pieces with the new list of pieces that reflects the deletion. This
        // effectively removes the specified range of text from the logical view without modifying
        // the underlying buffers.
        self.pieces = new_pieces;
    }

    /// Retrieves the full text represented by the piece table by concatenating the pieces from
    /// both buffers. This method iterates through the pieces and constructs the final text based
    /// on the buffer references.
    ///
    /// # Returns
    /// A `String` containing the full text represented by the piece table.
    pub fn get_text(&self) -> String {
        let total_length: usize = self.pieces.iter().map(|piece| piece.length).sum();

        let mut text = String::with_capacity(total_length);

        for piece in self.pieces.iter() {
            if piece.length == 0 {
                continue;
            }

            let end_idx = piece.start_idx + piece.length;

            // Depending on which buffer the piece references, we append the corresponding
            // substring to the final text. This allows us to reconstruct the full text based on
            // the pieces and their references to the original and add buffers.
            match piece.buffer_id {
                BufferID::Original => {
                    text.push_str(&self.original_buffer[piece.start_idx..end_idx]);
                }
                BufferID::Add => {
                    text.push_str(&self.add_buffer[piece.start_idx..end_idx]);
                }
            }
        }

        text
    }
}

// ================================================================
// Private helper methods for the PieceTable struct.
// ================================================================
impl PieceTable {
    /// Splits a target piece into three pieces: left, middle, and right. The left piece represents
    /// the portion of the original piece before the insertion point, the middle piece represents
    /// the newly inserted text, and the right piece represents the portion of the original piece
    /// after the insertion point.
    ///
    /// # Arguments
    /// - `target_piece`: The piece that is being split to accommodate the new insertion.
    /// - `relative_offset`: The offset within the target piece where the new text is being inserted.
    /// - `text_len_before_add`: The length of the add buffer before the newly inserted text is added, used to calculate the starting index for the middle piece.
    ///
    /// # Returns
    /// A tuple containing the left piece, middle piece, and right piece resulting from the split.
    fn split_piece(
        &self,
        target_piece: Piece,
        relative_offset: usize,
        text_len_before_add: usize,
        text_len: usize,
    ) -> (Piece, Piece, Piece) {
        let left_piece = Piece {
            buffer_id: target_piece.buffer_id,
            start_idx: target_piece.start_idx,
            length: relative_offset,
        };

        let middle_piece = Piece {
            buffer_id: BufferID::Add,
            start_idx: text_len_before_add,
            length: text_len,
        };

        let right_piece = Piece {
            buffer_id: target_piece.buffer_id,
            start_idx: target_piece.start_idx + relative_offset,
            length: target_piece.length - relative_offset,
        };

        (left_piece, middle_piece, right_piece)
    }

    /// Finds the index of the piece that contains the specified offset and calculates the relative_offset
    /// within that piece. This method iterates through the pieces, keeping track of the cumulative
    /// length until it finds the piece that contains the offset.
    ///
    /// # Arguments
    /// - `offset`: The position in the text for which to find the corresponding piece index and relative offset.
    ///
    /// # Returns
    /// A tuple containing the index of the piece that contains the offset and the relative offset within that piece.
    fn find_piece_idx_from_offset(&self, offset: usize) -> (usize, usize) {
        // logical offset represents the cumulative length of the pieces as we iterate through
        // them.
        let mut logical_offset = 0;

        // relative_offset will be the offset within the target piece where the new text will be
        // inserted.
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

        (target_idx, relative_offset)
    }
}

// ================================================================
// Unit tests for the PieceTable implementation.
// ================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_insertion() {
        let mut pt = PieceTable::new("Hello World".to_string());

        pt.insert(5, " Beautiful");
        assert_eq!(
            pt.get_text(),
            "Hello Beautiful World",
            "L'inserimento a metà ha fallito"
        );
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

        assert_eq!(
            pt.get_text(),
            "abc",
            "La gestione degli indici relativi ha fallito"
        );
    }

    #[test]
    fn test_delete_trimming() {
        let mut pt = PieceTable::new("Hello World".to_string());

        pt.delete(5, 6);
        assert_eq!(pt.get_text(), "Hello");
    }

    #[test]
    fn test_delete_hole_in_middle() {
        let mut pt = PieceTable::new("Beautiful".to_string());

        pt.delete(3, 3);
        assert_eq!(pt.get_text(), "Beaful");
    }

    #[test]
    fn test_delete_spanning_multiple_pieces() {
        let mut pt = PieceTable::new("Hello".to_string());
        pt.insert(5, " Beautiful");
        pt.insert(15, " World");

        pt.delete(3, 6);
        assert_eq!(pt.get_text(), "Helutiful World");
    }
}
