use crate::{movetable::MoveTable, types::*};

/// A method returning a list of pseudo-legal pawn moves playable according to
/// the information encoded in this instance of GameManager
/// * `color` - the `Color` of the side to move
/// * `pawn_locations` - a `Vec<u64>` containing a list of each pawn's location
/// * `friendly_pieces` - a `u64` representing the current position of allied pieces
/// * `enemy_pieces` - a `u64` representing the current position of enemy pieces
/// * `returns` - a `Vec` of tuples representing playable pawn moves in the following form:\
///     (the `PieceType` of the piece to move, the starting `Square`,
///     the target `Square`, and the `MoveType`)
pub fn pseudolegal_pawn_moves(
    color: Color,
    movetable: &MoveTable,
    pawn_locations: Vec<u64>,
    friendly_pieces: u64,
    enemy_pieces: u64,
    en_passant_target: &str,
) -> Vec<Move> {
    let mut pawn_pseudo_legal_moves = Vec::new();

    //pawns can do:
    /* Quiet Move
     * Double Pawn Push
     * Capture
     * En Passant Capture
     * 4 Promotions
     * 4 Capture Promotions
     */
    let rank_1: u64 = 0x00000000_000000FF;
    let rank_2: u64 = 0x00000000_0000FF00;
    let rank_7: u64 = 0x00FF0000_00000000;
    let rank_8: u64 = 0xFF000000_00000000;

    match color {
        Color::Black => {
            for pawn in pawn_locations {
                for r in movetable.get_moves(color, PieceType::Pawn, pawn) {
                    for m in r {
                        if m & friendly_pieces == 0 {
                            // ...then this move does not intersect any friendly pieces
                            let from = Square::from_u64(pawn).expect("Each u64 is a power of two");
                            let to = Square::from_u64(m).expect("Each u64 is a power of two");

                            if (m & (pawn >> 8)) == m {
                                // then this move is a type of pawn push
                                if m & enemy_pieces == 0 {
                                    //then this push is not blocked
                                    if m & rank_1 == m {
                                        //then this move will either be:
                                        /* Promotion to Knight
                                         * Promotion to Bishop
                                         * Promotion to Rook
                                         * Promotion to Queen
                                         */

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::NPromotion,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::BPromotion,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::RPromotion,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::QPromotion,
                                        ));
                                    } else {
                                        //just a normal pawn push

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::QuietMove,
                                        ));
                                    }
                                }
                            } else if (pawn & rank_7 == pawn) && (m & (pawn >> 16)) == m {
                                //then this move is a double push
                                if (friendly_pieces | enemy_pieces) & (m | (m << 8)) == 0 {
                                    //then there is in between the pawn and the fifth rank

                                    pawn_pseudo_legal_moves.push((
                                        PieceType::Pawn,
                                        from,
                                        to,
                                        MoveType::DoublePawnPush,
                                    ));
                                }
                            } else {
                                //this move is a type of capture
                                if m & enemy_pieces == m {
                                    //then this move is definitely a capture
                                    if m & rank_1 == m {
                                        //then this move is a promotion capture

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::NPromoCapture,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::BPromoCapture,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::RPromoCapture,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::QPromoCapture,
                                        ));
                                    } else {
                                        //then this move is a standard capture

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::Capture,
                                        ));
                                    }
                                } else if match Square::from_str(en_passant_target) {
                                    Some(coord) => m & coord.to_u64() == m,
                                    None => false,
                                } {
                                    //then this move is an en passant capture

                                    pawn_pseudo_legal_moves.push((
                                        PieceType::Pawn,
                                        from,
                                        to,
                                        MoveType::EPCapture,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        Color::White => {
            for pawn in pawn_locations {
                for r in movetable.get_moves(color, PieceType::Pawn, pawn) {
                    for m in r {
                        if m & friendly_pieces == 0 {
                            // ...then this move does not intersect any friendly pieces
                            let from = Square::from_u64(pawn).expect("Each u64 is a power of two");
                            let to = Square::from_u64(m).expect("Each u64 is a power of two");

                            if (m & (pawn << 8)) == m {
                                // then this move is a type of pawn push
                                if m & enemy_pieces == 0 {
                                    //then this push is not blocked
                                    if m & rank_8 == m {
                                        //then this move will either be:
                                        /* Promotion to Knight
                                         * Promotion to Bishop
                                         * Promotion to Rook
                                         * Promotion to Queen
                                         */

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::NPromotion,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::BPromotion,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::RPromotion,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::QPromotion,
                                        ));
                                    } else {
                                        //just a normal pawn push

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::QuietMove,
                                        ));
                                    }
                                }
                            } else if (pawn & rank_2 == pawn) && (m & (pawn << 16)) == m {
                                //then this move is a double push
                                if (friendly_pieces | enemy_pieces) & (m | (m >> 8)) == 0 {
                                    //then there is in between the pawn and the fourth rank

                                    pawn_pseudo_legal_moves.push((
                                        PieceType::Pawn,
                                        from,
                                        to,
                                        MoveType::DoublePawnPush,
                                    ));
                                }
                            } else {
                                //this move is a type of capture
                                if m & enemy_pieces == m {
                                    //then this move is definitely a capture
                                    if m & rank_8 == m {
                                        //then this move is a promotion capture

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::NPromoCapture,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::BPromoCapture,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::RPromoCapture,
                                        ));
                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::QPromoCapture,
                                        ));
                                    } else {
                                        //then this move is a standard capture

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::Capture,
                                        ));
                                    }
                                } else if match Square::from_str(en_passant_target) {
                                    Some(coord) => m & coord.to_u64() == m,
                                    None => false,
                                } {
                                    //then this move is an en passant capture

                                    pawn_pseudo_legal_moves.push((
                                        PieceType::Pawn,
                                        from,
                                        to,
                                        MoveType::EPCapture,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pawn_pseudo_legal_moves
}
