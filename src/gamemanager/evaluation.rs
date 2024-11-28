use crate::types::{Color, MoveType, PieceType};

use super::GameManager;

impl GameManager {
    pub fn evaluate(&self, movetype: MoveType) -> i32 {
        if self.white_to_move {
            // Evaluate for black; better or worse after its move?
            self.bitboard.piece_mass(Color::Black) + movetype_weight(movetype)
        } else {
            // Ditto for white.
            self.bitboard.piece_mass(Color::White) + movetype_weight(movetype)
        }
    }
}

fn movetype_weight(mt: MoveType) -> i32 {
    use PieceType::*;
    match mt {
        MoveType::QPromotion => Queen as i32,
        MoveType::QPromoCapture => Queen as i32 + 50,
        MoveType::RPromotion => Rook as i32,
        MoveType::RPromoCapture => Rook as i32 + 50,
        MoveType::BPromotion => Bishop as i32,
        MoveType::BPromoCapture => Bishop as i32 + 50,
        MoveType::NPromotion => Knight as i32,
        MoveType::NPromoCapture => Knight as i32 + 50,
        MoveType::EPCapture => Pawn as i32,
        _ => 0,
    }
}
