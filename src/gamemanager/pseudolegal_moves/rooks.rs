use crate::{movetable::MoveTable, types::*};

/// Returns all pseudolegal moves the rooks can make from their positions.
/// ## Inputs
/// - color: [`Color`][Color] enum for the current player,
/// - movetable: reference to a [`MoveTable`][MoveTable],
/// - rook_locations: list of individual pieces,
/// - friendly_pieces: all friendly pieces and-ed together,
/// - enemy_pieces: ditto for enemies
///
/// ## Returns
/// Returns a list of pseudolegal moves with the type alias [`Move`][Move],
/// which expands to `(PieceType, Square, Square, MoveType)`.
pub fn pseudolegal_rook_moves(
    color: Color,
    movetable: &MoveTable,
    rook_locations: Vec<u64>,
    friendly_pieces: u64,
    enemy_pieces: u64,
) -> Vec<Move> {
    let mut rook_pseudo_legal_moves = Vec::new();

    match color {
        Color::Black => {
            for rook in rook_locations {
                for r in movetable.get_moves(Color::Black, PieceType::Rook, rook) {
                    for m in r {
                        let from = Square::from_u64(rook).expect("Must be a power of two!");
                        let to = Square::from_u64(m).expect("Must be a power of two!");

                        if m & friendly_pieces != 0 {
                            break;
                        } else {
                            if m & enemy_pieces == 0 {
                                rook_pseudo_legal_moves.push((
                                    PieceType::Rook,
                                    from,
                                    to,
                                    MoveType::QuietMove,
                                ));
                            } else {
                                rook_pseudo_legal_moves.push((
                                    PieceType::Rook,
                                    from,
                                    to,
                                    MoveType::Capture,
                                ));
                                break;
                            }
                        }
                    }
                }
            }
        }
        Color::White => {
            for rook in rook_locations {
                for r in movetable.get_moves(Color::White, PieceType::Rook, rook) {
                    for m in r {
                        let from = Square::from_u64(rook).expect("Must be a power of two!");
                        let to = Square::from_u64(m).expect("Must be a power of two!");

                        if m & friendly_pieces != 0 {
                            break;
                        } else {
                            if m & enemy_pieces == 0 {
                                rook_pseudo_legal_moves.push((
                                    PieceType::Rook,
                                    from,
                                    to,
                                    MoveType::QuietMove,
                                ));
                            } else {
                                rook_pseudo_legal_moves.push((
                                    PieceType::Rook,
                                    from,
                                    to,
                                    MoveType::Capture,
                                ));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    rook_pseudo_legal_moves
}

#[cfg(test)]
mod tests {
    use crate::gamemanager::pseudolegal_moves::rooks;
    use crate::{movetable::MoveTable, types::*};
    use std::collections::HashSet;

    #[test]
    fn check_rook_pslm() {
        use Square::*;

        let pslm = rooks::pseudolegal_rook_moves(
            Color::Black,
            &MoveTable::default(),
            vec![B5.to_u64()],
            0,
            0,
        );
        let moves: HashSet<u64> = HashSet::from_iter(
            vec![
                A5.to_u64(),
                C5.to_u64(),
                D5.to_u64(),
                E5.to_u64(),
                F5.to_u64(),
                G5.to_u64(),
                H5.to_u64(),
                B8.to_u64(),
                B7.to_u64(),
                B6.to_u64(),
                B4.to_u64(),
                B3.to_u64(),
                B2.to_u64(),
                B1.to_u64(),
            ]
            .iter()
            .cloned(),
        );
        assert!(pslm.iter().all(|m| moves.contains(&m.2.to_u64())));
        assert_eq!(pslm.len(), moves.len())
    }
}
