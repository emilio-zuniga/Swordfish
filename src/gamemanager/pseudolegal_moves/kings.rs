use crate::{movetable::MoveTable, types::*};

/// Returns all pseudolegal moves the kings can make from their positions.
/// ## Inputs
/// - color: [`Color`][Color] enum for the current player,
/// - movetable: reference to a [`MoveTable`][MoveTable],
/// - king_locations: list of individual pieces,
/// - friendly_pieces: all friendly pieces and-ed together,
/// - enemy_pieces: ditto for enemies
///
/// ## Returns
/// Returns a list of pseudolegal moves with the type alias [`Move`][Move],
/// which expands to `(PieceType, Square, Square, MoveType)`.
///
/// ## Examples
/// ```rust
/// use crate::{movetable::MoveTable, types::*, gamemanager::pseudolegal_moves::kings};
/// use Square::*;
///
/// let pslnm = kings::pseudolegal_knight_moves(
///     Color::Black,
///     &MoveTable::default(),
///     vec![B5.to_u64()],
///     0,
///     0,
/// );
/// let moves: HashSet<u64> = HashSet::from_iter(
///         vec![
///             A6.to_u64(),
///             B6.to_u64(),
///             C6.to_u64(),
///             A5.to_u64(),
///             C5.to_u64(),
///             A4.to_u64(),
///             B4.to_u64(),
///             C4.to_u64(),
///         ]
///         .iter()
///         .cloned(),
///     );
/// assert!(pslnm.iter().all(|m| moves.contains(&m.2.to_u64())));
/// assert_eq!(pslnm.len(), moves.len())
/// ```
pub fn pseudolegal_king_moves(
    color: Color,
    movetable: &MoveTable,
    king_locations: Vec<u64>,
    friendly_pieces: u64,
    friendly_rooks: u64,
    enemy_pieces: u64,
    castling_rights: CastlingRecord,
) -> Vec<Move> {
    let mut king_pseudo_legal_moves = Vec::new();

    // TODO: Check that castling paths are not attacked.
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
                                // Capturing move.
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

            // Add castling moves to the normal moves.

            // MAGIC NUMBERS: These are masks for the squares between E8 and the corners.
            // Conditions:
            // - Correct side castling rights
            // - No friendly pieces in the way
            // - No enemy pieces in the way
            // - Rook present and not captured
            if castling_rights.contains("k")
                && friendly_pieces & 0x06000000_00000000 == 0
                && enemy_pieces & 0x06000000_00000000 == 0
                && friendly_rooks & Square::H8.to_u64() != 0
            {
                // Kingside castling (black)
                king_pseudo_legal_moves.push((
                    PieceType::King,
                    Square::E8,
                    Square::G8,
                    MoveType::KingCastle,
                ));
            }
            // Conditions: ditto.
            if castling_rights.contains("q")
                && friendly_pieces & 0x70000000_00000000 == 0
                && enemy_pieces & 0x70000000_00000000 == 0
                && friendly_rooks & Square::A8.to_u64() != 0
            {
                // Queenside castling (black)
                king_pseudo_legal_moves.push((
                    PieceType::King,
                    Square::E8,
                    Square::B8,
                    MoveType::QueenCastle,
                ));
            }
        }
        Color::White => {
            for king in king_locations {
                for r in movetable.get_moves(Color::White, PieceType::King, king) {
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
                                // Capturing move.
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

            // Add castling moves to the normal moves.

            // MAGIC NUMBERS: These are masks for the squares between E8 and the corners.
            // Conditions:
            // - Correct side castling rights
            // - No friendly pieces in the way
            // - No enemy pieces in the way
            // - Rook present and not captured
            if castling_rights.contains("K")
                && friendly_pieces & 0x06 == 0
                && enemy_pieces & 0x06 == 0
                && friendly_rooks & Square::H1.to_u64() != 0
            {
                // Kingside castling (black)
                king_pseudo_legal_moves.push((
                    PieceType::King,
                    Square::E1,
                    Square::G1,
                    MoveType::KingCastle,
                ));
            }
            // Conditions: ditto.
            if castling_rights.contains("Q")
                && friendly_pieces & 0x70 == 0
                && enemy_pieces & 0x70 == 0
                && friendly_rooks & Square::A1.to_u64() != 0
            {
                // Queenside castling (black)
                king_pseudo_legal_moves.push((
                    PieceType::King,
                    Square::E1,
                    Square::B1,
                    MoveType::QueenCastle,
                ));
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
            0,
            CastlingRecord {
                black: CastlingRights::Neither,
                white: CastlingRights::Neither,
            },
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

    #[test]
    fn check_black_king_pslm_castling() {
        let pslnm = kings::pseudolegal_king_moves(
            Color::Black,
            &MoveTable::default(),
            vec![E8.to_u64()],
            0,
            Square::A8.to_u64() | Square::H8.to_u64(),
            0,
            CastlingRecord {
                black: CastlingRights::Both,
                white: CastlingRights::Neither,
            },
        );
        let moves: HashSet<u64> = HashSet::from_iter(
            vec![
                D8.to_u64(),
                F8.to_u64(),
                D7.to_u64(),
                E7.to_u64(),
                F7.to_u64(),
                B8.to_u64(), // Queen-side castling.
                G8.to_u64(), // King-side castling.
            ]
            .iter()
            .cloned(),
        );

        assert!(pslnm.iter().all(|m| moves.contains(&m.2.to_u64())));
        assert_eq!(pslnm.len(), moves.len())
    }

    #[test]
    fn check_white_king_pslm_castling() {
        let pslnm = kings::pseudolegal_king_moves(
            Color::White,
            &MoveTable::default(),
            vec![E1.to_u64()],
            0,
            Square::A1.to_u64() | Square::H1.to_u64(),
            0,
            CastlingRecord {
                black: CastlingRights::Neither,
                white: CastlingRights::Both,
            },
        );
        let moves: HashSet<u64> = HashSet::from_iter(
            vec![
                D1.to_u64(),
                F1.to_u64(),
                D2.to_u64(),
                E2.to_u64(),
                F2.to_u64(),
                B1.to_u64(), // Queen-side castling.
                G1.to_u64(), // King-side castling.
            ]
            .iter()
            .cloned(),
        );

        assert!(pslnm.iter().all(|m| moves.contains(&m.2.to_u64())));
        assert_eq!(pslnm.len(), moves.len())
    }
}
