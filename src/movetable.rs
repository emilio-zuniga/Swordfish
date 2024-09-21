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
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for i in 0..8_usize {
            for j in 0..8_usize {
                table.insert((PieceType::Bishop, shift), bishop_move_rays((i, j)));
                shift >>= 1;
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
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for i in 0..8_usize {
            for j in 0..8_usize {
                table.insert((PieceType::WhiteKing, shift), king_move_rays((i, j)));
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for i in 0..8_usize {
            for j in 0..8_usize {
                table.insert((PieceType::Knight, shift), knight_move_hops((i, j)));
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for i in 0..8_usize {
            for j in 0..8_usize {
                table.insert((PieceType::BlackPawn, shift), black_pawn_moves((i, j)));
                shift >>= 1;
            }
        }

        shift = 0x8000000000000000; // Piece in the top left corner.
        for i in 0..8_usize {
            for j in 0..8_usize {
                table.insert((PieceType::WhitePawn, shift), white_pawn_moves((i, j)));
                shift >>= 1;
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
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
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
        while cx < 8 && cy < 8 {
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
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
                        }
                    }
                }
                moves.push(u64::from_str_radix(&bitstr, 2).unwrap()); // TODO: Watch out for this.
            }
        }
    }

    moves
}

/// Return the possible moves of a king on the given square, ignoring castling and other special moves.
fn king_move_rays(square: (usize, usize)) -> Vec<u64> {
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
    for i in 0..8_usize {
        for j in 0..8_usize {
            let mut bitstr = String::new();
            if board[i][j] == 1 {
                for k in 0..8_usize {
                    for l in 0..8_usize {
                        if i == k && j == l {
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
                        }
                    }
                }
                moves.push(u64::from_str_radix(&bitstr, 2).unwrap()); // TODO: Watch out for this.
            }
        }
    }

    moves
}

/// Returns the possible moves of a Knight on the given square.
fn knight_move_hops(square: (usize, usize)) -> Vec<u64> {
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

    let mut moves = Vec::new();
    let position = 1 << ((7 - square.0) * 8 + (7 - square.1));

    // North: North West Square
    if square.0 > 0 && square.1 > 1 {
        moves.push(position << 17);
    }
    // North: North East Square
    if square.0 < 7 && square.1 > 1 {
        moves.push(position << 15);
    }
    //North: West-most Square
    if square.0 > 1 && square.1 > 0 {
        moves.push(position << 10);
    }
    //North: East-most Square
    if square.0 < 6 && square.1 > 0 {
        moves.push(position << 6);
    }
    //South: West-Most Square
    if square.0 > 1 && square.1 < 7 {
        moves.push(position >> 6);
    }
    //South: East-Most Square
    if square.0 < 6 && square.1 < 7 {
        moves.push(position >> 10);
    }
    //South: South West Square
    if square.0 > 0 && square.1 < 6 {
        moves.push(position >> 15);
    }
    //South: South East Square
    if square.0 < 7 && square.1 < 6 {
        moves.push(position >> 17);
    }

    moves
}

/// Assumes black starts at the top of the board.
fn black_pawn_moves(square: (usize, usize)) -> Vec<u64> {
    // For square = (1, 1)...
    //   0 1 2 3 4 5 6 7 i
    // 0
    // 1   p
    // 2 . . .
    // 3 ␣ . ␣
    // 4 ╰───┴─╢ Only on captures!
    // 5   ␣
    // 6   ╰───╢ Only from the second rank!
    // 7
    // j

    let mut board = [[0_u64; 8]; 8];
    let directions: [(isize, isize); 4] = [(1, -1), (1, 0), (1, 1), (2, 0)];

    for (dx, dy) in directions {
        if dx == 0 && dy == 2 {
            if square.1 == 1 {
                let cx = (square.0 as isize + dx) as usize;
                let cy = (square.1 as isize + dy) as usize;
                if cx < 8 && cy < 8 {
                    board[cx][cy] = 1;
                }
            }
        } else {
            let cx = (square.0 as isize + dx) as usize;
            let cy = (square.1 as isize + dy) as usize;
            if cx < 8 && cy < 8 {
                board[cx][cy] = 1;
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
                            bitstr.push('1');
                        } else {
                            bitstr.push('0');
                        }
                    }
                }
                moves.push(u64::from_str_radix(&bitstr, 2).unwrap()); // TODO: Watch out for this.
            }
        }
    }

    moves
}

/// Returns a 'Vec<u64>' containing the possible moves of a white pawn, including standard captures.\
/// **NOTE**: invalid starting pawn spaces, such as those on the 1st & 8th ranks, return an empty vector
fn white_pawn_moves(square: (usize, usize)) -> Vec<u64> {
    let mut moves = Vec::new();
    let position: u64 = 1 << ((7 - square.0) * 8 + (7 - square.1));

    let a_file: u64 = 0x80808080_80808080;
    let h_file: u64 = 0x01010101_01010101;
    let rank_2: u64 = 0x00000000_0000FF00;
    let rank_3_thru_7: u64 = 0x00FFFFFF_FFFF0000;

    if (position & (rank_2 | rank_3_thru_7)) == position {
        moves.push(position << 8);
        if (position & rank_2) == position {
            moves.push(position << 16);
        }
        if (position & a_file) != position {
            moves.push(position << 9);
        }
        if (position & h_file) != position {
            moves.push(position << 7);
        } 
    }

    moves
}