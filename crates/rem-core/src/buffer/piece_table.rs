//! Piece table implementation

pub enum BufferID {
    Original,
    Add,
}

pub struct Piece {
    pub buffer_id: BufferID,
    pub start_idx: usize,
    pub length:    usize,
}

pub struct PieceTable {
    original_buffer: String,
    add_buffer:      String,
    pieces:          Vec<Piece>,
}

impl PieceTable {
    pub fn new(text: String) -> Self {
        Self { 
            original_buffer: text, 
            add_buffer: String::new(), 
            pieces: Vec::new(),
        }
    }

    pub fn insert(&mut self, offset: usize, text: &str) {
        todo!()
    }

    pub fn delete(&mut self, offset: usize, length: usize) {
        todo!()
    }

    pub fn get_text(&self) -> String {
        todo!()
    }
}
