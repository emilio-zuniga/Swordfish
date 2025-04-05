use crate::{
    bitboard::*,
    gamemanager::*,
    types::{CastlingRights, MoveType, PieceType, Square},
};

impl GameManager {
    /// Extracted from the large match block above.
    pub(super) fn white_match_block(
        &self,
        piecetype: PieceType,
        movetype: MoveType,
        from: Square,
        to: Square,
    ) -> GameManager {
        assert!(self.bitboard.king_black.is_power_of_two());
        assert!(self.bitboard.king_white.is_power_of_two());
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
        let retval = match piecetype {
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
                        white: match self.castling_rights.white {
                            CastlingRights::Both => CastlingRights::Kingside,
                            CastlingRights::Queenside => CastlingRights::Neither,
                            _ => self.castling_rights.white,
                        },
                        ..self.castling_rights
                    }
                } else if from == Square::H1 {
                    CastlingRecord {
                        white: match self.castling_rights.white {
                            CastlingRights::Both => CastlingRights::Queenside,
                            CastlingRights::Kingside => CastlingRights::Neither,
                            _ => self.castling_rights.white,
                        },
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
                    castling_rights: CastlingRecord {
                        white: CastlingRights::Neither,
                        ..self.castling_rights
                    },
                    en_passant_target: self.en_passant_target.clone(),
                    ..*self
                },
                MoveType::QueenCastle => GameManager {
                    bitboard: BitBoard {
                        king_white: (self.bitboard.king_white ^ from.to_u64()) | to.to_u64(),
                        rooks_white: (self.bitboard.rooks_white ^ Square::A1.to_u64())
                            | Square::D1.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: CastlingRecord {
                        white: CastlingRights::Neither,
                        ..self.castling_rights
                    },
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
                        castling_rights: CastlingRecord {
                            white: CastlingRights::Neither,
                            ..self.castling_rights
                        },
                        en_passant_target: self.en_passant_target.clone(),
                        ..*self
                    }
                }
                MoveType::QuietMove => GameManager {
                    bitboard: BitBoard {
                        king_white: (self.bitboard.king_white ^ from.to_u64()) | to.to_u64(),
                        ..self.bitboard
                    },
                    castling_rights: CastlingRecord {
                        white: CastlingRights::Neither,
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
                        A4 => A3,
                        B4 => B3,
                        C4 => C3,
                        D4 => D3,
                        E4 => E3,
                        F4 => F3,
                        G4 => G3,
                        H4 => H3,
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
                        en_passant_target: Some(target_coord),
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
