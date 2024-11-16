//! This module handles filtering of pseudolegal moves, and returns only legal moves from any game state.

use crate::{
    bitboard::*,
    gamemanager::pseudolegal_moves,
    types::{Color, MoveType, PieceType, Square},
    MOVETABLE,
};

use super::GameManager;

impl GameManager {
    /// Returns all possible legal evolutions of this GameManager,
    /// as a list of successive GameManagers.
    pub fn legal_evolution(&self) -> Vec<Self> {
        todo!()
    }

    /// Returns all legal moves possible from this GameManager's state.
    /// This results in a list of possible GameManagers that could result from
    /// the possible moves, which are also returned. Each can be evaluated for
    /// strengths and weaknesses.
    pub fn legal_moves(&self, color: Color) {
        let mut legal_moves: Vec<(PieceType, Square, Square, MoveType, GameManager)> = vec![];

        let (friendly_pieces, enemy_pieces) = match color {
            Color::Black => {
                let friendly_pieces = self.bitboard.pawns_black
                    | self.bitboard.rooks_black
                    | self.bitboard.knights_black
                    | self.bitboard.bishops_black
                    | self.bitboard.queens_black
                    | self.bitboard.king_black;
                let enemy_pieces = self.bitboard.pawns_white
                    | self.bitboard.rooks_white
                    | self.bitboard.knights_white
                    | self.bitboard.bishops_white
                    | self.bitboard.queens_white
                    | self.bitboard.king_white;

                (friendly_pieces, enemy_pieces)
            }
            Color::White => {
                let friendly_pieces = self.bitboard.pawns_white
                    | self.bitboard.rooks_white
                    | self.bitboard.knights_white
                    | self.bitboard.bishops_white
                    | self.bitboard.queens_white
                    | self.bitboard.king_white;
                let enemy_pieces = self.bitboard.pawns_black
                    | self.bitboard.rooks_black
                    | self.bitboard.knights_black
                    | self.bitboard.bishops_black
                    | self.bitboard.queens_black
                    | self.bitboard.king_black;

                (friendly_pieces, enemy_pieces)
            }
        };

        let (friendly_king, enemy_king) = match color {
            Color::Black => (self.bitboard.king_black, self.bitboard.king_white),
            Color::White => (self.bitboard.king_white, self.bitboard.king_black),
        };

        // First get all the pseudolegal moves.
        let pslm = pseudolegal_moves::pseudolegal_moves(
            Color::Black,
            self.bitboard,
            &MOVETABLE,
            &self.castling_rights,
            &self.en_passant_target,
            self.halfmoves,
            self.fullmoves,
        );

        // ASSERT: We will never have Super moves in the pseudolegal moves vector.
        debug_assert!(pslm
            .iter()
            .all(|(piecetype, _, _, _)| *piecetype != PieceType::Super));

        // A pseudolegal move may be illegal iff it leaves the king in check.
        // For each possible move, check legality.
        for (piecetype, from, to, movetype) in pslm {
            // Test for king safety against each enemy bitboard,
            // by grabbing all the moves a super piece can make
            // from the king's square.

            // Create a new bitboard here.
            let modified_gm = {
                match color {
                    // For a black piece move, match on the piece's type.
                    Color::Black => {
                        self.black_match_block(piecetype.clone(), movetype.clone(), from, to)
                    }
                    Color::White => {
                        self.white_match_block(piecetype.clone(), movetype.clone(), from, to)
                    }
                }
            };

            // NOTE: Maybe pass a bitboard and movetable reference to a check_legality function? IDK...
            let super_moves = self
                .movetable
                .get_moves(color, PieceType::Super, friendly_king);

            // Just all the Super moves ORed together.
            let all_super_moves: u64 = super_moves
                .iter()
                .fold(0, |acc, ray| acc | ray.iter().fold(0, |acc2, &i| acc2 | i));

            if all_super_moves & enemy_pieces == 0 {
                // The king is not under threat
                // and this is guaranteed (?) to be a legal move.
                // Let's see.
                // * The move doesn't do anything funny. (MoveTable)
                // * The move respects the state of the game. (pseudolegal_moves)
                // * The move doesn't leave the king in check. (No way to reach an enemy piece.)

                // Push it and the modified bitboard to our list of moves.
                legal_moves.push((piecetype, from, to, movetype, modified_gm));
            } else {
                // ...he POTENTIALLY is.
                // Continue to check against each bitboard with moves from the corresponding piece type.
                // List shouldn't include Super variant.
                use PieceType::*;
                for piece in [King, Queen, Knight, Rook, Bishop, Pawn].iter() {
                    let moves = self
                        .movetable
                        .get_moves(color, piece.clone(), friendly_king);

                    let all_type_moves = moves
                        .iter()
                        .fold(0, |acc, ray| acc | ray.iter().fold(0, |acc2, &i| acc2 | i));

                    match color {
                        Color::Black => {
                            todo!()
                        }
                        Color::White => {
                            todo!()
                        }
                    }
                }
            }
        }

        todo!()
    }

    /// Extracted from the large match block above.
    fn black_match_block(
        &self,
        piecetype: PieceType,
        movetype: MoveType,
        from: Square,
        to: Square,
    ) -> GameManager {
        match piecetype {
            PieceType::Bishop => match movetype {
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        bishops_black: (self.bitboard.bishops_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),
                    movetable: &MOVETABLE,
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
                        movetable: &MOVETABLE,
                        ..*self
                    }
                }
                _ => unreachable!("Bishops will never make another type of move."),
            },
            PieceType::Rook => match movetype {
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        rooks_black: (self.bitboard.rooks_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),
                    movetable: &MOVETABLE,
                    ..*self
                },
                MoveType::Capture => {
                    let to_square = to.to_u64();
                    GameManager {
                        bitboard: BitBoard {
                            rooks_black: (self.bitboard.rooks_black ^ from.to_u64()) | to_square,
                            pawns_white: self.bitboard.pawns_white & !to_square,
                            rooks_white: self.bitboard.rooks_white & !to_square,
                            knights_white: self.bitboard.knights_white & !to_square,
                            bishops_white: self.bitboard.bishops_white & !to_square,
                            queens_white: self.bitboard.queens_white & !to_square,
                            ..self.bitboard
                        },
                        castling_rights: self.castling_rights.clone(),
                        en_passant_target: self.en_passant_target.clone(),
                        movetable: &MOVETABLE,
                        ..*self
                    }
                }
                _ => unreachable!("Rooks will never make another type of move."),
            },
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
                    movetable: &MOVETABLE,
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
                    movetable: &MOVETABLE,
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
                        movetable: &MOVETABLE,
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
                    movetable: &MOVETABLE,
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
                    movetable: &MOVETABLE,
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
                        movetable: &MOVETABLE,
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
                    movetable: &MOVETABLE,
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
                    movetable: &MOVETABLE,
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
                    movetable: &MOVETABLE,
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
                    movetable: &MOVETABLE,
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
                        movetable: &MOVETABLE,
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
                        movetable: &MOVETABLE,
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
                        movetable: &MOVETABLE,
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
                        movetable: &MOVETABLE,
                        ..*self
                    }
                }
                MoveType::EPCapture => {
                    // Color-dependent logic.
                    use Square::*;
                    let to_square = match to {
                        A3 => A2.to_u64(),
                        B3 => B2.to_u64(),
                        C3 => C2.to_u64(),
                        D3 => D2.to_u64(),
                        E3 => E2.to_u64(),
                        F3 => F2.to_u64(),
                        G3 => G2.to_u64(),
                        H3 => H2.to_u64(),
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
                        movetable: &MOVETABLE,
                        ..*self
                    }
                }
                MoveType::DoublePawnPush => {
                    // Color-dependent logic.
                    // Update en_passant_target to square behind the double push.
                    // Oh, wait. That will update it for all future move searches, regardless of whether or not
                    // they actually have this property.
                    //
                    // OK, the solution will HAVE to be to copy the GameManager struct.
                    todo!()
                }
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        pawns_black: (self.bitboard.pawns_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: self.castling_rights.clone(),
                    en_passant_target: self.en_passant_target.clone(),
                    movetable: &MOVETABLE,
                    ..*self
                },
                _ => unreachable!("Pawns will never make another type of move."),
            },
            PieceType::Queen => GameManager {
                bitboard: BitBoard {
                    queens_black: (self.bitboard.queens_black ^ from.to_u64()) | to.to_u64(),
                    ..self.bitboard
                },
                castling_rights: self.castling_rights.clone(),
                en_passant_target: self.en_passant_target.clone(),
                movetable: &MOVETABLE,
                ..*self
            },
            PieceType::Super => {
                unreachable!("We will never generate pseudolegal Super moves.")
            }
        }
    }

    fn white_match_block(
        &self,
        piecetype: PieceType,
        movetype: MoveType,
        from: Square,
        to: Square,
    ) -> GameManager {
        todo!()
    }
}
