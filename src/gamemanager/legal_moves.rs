//! This module handles filtering of pseudolegal moves, and returns only legal moves from any game state.

use crate::{
    bitboard::*,
    gamemanager::*,
    types::{CastlingRights, Color, MoveType, PieceType, Square},
};

pub mod perft;

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
            &self.en_passant_target,
            self.halfmoves,
            self.fullmoves,
            &tbl,
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
            use MoveType::*;
            match mv.3 {
                QuietMove | KingCastle | QueenCastle => {
                    modified_gm.halfmoves += 1;
                    modified_gm.en_passant_target = String::new(); // Made a quiet move instead of EPCapture.
                }
                EPCapture => {
                    modified_gm.halfmoves = 0;
                    modified_gm.en_passant_target = String::new()
                }
                _ => modified_gm.halfmoves = 0,
            }

            // TODO: Test mask and king intersection.
            let enemy_attacked = modified_gm.attacked_by(
                &tbl,
                match color {
                    Color::Black => Color::White,
                    Color::White => Color::Black,
                },
            );

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

    /// Extracted from the large match block above.
    pub(super) fn black_match_block(
        &self,
        piecetype: PieceType,
        movetype: MoveType,
        from: Square,
        to: Square,
    ) -> GameManager {
        // For each type of piece, there are at least two moves that can be made, Quiet and Capture.
        // A quiet move needs to update only a handful of things, namely the move clocks, bitboard
        // of the moving piece type, and white_to_move boolean.
        //
        // A capture needs to update the above, and all enemy bitboards. If the capture is also a
        // pawn move, then it may be a pawn promotion, which means two friendly and all enemy boards
        // would be updated.
        //
        // If the move is a special move, it may also need to update:
        // - en_passant_target if it is a DoublePawnPush or EPCapture,
        // - castling_rights if it is a KingCastle or QueenCastle move.
        //
        // THESE SHOULD HOLD FOR ALL CODE BLOCKS BELOW! CHECK THIS IN REVIEW, VERY CAREFULLY, OR ELSE!
        match piecetype {
            PieceType::Bishop => match movetype {
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        bishops_black: (self.bitboard.bishops_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),
                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            bishops_black: (self.bitboard.bishops_black ^ from.to_u64())
                                | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                _ => unreachable!("Bishops will never make another type of move."),
            },
            PieceType::Rook => {
                // NOTE: Color-dependent logic.
                let new_castling_rights = if from == Square::A8 {
                    CastlingRecord {
                        black: CastlingRights::Kingside,
                        ..self.castling_rights
                    }
                } else if from == Square::H8 {
                    CastlingRecord {
                        black: CastlingRights::Queenside,
                        ..self.castling_rights
                    }
                } else {
                    self.castling_rights
                };

                match movetype {
                    MoveType::QuietMove => GameManager {
                        bitboard: BitBoard {
                            rooks_black: (self.bitboard.rooks_black ^ from.to_u64()) | to.to_u64(),
                            ..self.bitboard
                        },
                        castling_rights: new_castling_rights,
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    },
                    MoveType::Capture => {
                        let to_square = to.to_u64();
                        GameManager {
                            bitboard: BitBoard {
                                rooks_black: (self.bitboard.rooks_black ^ from.to_u64())
                                    | to_square,
                                pawns_white: self.bitboard.pawns_white & !to_square,
                                rooks_white: self.bitboard.rooks_white & !to_square,
                                knights_white: self.bitboard.knights_white & !to_square,
                                bishops_white: self.bitboard.bishops_white & !to_square,
                                queens_white: self.bitboard.queens_white & !to_square,
                                ..self.bitboard
                            },
                            castling_rights: new_castling_rights,
                            en_passant_target: self.en_passant_target.clone(),

                            ..*self
                        }
                    }
                    _ => unreachable!("Rooks will never make another type of move."),
                }
            }
            PieceType::King => match movetype {
                // Handle checks on king-side and queen-side castling differently!
                MoveType::KingCastle => GameManager {
                    bitboard: BitBoard {
                        // Check castling spaces for attacks.
                        king_black: (self.bitboard.king_black ^ from.to_u64()) | to.to_u64(),
                        rooks_black: (self.bitboard.rooks_black ^ Square::H8.to_u64())
                            | Square::F8.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::QueenCastle => GameManager {
                    bitboard: BitBoard {
                        king_black: (self.bitboard.king_black ^ from.to_u64()) | to.to_u64(),
                        rooks_black: (self.bitboard.rooks_black ^ Square::A8.to_u64())
                            | Square::C8.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            king_black: (self.bitboard.king_black ^ from.to_u64()) | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        king_black: (self.bitboard.king_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                _ => unreachable!("Kings will never make another type of move."),
            },
            PieceType::Knight => match movetype {
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        knights_black: (self.bitboard.knights_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            knights_black: (self.bitboard.knights_black ^ from.to_u64())
                                | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                _ => unreachable!("Knights will never make another type of move."),
            },
            PieceType::Pawn => match movetype {
                // On a promotion to X, delete the pawn, and place a(n) X on square 'to'.
                MoveType::BPromotion => GameManager {
                    bitboard: BitBoard {
                        pawns_black: (self.bitboard.pawns_black ^ from.to_u64()),
                        bishops_black: self.bitboard.bishops_black | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::RPromotion => GameManager {
                    bitboard: BitBoard {
                        pawns_black: (self.bitboard.pawns_black ^ from.to_u64()),
                        rooks_black: self.bitboard.rooks_black | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::NPromotion => GameManager {
                    bitboard: BitBoard {
                        pawns_black: (self.bitboard.pawns_black ^ from.to_u64()),
                        knights_black: self.bitboard.knights_black | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::QPromotion => GameManager {
                    bitboard: BitBoard {
                        pawns_black: (self.bitboard.pawns_black ^ from.to_u64()),
                        queens_black: self.bitboard.queens_black | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                // On a promotion capture to X delete all enemy pieces
                // at 'to' and place a new X on 'to'.
                MoveType::BPromoCapture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_black: (self.bitboard.pawns_black ^ from.to_u64()),
                            bishops_black: self.bitboard.bishops_black | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::NPromoCapture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_black: (self.bitboard.pawns_black ^ from.to_u64()),
                            knights_black: self.bitboard.knights_black | to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::RPromoCapture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_black: (self.bitboard.pawns_black ^ from.to_u64()),
                            rooks_black: self.bitboard.rooks_black | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::QPromoCapture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_black: (self.bitboard.pawns_black ^ from.to_u64()),
                            queens_black: self.bitboard.queens_black | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::EPCapture => {
                    // Color-dependent logic.
                    use Square::*;
                    let to_square = match to {
                        A3 => A4.to_u64(),
                        B3 => B4.to_u64(),
                        C3 => C4.to_u64(),
                        D3 => D4.to_u64(),
                        E3 => E4.to_u64(),
                        F3 => F4.to_u64(),
                        G3 => G4.to_u64(),
                        H3 => H4.to_u64(),
                        _ => unreachable!(
                            "We will never have a non-rank-3 square as a valid en passant target."
                        ),
                    };

                    GameManager {
                        bitboard: BitBoard {
                            // Move to the target square, behind the targeted piece.
                            pawns_black: (self.bitboard.pawns_black ^ from.to_u64()) | to.to_u64(),
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::DoublePawnPush => {
                    // Color-dependent logic.
                    // Update en_passant_target to square behind the double push.
                    use Square::*;
                    let target_coord = match to {
                        A5 => A6.to_str(),
                        B5 => B6.to_str(),
                        C5 => C6.to_str(),
                        D5 => D6.to_str(),
                        E5 => E6.to_str(),
                        F5 => F6.to_str(),
                        G5 => G6.to_str(),
                        H5 => H6.to_str(),
                        _ => unreachable!(
                            "We will never have a non-rank-5 square as a valid `to` coordinate here."
                        ),
                    };

                    GameManager {
                        bitboard: BitBoard {
                            pawns_black: (self.bitboard.pawns_black ^ from.to_u64()) | to.to_u64(),
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: String::from(target_coord),

                        ..*self
                    }
                }
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        pawns_black: (self.bitboard.pawns_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_black: (self.bitboard.pawns_black ^ from.to_u64()) | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                _ => {
                    eprintln!("{:?}", movetype);
                    unreachable!("Pawns will never make another type of move.")
                }
            },
            PieceType::Queen => match movetype {
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        queens_black: (self.bitboard.queens_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            queens_black: (self.bitboard.queens_black ^ from.to_u64()) | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                _ => unreachable!("Queens will never make another type of move."),
            },
            PieceType::Super => {
                unreachable!("We will never generate pseudolegal Super moves.")
            }
        }
    }

    /// Extracted from the large match block above.
    pub(super) fn white_match_block(
        &self,
        piecetype: PieceType,
        movetype: MoveType,
        from: Square,
        to: Square,
    ) -> GameManager {
        // For each type of piece, there are at least two moves that can be made, Quiet and Capture.
        // A quiet move needs to update only a handful of things, namely the move clocks, bitboard
        // of the moving piece type, and black_to_move boolean.
        //
        // A capture needs to update the above, and all enemy bitboards. If the capture is also a
        // pawn move, then it may be a pawn promotion, which means two friendly and all enemy boards
        // would be updated.
        //
        // If the move is a special move, it may also need to update:
        // - en_passant_target if it is a DoublePawnPush or EPCapture,
        // - castling_rights if it is a KingCastle or QueenCastle move.
        //
        // THESE SHOULD HOLD FOR ALL CODE BLOCKS BELOW! CHECK THIS IN REVIEW, VERY CAREFULLY, OR ELSE!
        match piecetype {
            PieceType::Bishop => match movetype {
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        bishops_white: (self.bitboard.bishops_white ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            bishops_white: (self.bitboard.bishops_white ^ from.to_u64())
                                | to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                _ => unreachable!("Bishops will never make another type of move."),
            },
            PieceType::Rook => {
                // NOTE: Color-dependent logic.
                let new_castling_rights = if from == Square::A1 {
                    CastlingRecord {
                        white: CastlingRights::Kingside,
                        ..self.castling_rights
                    }
                } else if from == Square::H1 {
                    CastlingRecord {
                        white: CastlingRights::Queenside,
                        ..self.castling_rights
                    }
                } else {
                    self.castling_rights
                };

                match movetype {
                    MoveType::QuietMove => GameManager {
                        bitboard: BitBoard {
                            rooks_white: (self.bitboard.rooks_white ^ from.to_u64()) | to.to_u64(),
                            ..self.bitboard
                        },
                        castling_rights: new_castling_rights,
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    },
                    MoveType::Capture => {
                        let to_square = to.to_u64();
                        GameManager {
                            bitboard: BitBoard {
                                rooks_white: (self.bitboard.rooks_white ^ from.to_u64())
                                    | to_square,
                                pawns_black: self.bitboard.pawns_black & !to_square,
                                rooks_black: self.bitboard.rooks_black & !to_square,
                                knights_black: self.bitboard.knights_black & !to_square,
                                bishops_black: self.bitboard.bishops_black & !to_square,
                                queens_black: self.bitboard.queens_black & !to_square,
                                ..self.bitboard
                            },
                            castling_rights: new_castling_rights,
                            en_passant_target: self.en_passant_target.clone(),

                            ..*self
                        }
                    }
                    _ => unreachable!("Rooks will never make another type of move."),
                }
            }
            PieceType::King => match movetype {
                // Handle checks on king-side and queen-side castling differently!
                MoveType::KingCastle => GameManager {
                    bitboard: BitBoard {
                        // Check castling spaces for attacks.
                        king_white: (self.bitboard.king_white ^ from.to_u64()) | to.to_u64(),
                        rooks_white: (self.bitboard.rooks_white ^ Square::H1.to_u64())
                            | Square::F1.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::QueenCastle => GameManager {
                    bitboard: BitBoard {
                        king_white: (self.bitboard.king_white ^ from.to_u64()) | to.to_u64(),
                        rooks_white: (self.bitboard.rooks_white ^ Square::A1.to_u64())
                            | Square::C1.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            king_white: (self.bitboard.king_white ^ from.to_u64()) | to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        king_white: (self.bitboard.king_white ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                _ => unreachable!("Kings will never make another type of move."),
            },
            PieceType::Knight => match movetype {
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        knights_white: (self.bitboard.knights_white ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            knights_white: (self.bitboard.knights_white ^ from.to_u64())
                                | to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                _ => unreachable!("Knights will never make another type of move."),
            },
            PieceType::Pawn => match movetype {
                // On a promotion to X, delete the pawn, and place a(n) X on square 'to'.
                MoveType::BPromotion => GameManager {
                    bitboard: BitBoard {
                        pawns_white: (self.bitboard.pawns_white ^ from.to_u64()),
                        bishops_white: self.bitboard.bishops_white | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::RPromotion => GameManager {
                    bitboard: BitBoard {
                        pawns_white: (self.bitboard.pawns_white ^ from.to_u64()),
                        rooks_white: self.bitboard.rooks_white | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::NPromotion => GameManager {
                    bitboard: BitBoard {
                        pawns_white: (self.bitboard.pawns_white ^ from.to_u64()),
                        knights_white: self.bitboard.knights_white | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::QPromotion => GameManager {
                    bitboard: BitBoard {
                        pawns_white: (self.bitboard.pawns_white ^ from.to_u64()),
                        queens_white: self.bitboard.queens_white | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                // On a promotion capture to X delete all enemy pieces
                // at 'to' and place a new X on 'to'.
                MoveType::BPromoCapture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_white: (self.bitboard.pawns_white ^ from.to_u64()),
                            bishops_white: self.bitboard.bishops_white | to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::NPromoCapture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_white: (self.bitboard.pawns_white ^ from.to_u64()),
                            knights_white: self.bitboard.knights_white | to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::RPromoCapture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_white: (self.bitboard.pawns_white ^ from.to_u64()),
                            rooks_white: self.bitboard.rooks_white | to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::QPromoCapture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_white: (self.bitboard.pawns_white ^ from.to_u64()),
                            queens_white: self.bitboard.queens_white | to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::EPCapture => {
                    // Color-dependent logic.
                    use Square::*;
                    let to_square = match to {
                        A6 => A5.to_u64(),
                        B6 => B5.to_u64(),
                        C6 => C5.to_u64(),
                        D6 => D5.to_u64(),
                        E6 => E5.to_u64(),
                        F6 => F5.to_u64(),
                        G6 => G5.to_u64(),
                        H6 => H5.to_u64(),
                        _ => unreachable!(
                            "We will never have a non-rank-6 square as a valid en passant target."
                        ),
                    };

                    GameManager {
                        bitboard: BitBoard {
                            // Move to the target square, behind the targeted piece.
                            pawns_white: (self.bitboard.pawns_white ^ from.to_u64()) | to.to_u64(),
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                MoveType::DoublePawnPush => {
                    // Color-dependent logic.
                    // Update en_passant_target to square behind the double push.
                    use Square::*;
                    let target_coord = match to {
                        A4 => A3.to_str(),
                        B4 => B3.to_str(),
                        C4 => C3.to_str(),
                        D4 => D3.to_str(),
                        E4 => E3.to_str(),
                        F4 => F3.to_str(),
                        G4 => G3.to_str(),
                        H4 => H3.to_str(),
                        _ => unreachable!(
                            "We will never have a non-rank-4 square as a valid `to` coordinate here."
                        ),
                    };

                    GameManager {
                        bitboard: BitBoard {
                            pawns_white: (self.bitboard.pawns_white ^ from.to_u64()) | to.to_u64(),
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: String::from(target_coord),

                        ..*self
                    }
                }
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        pawns_white: (self.bitboard.pawns_white ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            pawns_white: (self.bitboard.pawns_white ^ from.to_u64()) | to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                _ => {
                    eprintln!("{:?}", movetype);
                    unreachable!("Pawns will never make another type of move.")
                }
            },
            PieceType::Queen => match movetype {
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        queens_white: (self.bitboard.queens_white ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),

                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            queens_white: (self.bitboard.queens_white ^ from.to_u64()) | to_square,
                            pawns_black: self.bitboard.pawns_black & !to_square,
                            rooks_black: self.bitboard.rooks_black & !to_square,
                            knights_black: self.bitboard.knights_black & !to_square,
                            bishops_black: self.bitboard.bishops_black & !to_square,
                            queens_black: self.bitboard.queens_black & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),

                        ..*self
                    }
                }
                _ => unreachable!("Queens will never make another type of move."),
            },
            PieceType::Super => {
                unreachable!("We will never generate pseudolegal Super moves.")
            }
        }
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
