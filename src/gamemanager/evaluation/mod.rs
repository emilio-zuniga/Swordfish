use heatmaps::Heatmap;

use crate::{
    bitboard::BitBoard,
    types::{Color, MoveType, PieceType},
};

use super::GameManager;

mod heatmaps;

const START_MASS: i32 = PieceType::Queen as i32
    + PieceType::Rook as i32 * 2
    + PieceType::Bishop as i32 * 2
    + PieceType::Knight as i32 * 2
    + PieceType::Pawn as i32 * 8;

impl GameManager {
    pub fn evaluate(&self, movetype: MoveType) -> i32 {
        if self.white_to_move {
            // Evaluate for black; better or worse after its move?
            let mass_score = self.bitboard.piece_mass(Color::Black) + movetype_weight(movetype);
            let heatmap = heatmaps::Heatmap::default();

            let endgame_weight = mass_score * 100 / START_MASS;

            let position_score =
                eval_heatmaps(Color::Black, self.bitboard, heatmap, endgame_weight);

            position_score + mass_score
        } else {
            // Ditto for white.
            let mass_score = self.bitboard.piece_mass(Color::White) + movetype_weight(movetype);
            let heatmap = heatmaps::Heatmap::default().rev();

            let endgame_weight = mass_score * 100 / START_MASS;

            let position_score =
                eval_heatmaps(Color::White, self.bitboard, heatmap, endgame_weight);

            position_score + mass_score
        }
    }
}

fn movetype_weight(mt: MoveType) -> i32 {
    use MoveType::*;
    use PieceType::*;
    match mt {
        QPromotion => Queen as i32,
        QPromoCapture => Queen as i32 + 50,
        RPromotion => Rook as i32,
        RPromoCapture => Rook as i32 + 50,
        BPromotion => Bishop as i32,
        BPromoCapture => Bishop as i32 + 50,
        NPromotion => Knight as i32,
        NPromoCapture => Knight as i32 + 50,
        EPCapture => Pawn as i32,
        Capture => 400,
        _ => 0,
    }
}

fn eval_heatmaps(color: Color, board: BitBoard, map: Heatmap, endgame_weight: i32) -> i32 {
    let base_value = match color {
        Color::Black => {
            eval_heatmap(map.knights, board.knights_black)
                + eval_heatmap(map.bishops, board.bishops_black)
                + eval_heatmap(map.rooks, board.rooks_black)
                + eval_heatmap(map.queens, board.queens_black)
        }
        Color::White => {
            eval_heatmap(map.knights, board.knights_white)
                + eval_heatmap(map.bishops, board.bishops_white)
                + eval_heatmap(map.rooks, board.rooks_white)
                + eval_heatmap(map.queens, board.queens_white)
        }
    };

    let weighted_value = match color {
        Color::Black => {
            eval_heatmap(map.pawns_start, board.pawns_black) * (100 - endgame_weight)
                + eval_heatmap(map.pawns_end, board.pawns_black) * endgame_weight
                + eval_heatmap(map.kings_start, board.king_black) * (100 - endgame_weight)
                + eval_heatmap(map.kings_end, board.king_black) * endgame_weight
        }
        Color::White => {
            eval_heatmap(map.pawns_start, board.pawns_white) * (100 - endgame_weight)
                + eval_heatmap(map.pawns_end, board.pawns_white) * endgame_weight
                + eval_heatmap(map.kings_start, board.king_white) * (100 - endgame_weight)
                + eval_heatmap(map.kings_end, board.king_white) * endgame_weight
        }
    };

    base_value + weighted_value
}

fn eval_heatmap(table: [i32; 64], bits: u64) -> i32 {
    let mut score = 0;
    let split_bits = split_bits(bits);
    for idx in 0..64 {
        score += table[idx] * split_bits[idx] as i32;
    }
    score
}

fn split_bits(int: u64) -> [u8; 64] {
    let mut bits = [0u8; 64];
    for idx in 0..64 {
        bits[idx] = ((int >> idx) & 1) as u8;
    }
    bits
}
