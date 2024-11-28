use crate::types::Color;

use super::GameManager;

impl GameManager {
    pub fn evaluate(&self) -> i32 {
        if self.white_to_move {
            // Evaluate for black; better or worse after its move?
            self.bitboard.piece_mass(Color::Black)
        } else {
            // Ditto for white.
            self.bitboard.piece_mass(Color::White)
        }
    }
}
