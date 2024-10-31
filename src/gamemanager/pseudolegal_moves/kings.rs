use crate::{movetable::MoveTable, types::*};

fn pseudolegal_king_moves(
    color: Color,
    movetable: &MoveTable,
    king_locations: Vec<u64>,
    friendly_pieces: u64,
    enemy_pieces: u64,
) -> Vec<Move> {
    let mut king_pseudo_legal_moves = Vec::new();

    match color {
        Color::Black => {
            for king in king_locations {
                for r in movetable.get_moves(Color::Black, PieceType::King, king) {
                    for m in r {
                        if m & friendly_pieces == 0 {
                            let from = Square::from_u64(king).expect("Each u64 is a power of two");
                            let to = Square::from_u64(m).expect("Each u64 is a power of two");
                            if m & enemy_pieces == 0 {
                                king_pseudo_legal_moves.push((
                                    PieceType::King,
                                    from,
                                    to,
                                    MoveType::QuietMove,
                                ));
                            } else {
                                // Capture.
                                king_pseudo_legal_moves.push((
                                    PieceType::King,
                                    from,
                                    to,
                                    MoveType::Capture,
                                ));
                            }
                        }
                    }
                }
            }
        }
        Color::White => {
            for king in king_locations {
                for r in movetable.get_moves(Color::White, PieceType::King, king) {
                    for m in r {
                        if m & friendly_pieces == 0 {
                            // ...then this move does not intersect any friendly pieces
                            let from = Square::from_u64(king).expect("Each u64 is a power of two");
                            let to = Square::from_u64(m).expect("Each u64 is a power of two");
                            if m & enemy_pieces == 0 {
                                king_pseudo_legal_moves.push((
                                    PieceType::King,
                                    from,
                                    to,
                                    MoveType::QuietMove,
                                ));
                            } else {
                                // Capture.
                                king_pseudo_legal_moves.push((
                                    PieceType::King,
                                    from,
                                    to,
                                    MoveType::Capture,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    king_pseudo_legal_moves
}

#[cfg(test)]
mod tests {
    use crate::{gamemanager::pseudolegal_moves::kings, movetable::MoveTable, types::*};
    use std::collections::HashSet;
    use Square::*;

    #[test]
    fn check_king_pslm() {
        let pslnm = kings::pseudolegal_king_moves(
            Color::Black,
            &MoveTable::default(),
            vec![B5.to_u64()],
            0,
            0,
        );
        let moves: HashSet<u64> = HashSet::from_iter(
            vec![
                A6.to_u64(),
                B6.to_u64(),
                C6.to_u64(),
                A5.to_u64(),
                C5.to_u64(),
                A4.to_u64(),
                B4.to_u64(),
                C4.to_u64(),
            ]
            .iter()
            .cloned(),
        );
        assert!(pslnm.iter().all(|m| moves.contains(&m.2.to_u64())));
        assert_eq!(pslnm.len(), moves.len())
    }
}
