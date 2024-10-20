use crate::types::{Color, PieceType};
use std::collections::HashMap;

/// A HashMap of [`(Color, PieceType, u64)`] indexing [`Vec<Vec<u64>>`] where
/// the index integer is a position on the board (must be a power of two) and
/// the list of lists is a list of rays---that is, each direction the object
/// can move in is a separate list. This facilitates move legality checking,
/// because sliding pieces simply start at the head of the list and work out.
pub struct MoveTable {
    table: HashMap<(Color, PieceType, u64), Vec<Vec<u64>>>,
}

impl Default for MoveTable {
    fn default() -> Self {
        let mut table: HashMap<(Color, PieceType, u64), Vec<Vec<u64>>> = HashMap::new();

        let mut shift = 0x8000000000000000; // Piece in the top left corner.
        for y in 0..8_usize {
            for x in 0..8_usize {
                table.insert(
                    (Color::White, PieceType::Knight, shift),
                    knight_move_hops((x, y)),
                );
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for y in 0..8_usize {
            for x in 0..8_usize {
                table.insert(
                    (Color::White, PieceType::Bishop, shift),
                    bishop_move_rays((x, y)),
                );
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for y in 0..8_usize {
            for x in 0..8_usize {
                table.insert(
                    (Color::White, PieceType::Rook, shift),
                    rook_move_rays((x, y)),
                );
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for y in 0..8_usize {
            for x in 0..8_usize {
                table.insert(
                    (Color::White, PieceType::Queen, shift),
                    rook_move_rays((x, y))
                        .into_iter()
                        .chain(bishop_move_rays((x, y)))
                        .collect(),
                );
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for y in 0..8_usize {
            for x in 0..8_usize {
                table.insert(
                    (Color::White, PieceType::King, shift),
                    king_move_rays((x, y)),
                );
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for y in 0..8_usize {
            for x in 0..8_usize {
                table.insert(
                    (Color::Black, PieceType::King, shift),
                    king_move_rays((x, y)),
                );
                shift >>= 1;
            }
        }

        shift = 0x0080000000000000; // Piece on a7
        for y in 1..7_usize {
            for x in 0..8_usize {
                table.insert(
                    (Color::White, PieceType::Pawn, shift),
                    white_pawn_moves((x, y)),
                );
                shift >>= 1;
            }
        }

        shift = 0x0080000000000000; // Piece on a7
        for y in 1..7_usize {
            for x in 0..8_usize {
                table.insert(
                    (Color::Black, PieceType::Pawn, shift),
                    black_pawn_moves((x, y)),
                );
                shift >>= 1;
            }
        }

        MoveTable { table }
    }
}

/// Generate all possible locations reachable by a rook from the given
/// square, where the input tuple is an xy coord. taking the origin to
/// be the top left of the board.
/// * `square` - the xy coordinates of the piece
/// * `returns` - a `Vec<u64>` containing each pseudo legal move possible from that coordinate
fn rook_move_rays(square: (usize, usize)) -> Vec<Vec<u64>> {
    // For square = (0, 0)...
    //   0 1 2 3 4 5 6 7 i
    // 0 q . . . . . . .
    // 1 . .
    // 2 .   .
    // 3 .     .
    // 4 .       .
    // 5 .         .
    // 6 .           .
    // 7 .             .
    // j

    // To calculate the bit position given an x-y coord, let the x-val be the base, and shift it by eight for each row.
    let mut board = [[0_u64; 8]; 8];
    for x in 0..8_usize {
        for y in 0..8_usize {
            if x == square.0 || y == square.1 {
                board[x][y] = 1;
            } else {
                board[x][y] = 0;
            }
        }
    }

    board[square.0][square.1] = 0; // Can't move to the current square.

    let mut moves = vec![];

    let mut up_moves = vec![];
    for y in (0..square.1).rev() {
        let x = square.0;
        let mut bitstr = String::new();
        if board[x][y] == 1 {
            for l in 0..8_usize {
                for k in 0..8_usize {
                    if x == k && y == l {
                        bitstr.push('1');
                    } else {
                        bitstr.push('0');
                    }
                }
            }
            up_moves.push(u64::from_str_radix(&bitstr, 2).unwrap()); // TODO: Watch out for this.
        }
    }
    moves.push(up_moves);

    let mut down_moves = vec![];
    for y in square.1..8 {
        let x = square.0;
        let mut bitstr = String::new();
        if board[x][y] == 1 {
            for l in 0..8_usize {
                for k in 0..8_usize {
                    if x == k && y == l {
                        bitstr.push('1');
                    } else {
                        bitstr.push('0');
                    }
                }
            }
            down_moves.push(u64::from_str_radix(&bitstr, 2).unwrap()); // TODO: Watch out for this.
        }
    }
    moves.push(down_moves);

    let mut left_moves = vec![];
    for x in (0..square.0).rev() {
        let y = square.1;
        let mut bitstr = String::new();
        if board[x][y] == 1 {
            for l in 0..8_usize {
                for k in 0..8_usize {
                    if x == k && y == l {
                        bitstr.push('1');
                    } else {
                        bitstr.push('0');
                    }
                }
            }
            left_moves.push(u64::from_str_radix(&bitstr, 2).unwrap());
        }
    }
    moves.push(left_moves);

    let mut right_moves = vec![];
    for x in square.0..8 {
        let y = square.1;
        let mut bitstr = String::new();
        if board[x][y] == 1 {
            for l in 0..8_usize {
                for k in 0..8_usize {
                    if x == k && y == l {
                        bitstr.push('1');
                    } else {
                        bitstr.push('0');
                    }
                }
            }
            right_moves.push(u64::from_str_radix(&bitstr, 2).unwrap()); // TODO: Watch out for this.
        }
    }
    moves.push(right_moves);

    moves
}

/// Generate all possible locations reachable by a bishop from the given
/// square, where the input tuple is an xy coord. taking the origin to
/// be the top left of the board.
/// * `square` - the xy coordinates of the piece
/// * `returns` - a `Vec<u64>` containing each pseudo legal move possible from that coordinate
fn bishop_move_rays(square: (usize, usize)) -> Vec<Vec<u64>> {
    // For square = (1, 1)...
    //   0 1 2 3 4 5 6 7 i
    // 0 .   .
    // 1   b
    // 2 .   .
    // 3       .
    // 4         .
    // 5           .
    // 6             .
    // 7               .
    // j

    let directions: [(isize, isize); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];

    // To calculate the bit position given an x-y coord, let the x-val be the base, and shift it by eight for each row.
    let mut board = [[0_u64; 8]; 8];
    for (dx, dy) in directions {
        let mut cx = (square.0 as isize + dx) as usize;
        let mut cy = (square.1 as isize + dy) as usize;
        while cx < 8 && cy < 8 {
            board[cx][cy] = 1;
            cx = (cx as isize + dx) as usize;
            cy = (cy as isize + dy) as usize;
        }
    }

    board[square.0][square.1] = 0; // Can't move to the current square.

    let mut moves = vec![];

    let mut upper_left_moves = vec![];
    for y in (0..square.1).rev() {
        for x in (0..square.0).rev() {
            let mut bitstr = String::new();
            if board[x][y] == 1 {
                for l in 0..8_usize {
                    for k in 0..8_usize {
                        if x == k && y == l {
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
                        }
                    }
                }
                upper_left_moves.push(u64::from_str_radix(&bitstr, 2).unwrap());
            }
        }
    }
    moves.push(upper_left_moves);

    let mut lower_left_moves = vec![];
    for y in (0..square.1 + 1).rev() {
        for x in (0..square.0).rev() {
            let mut bitstr = String::new();
            if board[x][y] == 1 {
                for l in 0..8_usize {
                    for k in 0..8_usize {
                        if x == k && y == l {
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
                        }
                    }
                }
                lower_left_moves.push(u64::from_str_radix(&bitstr, 2).unwrap());
            }
        }
    }
    moves.push(lower_left_moves);

    let mut upper_right_moves = vec![];
    for y in (0..square.1).rev() {
        for x in square.0 + 1..8 {
            let mut bitstr = String::new();
            if board[x][y] == 1 {
                for l in 0..8_usize {
                    for k in 0..8_usize {
                        if x == k && y == l {
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
                        }
                    }
                }
                upper_right_moves.push(u64::from_str_radix(&bitstr, 2).unwrap());
            }
        }
    }
    moves.push(upper_right_moves);

    let mut lower_right_moves = vec![];
    for y in square.1 + 1..8 {
        for x in square.0 + 1..8 {
            let mut bitstr = String::new();
            if board[x][y] == 1 {
                for l in 0..8_usize {
                    for k in 0..8_usize {
                        if x == k && y == l {
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
                        }
                    }
                }
                lower_right_moves.push(u64::from_str_radix(&bitstr, 2).unwrap());
            }
        }
    }
    moves.push(lower_right_moves);

    moves
}

/// Return the possible moves of a king on the given square, ignoring castling and other special moves.
/// * `square` - the xy coordinates of the piece
/// * `returns` - a [`Vec<Vec<u64>>`] containing each pseudo legal move possible from that coordinate
fn king_move_rays(square: (usize, usize)) -> Vec<Vec<u64>> {
    // For square = (1, 1)...
    //   0 1 2 3 4 5 6 7 i
    // 0 . . .
    // 1 . k .
    // 2 . . .
    // 3
    // 4
    // 5
    // 6
    // 7
    // j

    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];
    let mut board = [[0_u64; 8]; 8];

    for (dx, dy) in directions {
        let cx = (square.0 as isize + dx) as usize;
        let cy = (square.1 as isize + dy) as usize;
        if cx < 8 && cy < 8 {
            board[cx][cy] = 1;
        }
    }

    let mut moves = vec![];

    for y in 0..8_usize {
        for x in 0..8_usize {
            let mut bitstr = String::new();
            if board[x][y] == 1 {
                for l in 0..8_usize {
                    for k in 0..8_usize {
                        if x == k && y == l {
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
                        }
                    }
                }
                moves.push(vec![u64::from_str_radix(&bitstr, 2).unwrap()]); // TODO: Watch out for this.
            }
        }
    }

    moves
}

/// Returns the possible moves of a Knight on the given square.
/// * `square` - the xy coordinates of the piece
/// * `returns` - a `Vec<u64>` containing each pseudo legal move possible from that coordinate
fn knight_move_hops(square: (usize, usize)) -> Vec<Vec<u64>> {
    //square.0 = x-coord
    //square.1 = y-coord
    // For square = (1, 1)...
    //   0 1 2 3 4 5 6 7 i
    // 0       .
    // 1   n
    // 2       .
    // 3 .   .
    // 4
    // 5
    // 6
    // 7
    // j

    let mut moves: Vec<Vec<u64>> = Vec::new();
    // Explicitly specify behavior when shl-ing by more than the no. of bits in lhs, i.e. 64.
    let position = (1_u64.checked_shl(((7 - square.1) * 8 + (7 - square.0)) as u32)).unwrap_or(0);

    // North: North West Square
    if square.0 > 0 && square.1 > 1 {
        moves.push(vec![position << 17]);
    }
    // North: North East Square
    if square.0 < 7 && square.1 > 1 {
        moves.push(vec![position << 15]);
    }
    //North: West-most Square
    if square.0 > 1 && square.1 > 0 {
        moves.push(vec![position << 10]);
    }
    //North: East-most Square
    if square.0 < 6 && square.1 > 0 {
        moves.push(vec![position << 6]);
    }
    //South: West-Most Square
    if square.0 > 1 && square.1 < 7 {
        moves.push(vec![position >> 6]);
    }
    //South: East-Most Square
    if square.0 < 6 && square.1 < 7 {
        moves.push(vec![position >> 10]);
    }
    //South: South West Square
    if square.0 > 0 && square.1 < 6 {
        moves.push(vec![position >> 15]);
    }
    //South: South East Square
    if square.0 < 7 && square.1 < 6 {
        moves.push(vec![position >> 17]);
    }

    moves
}

/// A function generating the pseudo legal pushes and captures of a black pawn at a given position\
/// Invalid starting spaces, such as those on the 1st & 8th ranks, return an empty vector\
/// * `square` - the x and y coordinates of the piece's position\
/// * `returns` - a `Vec<u64>` containing each pseudo legal move of the pawn possible from that square
fn black_pawn_moves(square: (usize, usize)) -> Vec<Vec<u64>> {
    let mut moves = Vec::new();
    let position: u64 = 1 << ((7 - square.1) * 8 + (7 - square.0));

    let a_file: u64 = 0x80808080_80808080;
    let h_file: u64 = 0x01010101_01010101;
    let rank_7: u64 = 0x00FF0000_00000000;
    let rank_2_thru_6: u64 = 0x0000FFFF_FFFFFF00;

    if (position & (rank_2_thru_6 | rank_7)) == position {
        moves.push(vec![position >> 8]);

        if (position & rank_7) == position {
            moves.push(vec![position >> 16]);
        }
        if (position & a_file) != position {
            moves.push(vec![position >> 7]);
        }
        if (position & h_file) != position {
            moves.push(vec![position >> 9]);
        }
    }

    moves
}

/// A function generating the pseudo legal pushes and captures of a white pawn at a given position\
/// Invalid starting spaces, such as those on the 1st & 8th ranks, return an empty vector\
/// * `square` - the x and y coordinates of the piece's position\
/// * `returns` - a `Vec<u64>` containing each pseudo legal move of the pawn possible from that square
fn white_pawn_moves(square: (usize, usize)) -> Vec<Vec<u64>> {
    let mut moves = Vec::new();
    let position: u64 = 1 << ((7 - square.1) * 8 + (7 - square.0));

    let a_file: u64 = 0x80808080_80808080;
    let h_file: u64 = 0x01010101_01010101;
    let rank_2: u64 = 0x00000000_0000FF00;
    let rank_3_thru_7: u64 = 0x00FFFFFF_FFFF0000;

    if (position & (rank_2 | rank_3_thru_7)) == position {
        moves.push(vec![position << 8]);
        if (position & rank_2) == position {
            moves.push(vec![position << 16]);
        }
        if (position & a_file) != position {
            moves.push(vec![position << 9]);
        }
        if (position & h_file) != position {
            moves.push(vec![position << 7]);
        }
    }

    moves
}

impl MoveTable {
    /// Given a single piece position & the type, return the possible moves as a [`Vec<Vec<u64>>`].
    pub fn moves(&self, color: Color, piece: PieceType, position: u64) -> Vec<Vec<u64>> {
        let res = match self.table.get(&(color, piece, position)) {
            Some(v) => v.clone(),
            None => Vec::new(),
        };

        res
    }

    /// A utility method for getting the possible moves of a piece at a given position\
    /// * `color` - the `Color` of the piece\
    /// * `piece` - the `PieceType`\
    /// * `square` - the x and y coordinates of the piece's position\
    /// * `returns` - a `Vec<u64>` containing each pseudo legal move of that piece possible from that square
    pub fn get_moves(
        &self,
        color: Color,
        piece: PieceType,
        square: (usize, usize),
    ) -> Vec<Vec<u64>> {
        let position = 1 << ((7 - square.1) * 8 + (7 - square.0));

        match piece {
            PieceType::Knight
            | PieceType::Bishop
            | PieceType::Rook
            | PieceType::Queen
            | PieceType::King => self
                .table
                .get(&(Color::White, piece, position))
                .unwrap()
                .clone(),
            PieceType::Pawn => self.table.get(&(color, piece, position)).unwrap().clone(),
        }
    }

    /// A utility method for getting the possible moves of a piece at a given position
    /// * `color` - the `Color` of the piece
    /// * `piece` - the `PieceType`
    /// * `square` - the x and y coordinates of the piece's position
    /// * `returns` - a `u64` bitboard representing the pseudo legal move of that piece possible from that square
    pub fn get_moves_as_bitboard(
        &self,
        color: Color,
        piece: PieceType,
        square: (usize, usize),
    ) -> u64 {
        let moverays = &self.get_moves(color, piece, square);
        let mut board = 0_u64;

        for ray in moverays {
            for possible_move in ray {
                board |= possible_move;
            }
        }

        board
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::MoveTable;

    #[test]
    fn check_knight_moves() {
        let table = MoveTable::default();
        let rays = table.get_moves(
            crate::types::Color::Black,
            crate::types::PieceType::Knight,
            (0, 1),
        );

        dbg!(&rays);

        let mut pslm: HashSet<u64> = HashSet::new();
        pslm.insert(0x80000000000000);
        pslm.insert(0x20000000000000);
        pslm.insert(0x4000000000000);
        pslm.insert(0x1000000000000);

        let all_are_members = rays.iter().all(|r| r.iter().all(|m| pslm.contains(m)));
        let only_four = rays
            .iter()
            .fold(0, |acc, r: &Vec<u64>| acc + r.iter().count());

        assert!(all_are_members);
        assert_eq!(only_four, 4);
    }
}
