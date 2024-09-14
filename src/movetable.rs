use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MoveTable {
    pub table: HashMap<(PieceType, u64), Vec<u64>>,
}

impl Default for MoveTable {
    fn default() -> Self {
        let mut table: HashMap<(PieceType, u64), Vec<u64>> = HashMap::new();

        let mut shift = 0x8000000000000000; // Piece in the top left corner.
        for i in 0..8_usize {
            for j in 0..8_usize {
                table.insert((PieceType::Rook, shift), rook_move_rays((i, j)));
                shift = shift >> 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for i in 0..8_usize {
            for j in 0..8_usize {
                table.insert((PieceType::Bishop, shift), bishop_move_rays((i, j)));
                shift = shift >> 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for i in 0..8_usize {
            for j in 0..8_usize {
                table.insert(
                    (PieceType::Queen, shift),
                    rook_move_rays((i, j))
                        .into_iter()
                        .chain(bishop_move_rays((i, j)))
                        .collect(),
                );
                shift = shift >> 1;
            }
        }

        MoveTable { table }
    }
}

/// An `enum` to represent which type the piece is. This provides indexing for our hash table of moves.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum PieceType {
    Queen,
    Rook,
    Bishop,
    Knight,
    BlackKing,
    WhiteKing,
    BlackPawn,
    WhitePawn,
}

/// Generate all possible locations reachable by a rook from the given
/// square, where the input tuple is an xy coord. taking the origin to
/// be the top left of the board.
pub fn rook_move_rays(square: (usize, usize)) -> Vec<u64> {
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
    for i in 0..8_usize {
        for j in 0..8_usize {
            if i == square.0 || j == square.1 {
                board[i][j] = 1;
            } else {
                board[i][j] = 0;
            }
        }
    }

    board[square.0][square.1] = 0; // Can't move to the current square.

    let mut moves = vec![];

    for i in 0..8_usize {
        for j in 0..8_usize {
            let mut bitstr = String::new();
            if board[i][j] == 1 {
                for k in 0..8_usize {
                    for l in 0..8_usize {
                        if i == k && j == l {
                            bitstr.push_str("1");
                        } else {
                            bitstr.push_str("0");
                        }
                    }
                }
                moves.push(u64::from_str_radix(&bitstr, 2).unwrap()); // TODO: Watch out for this.
            }
        }
    }

    moves
}

pub fn bishop_move_rays(square: (usize, usize)) -> Vec<u64> {
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
        while 0 <= cx && cx < 8 && 0 <= cy && cy < 8 {
            board[cx][cy] = 1;
            cx = (cx as isize + dx) as usize;
            cy = (cy as isize + dy) as usize;
        }
    }

    board[square.0][square.1] = 0; // Can't move to the current square.

    let mut moves = vec![];

    for i in 0..8_usize {
        for j in 0..8_usize {
            let mut bitstr = String::new();
            if board[i][j] == 1 {
                for k in 0..8_usize {
                    for l in 0..8_usize {
                        if i == k && j == l {
                            bitstr.push_str("1");
                        } else {
                            bitstr.push_str("0");
                        }
                    }
                }
                moves.push(u64::from_str_radix(&bitstr, 2).unwrap()); // TODO: Watch out for this.
            }
        }
    }

    moves
}
