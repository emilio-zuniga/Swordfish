//! This module handles filtering of pseudolegal moves, and returns only legal moves from any game state.

use crate::{
    gamemanager::*,
    types::{Color, MoveType, PieceType, Square},
};

mod black;
pub mod perft;
mod white;

impl GameManager {
    /// Returns all legal moves possible from this GameManager's state.
    /// This results in a list of possible GameManagers that could result from
    /// the possible moves, which are also returned. Each can be evaluated for
    /// strengths and weaknesses.
    pub fn legal_moves(
        &self,
        tbl: &NoArc<MoveTable>,
    ) -> Vec<(PieceType, Square, Square, MoveType, GameManager)> {
        /* ************************************************************************************* */
        /* WARNING: THIS FUNCTION WILL ERROR SILENTLY IF ANY COLOR-DEPENDENT LOGIC IS USED HERE. */
        /*          ALL LOGIC IN THIS FUNCTION MUST BE COLOR-AGNOSTIC.                           */
        /* ************************************************************************************* */
        let color = if self.white_to_move {
            Color::White
        } else {
            Color::Black
        };

        let mut legal_moves: Vec<(PieceType, Square, Square, MoveType, GameManager)> = vec![];

        // First get all the pseudolegal moves.
        let pslm = pseudolegal_moves::pseudolegal_moves(
            color,
            self.bitboard,
            self.castling_rights,
            self.en_passant_target,
            self.halfmoves,
            self.fullmoves,
            &tbl,
        );

        let currently_attacked = self.attacked_by(
            tbl,
            match color {
                Color::Black => Color::White,
                Color::White => Color::Black,
            },
        );

        // ASSERT: We will never have Super moves in the pseudolegal moves vector.
        debug_assert!(pslm
            .iter()
            .all(|(piecetype, _, _, _)| *piecetype != PieceType::Super));

        /* ************************************************************************************* */
        /* DESIGN NOTE: We can make a pslm, then check whether the king intersects the attacked  */
        /*              mask. If the king doesn't intersect the attacked mask, and made no       */
        /*              castling move that put him in danger, then the move was valid. Otherwise */
        /*              discard it and carry on. That should be more performant.                 */
        /* ************************************************************************************* */

        for mv in &pslm {
            debug_assert!(mv.1 != mv.2);

            // Create a new GameManager here.
            let mut modified_gm = {
                match color {
                    Color::Black => self.black_match_block(mv.0.clone(), mv.3.clone(), mv.1, mv.2),
                    Color::White => self.white_match_block(mv.0.clone(), mv.3.clone(), mv.1, mv.2),
                }
            };

            // Increment the fullmove clock every black move.
            if color == Color::Black {
                modified_gm.fullmoves += 1;
                modified_gm.white_to_move = true;
            } else {
                modified_gm.white_to_move = false;
            }

            // Increment the halfmove counter every quiet/non-pawn move.
            // En passant target always equals an empty string unless the
            // move was a double pawn push.
            use MoveType::*;
            match mv.3 {
                QuietMove | KingCastle | QueenCastle => {
                    modified_gm.halfmoves += 1;
                    modified_gm.en_passant_target = None; // Made a quiet move instead of EPCapture.
                }
                DoublePawnPush => {
                    modified_gm.halfmoves = 0; // Leave en passant target alone; it was set by the color-specific function.
                }
                _ => {
                    modified_gm.halfmoves = 0;
                    modified_gm.en_passant_target = None;
                }
            }

            let enemy_attacked = modified_gm.attacked_by(
                &tbl,
                match color {
                    Color::Black => Color::White,
                    Color::White => Color::Black,
                },
            );

            // Test that the castle is not castling through/out of/into check.
            use Square::*;
            match mv.3 {
                KingCastle => {
                    if color == Color::Black
                        && ((E8.to_u64() | F8.to_u64() | G8.to_u64()) & enemy_attacked != 0
                            || (E8.to_u64() | F8.to_u64() | G8.to_u64()) & currently_attacked != 0)
                        || color == Color::White
                            && ((E1.to_u64() | F1.to_u64() | G1.to_u64()) & enemy_attacked != 0
                                || (E1.to_u64() | F1.to_u64() | G1.to_u64()) & currently_attacked
                                    != 0)
                    {
                        continue; // We don't want this move!
                    }
                }
                QueenCastle => {
                    if color == Color::Black
                        && ((E8.to_u64() | D8.to_u64() | C8.to_u64()) & enemy_attacked != 0
                            || (E8.to_u64() | D8.to_u64() | C8.to_u64()) & currently_attacked != 0)
                        || color == Color::White
                            && ((E1.to_u64() | D1.to_u64() | C1.to_u64()) & enemy_attacked != 0
                                || (E1.to_u64() | D1.to_u64() | C1.to_u64()) & currently_attacked
                                    != 0)
                    {
                        continue; // Ditto.
                    }
                }
                _ => {}
            }

            match color {
                Color::Black => {
                    if modified_gm.bitboard.king_black & enemy_attacked == 0 {
                        // Good move; push it.
                        legal_moves.push((mv.0.clone(), mv.1, mv.2, mv.3.clone(), modified_gm));
                    }
                }
                Color::White => {
                    if modified_gm.bitboard.king_white & enemy_attacked == 0 {
                        // Good move; push it.
                        legal_moves.push((mv.0.clone(), mv.1, mv.2, mv.3.clone(), modified_gm));
                    }
                }
            }
        }

        legal_moves
    }
}

#[cfg(test)]
mod tests {
    use crate::gamemanager::*;
    #[test]
    fn test_en_passant() {
        let gm = GameManager::black_match_block(
            &GameManager::from_fen_str("6k1/5p2/4p3/2p1P3/1pP2P2/1P6/8/6K1 b - c3 0 1"),
            crate::types::PieceType::Pawn,
            crate::types::MoveType::EPCapture,
            crate::types::Square::B4,
            crate::types::Square::C3,
        );

        assert_eq!(
            gm.bitboard.pawns_black,
            BitBoard::from_fen_string("6k1/5p2/4p3/2p1P3/5P2/1Pp5/8/6K1 w - - 0 1").pawns_black
        );
        assert_eq!(
            gm.bitboard.pawns_white,
            BitBoard::from_fen_string("6k1/5p2/4p3/2p1P3/5P2/1Pp5/8/6K1 w - - 0 1").pawns_white
        );
    }
}
