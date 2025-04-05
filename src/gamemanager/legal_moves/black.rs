use crate::{
    bitboard::*,
    gamemanager::*,
    types::{CastlingRights, MoveType, PieceType, Square},
};

impl GameManager {
    /// Extracted from legal_moves().
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
        let retval = match piecetype {
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
                        black: match self.castling_rights.black {
                            CastlingRights::Both => CastlingRights::Kingside,
                            CastlingRights::Queenside => CastlingRights::Neither,
                            _ => self.castling_rights.black,
                        },
                        ..self.castling_rights
                    }
                } else if from == Square::H8 {
                    CastlingRecord {
                        black: match self.castling_rights.black {
                            CastlingRights::Both => CastlingRights::Queenside,
                            CastlingRights::Kingside => CastlingRights::Neither,
                            _ => self.castling_rights.black,
                        },
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
                    castling_rights: CastlingRecord {
                        black: CastlingRights::Neither,
                        ..self.castling_rights
                    },
                    en_passant_target: self.en_passant_target.clone(),
                    ..*self
                },
                MoveType::QueenCastle => GameManager {
                    bitboard: BitBoard {
                        king_black: (self.bitboard.king_black ^ from.to_u64()) | to.to_u64(),
                        rooks_black: (self.bitboard.rooks_black ^ Square::A8.to_u64())
                            | Square::D8.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: CastlingRecord {
                        black: CastlingRights::Neither,
                        ..self.castling_rights
                    },
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
                        castling_rights: CastlingRecord {
                            black: CastlingRights::Neither,
                            ..self.castling_rights
                        },
                        en_passant_target: self.en_passant_target.clone(),
                        ..*self
                    }
                }
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        king_black: (self.bitboard.king_black ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: CastlingRecord {
                        black: CastlingRights::Neither,
                        ..self.castling_rights
                    },
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
                        A5 => A6,
                        B5 => B6,
                        C5 => C6,
                        D5 => D6,
                        E5 => E6,
                        F5 => F6,
                        G5 => G6,
                        H5 => H6,
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
                        en_passant_target: Some(target_coord),
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
        };

        assert!(
            retval.bitboard.king_black.is_power_of_two(),
            "{} {:?} {:?} {:#X} {:#X}\n",
            retval.bitboard.to_string(),
            retval.castling_rights.black,
            retval.castling_rights.white,
            retval.bitboard.king_black,
            retval.bitboard.king_white
        );
        assert!(
            retval.bitboard.king_white.is_power_of_two(),
            "{} {:?} {:?} {:#X} {:#X}\n",
            retval.bitboard.to_string(),
            retval.castling_rights.black,
            retval.castling_rights.white,
            retval.bitboard.king_black,
            retval.bitboard.king_white
        );

        retval
    }
}
