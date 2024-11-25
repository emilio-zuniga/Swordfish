use crate::{movetable::MoveTable, types::*};

pub fn pseudolegal_queen_moves(
    color: Color,
    movetable: &MoveTable,
    queen_locations: Vec<u64>,
    friendly_pieces: u64,
    enemy_pieces: u64,
) -> Vec<Move> {
    let mut queen_pseudo_legal_moves = Vec::new();

    match color {
        Color::Black => {
            for queen in queen_locations {
                for r in movetable.get_moves(Color::Black, PieceType::Queen, queen) {
                    for m in r {
                        if m & friendly_pieces != 0 {
                            // then this move encounters a collision with a friendly piece
                            break;
                        } else {
                            // ...then this move does not intersect any friendly pieces
                            let from = Square::from_u64(queen).expect("Each u64 is a power of two");
                            let to = Square::from_u64(m).expect("Each u64 is a power of two");
                            debug_assert!(from != to);
                            if m & enemy_pieces != 0 {
                                // then this move is a capture
                                queen_pseudo_legal_moves.push((
                                    PieceType::Queen,
                                    from,
                                    to,
                                    MoveType::Capture,
                                ));
                                break;
                            } else {
                                // this is a quiet move
                                queen_pseudo_legal_moves.push((
                                    PieceType::Queen,
                                    from,
                                    to,
                                    MoveType::QuietMove,
                                ));
                            }
                        }
                    }
                }
            }
        }
        Color::White => {
            for queen in queen_locations {
                for r in movetable.get_moves(Color::White, PieceType::Queen, queen) {
                    for m in r {
                        if m & friendly_pieces != 0 {
                            // then this move does not intersect with a friendly piece
                            break;
                        } else {
                            // then this move does not collide with any friendly pieces
                            let from = Square::from_u64(queen).expect("Each u64 is a power of two");
                            let to = Square::from_u64(m).expect("Each u64 is a power of two");
                            debug_assert!(from != to);
                            if m & enemy_pieces != 0 {
                                // then this move is a capture
                                queen_pseudo_legal_moves.push((
                                    PieceType::Queen,
                                    from,
                                    to,
                                    MoveType::Capture,
                                ));
                                break;
                            } else {
                                // this is a quiet move
                                queen_pseudo_legal_moves.push((
                                    PieceType::Queen,
                                    from,
                                    to,
                                    MoveType::QuietMove,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    queen_pseudo_legal_moves
}
