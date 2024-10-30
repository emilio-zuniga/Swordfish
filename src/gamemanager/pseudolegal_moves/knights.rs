use crate::{movetable::MoveTable, types::*};

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
/// use crate::{movetable::MoveTable, types::*, gamemanager::pseudolegal_moves::knights};
///
/// let pslnm = knights::pseudolegal_knight_moves(
///     Color::Black,
///     &MoveTable::default(),
///     vec![0x40000000_00000000],
///     0xFFFF0000_00000000,
///     0xFFFF,
/// );
/// let moves = HashSet::from_iter(vec![0x00008000_00000000, 0x00002000_00000000].iter().cloned());
/// assert!(pslnm.iter().all(|m| moves.contains(m)))
/// ```
pub fn pseudolegal_knight_moves(
    color: Color,
    movetable: &MoveTable,
    knight_locations: Vec<u64>,
    friendly_pieces: u64,
    enemy_pieces: u64,
) -> Vec<Move> {
    let mut knight_pseudo_legal_moves = Vec::new();

    for knight in knight_locations {
        for r in movetable.get_moves(color, PieceType::Knight, knight) {
            for m in r {
                if m & friendly_pieces == 0 {
                    // The knight's move does not intersect any friendly pieces

                    let from = Square::from_u64(knight).expect("Each u64 is a power of two");
                    let to = Square::from_u64(m).expect("Each u64 is a power of two");

                    if m & enemy_pieces != 0 {
                        // It's a capture move if the destination is occupied by an enemy piece
                        knight_pseudo_legal_moves.push((
                            PieceType::Knight,
                            from,
                            to,
                            MoveType::Capture,
                        ));
                    } else {
                        // It's a quiet move (no capture)
                        knight_pseudo_legal_moves.push((
                            PieceType::Knight,
                            from,
                            to,
                            MoveType::QuietMove,
                        ));
                    }
                }
            }
        }
    }

    knight_pseudo_legal_moves
}

#[cfg(test)]
mod tests {
    use crate::gamemanager::pseudolegal_moves::knights;
    use crate::{movetable::MoveTable, types::*};
    use std::collections::HashSet;

    #[test]
    fn check_knight_pslm() {
        let pslnm = knights::pseudolegal_knight_moves(
            Color::Black,
            &MoveTable::default(),
            vec![0x40000000_00000000],
            0xFFFF0000_00000000,
            0xFFFF,
        );
        let moves: HashSet<u64> = HashSet::from_iter(
            vec![0x00008000_00000000, 0x00002000_00000000]
                .iter()
                .cloned(),
        );
        assert!(pslnm.iter().all(|m| moves.contains(&m.2.to_u64())))
    }
}
