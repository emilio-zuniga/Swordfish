use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Offset(u64, Direction);

/// Indicates the direction in which an offset should be applied.
#[derive(Debug, Clone)]
pub enum Direction {
    /// Use a left-shift operation.
    Shl,
    /// Use a right-shift operation.
    Shr,
}

#[derive(Debug, Clone)]
pub struct MoveTable {
    table: HashMap<PieceType, ([u64; 64], Vec<MoveRays>)>,
}

impl Default for MoveTable {
    fn default() -> Self {
        // Manually create a MoveTable.
        todo!()
    }
}

/// An `enum` to represent which type the piece is. This provides indexing for our hash table of moves.
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum MoveRays {
    Queen { rays: [Vec<u64>; 8] },
    Rook { rays: [Vec<u64>; 4] },
    Bishop { rays: [Vec<u64>; 4] },
    King { rays: [Vec<u64>; 8] },
    Knight { rays: [Vec<u64>; 8] },
    BlackPawn { rays: [Vec<u64>; 8] },
    WhitePawn { rays: [Vec<u64>; 8] },
}

pub fn rook_move_rays(square: (usize, usize)) -> Vec<MoveRays> {
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

    let bitpos = 0x8000000000000000_u64; // Top left.

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

    let mut bitstring = String::new();
    for i in 0..8_usize {
        for j in 0..8_usize {
            bitstring.push_str(board[i][j].to_string().as_str());
        }
        bitstring.push_str("\n");
    }

    dbg!(&bitstring);

    let mut rays: [Vec<u64>; 4];
    // The left ray is from board[0][square.1] to board[square.0][square.1].
    // The right ray is ``  board[square.0][square.1] to board[7][square.1].
    // The upwards      ``  board[square.0][0] to board[square.0][square.1].
    // The downwards    ``  board[square.0][square.1] to board[square.0][7].

    let mut leftray = vec![];
    for i in (0..=square.0).rev() {
        // Compute the offsets needed to reach these squares.
        if i <= square.0 {
            leftray.push(bitpos << i);
        }
    }

    let mut rightray = vec![];
    for i in square.0..7 {
        if i > square.0 {
            rightray.push(bitpos >> i);
        }
    }

    dbg!(&leftray, &rightray);

    todo!()
}
