use crate::{
    movetable::{noarc::NoArc, MoveTable},
    types::*,
};

/// Returns all pseudolegal moves the knights can make from their positions.
/// ## Inputs
/// - color: [`Color`][Color] enum for the current player,
/// - movetable: reference to a [`MoveTable`][MoveTable],
/// - knight_locations: list of individual pieces,
/// - friendly_pieces: all friendly pieces and-ed together,
/// - enemy_pieces: ditto for enemies
///
/// ## Returns
/// Returns a list of pseudolegal moves with the type alias [`Move`][Move],
/// which expands to `(PieceType, Square, Square, MoveType)`.
///
/// ## Examples
/// ```rust
/// use crate::{movetable::MoveTable, types::*, gamemanager::pseudolegal_moves::bishops};
///
/// let pslnm = bishops::pseudolegal_bishop_moves(
///     Color::Black,
///     &MoveTable::default(),
///     vec![0x20000000_00000000],
///     0xFFAF5000_00000000,
///     0xFFFF,
/// );
/// let moves: HashSet<u64> = HashSet::from_iter(
///     vec![
///         0x00400000_00000000,
///         0x00100000_00000000,
///         0x00008000_00000000,
///         0x00000800_00000000,
///         0x00000004_00000000,
///         0x00000000_02000000,
///         0x00000000_00010000
///     ]
///     .iter()
///     .cloned(),
/// );
/// assert!(pslnm.iter().all(|m| moves.contains(&m.2.to_u64())));
/// assert_eq!(pslnm.len(), moves.len())
/// ```
pub fn pseudolegal_bishop_moves(
    color: Color,
    bishop_locations: Vec<u64>,
    friendly_pieces: u64,
    enemy_pieces: u64,
    movetable: &NoArc<MoveTable>,
) -> Vec<(PieceType, Square, Square, MoveType)> {
    let mut bishop_pseudo_legal_moves = Vec::new();

    for bishop in bishop_locations {
        debug_assert!(bishop.is_power_of_two()); // Must be a power of two.
        for r in movetable.get_moves(color, PieceType::Bishop, bishop) {
            for m in r {
                if m & friendly_pieces != 0 {
                    // If the move is blocked by a friendly piece, stop in this direction
                    break;
                } else {
                    let from = Square::from_u64(bishop).expect("Each u64 is a power of two");
                    let to = Square::from_u64(m).expect("Each u64 is a power of two");

                    if m & enemy_pieces != 0 {
                        // It's a capture move
                        bishop_pseudo_legal_moves.push((
                            PieceType::Bishop,
                            from,
                            to,
                            MoveType::Capture,
                        ));
                        // Stop after capturing the enemy piece
                        break;
                    } else {
                        // It's a quiet move (no piece in the way)
                        bishop_pseudo_legal_moves.push((
                            PieceType::Bishop,
                            from,
                            to,
                            MoveType::QuietMove,
                        ));
                    }
                }
            }
        }
    }

    bishop_pseudo_legal_moves
}

#[cfg(test)]
mod test {
    use crate::{
        gamemanager::pseudolegal_moves::bishops,
        movetable::{noarc::NoArc, MoveTable},
        types::*,
    };
    use std::collections::HashSet;

    #[test]
    fn check_bishop_pslm() {
        let pslnm = bishops::pseudolegal_bishop_moves(
            Color::Black,
            vec![0x20000000_00000000],
            0xFFAF5000_00000000,
            0xFFFF,
            &NoArc::new(MoveTable::default()),
        );
        let moves: HashSet<u64> = HashSet::from_iter(
            vec![
                0x00400000_00000000,
                0x00100000_00000000,
                0x00008000_00000000,
                0x00000800_00000000,
                0x00000004_00000000,
                0x00000000_02000000,
                0x00000000_00010000,
            ]
            .iter()
            .cloned(),
        );
        assert!(pslnm.iter().all(|m| moves.contains(&m.2.to_u64())));
        assert_eq!(pslnm.len(), moves.len())
    }
}
