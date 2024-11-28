use crate::types::{Color, PieceType};

#[derive(Copy, Clone, Debug)]
/// This is a representation of the board. Each piece gets a [`u64`] integer.
pub struct BitBoard {
    pub pawns_white: u64,
    pub rooks_white: u64,
    pub knights_white: u64,
    pub bishops_white: u64,
    pub queens_white: u64,
    pub king_white: u64,
    pub pawns_black: u64,
    pub rooks_black: u64,
    pub knights_black: u64,
    pub bishops_black: u64,
    pub queens_black: u64,
    pub king_black: u64,
}

impl Default for BitBoard {
    /// Constructs a new `BitBoard`, set to Chess's starting position
    fn default() -> Self {
        // Return a default BitBoard, i.e. a normal starting game.
        // Let's assemble one by bits for now. Later, we'll just use FEN.
        // Assume black starts at the top of the board. Every two hexadecimal digits
        // represents one row. Top rows are in the high bits.
        // BLANK: 0b0000000000000000000000000000000000000000000000000000000000000000
        BitBoard {
            pawns_white: 0x00000000_0000FF00,
            rooks_white: 0x00000000_00000081,
            knights_white: 0x00000000_00000042,
            bishops_white: 0x00000000_00000024,
            queens_white: 0x00000000_00000010,
            king_white: 0x00000000_00000008,

            pawns_black: 0x00FF0000_00000000,
            rooks_black: 0x81000000_00000000,
            knights_black: 0x42000000_00000000,
            bishops_black: 0x24000000_00000000,
            queens_black: 0x10000000_00000000,
            king_black: 0x08000000_00000000,
        }
    }
}

impl BitBoard {
    /// Gets the total piece weights for a given side.
    pub fn piece_mass(&self, color: Color) -> i32 {
        match color {
            Color::Black => {
                self.king_black.count_ones() as i32 * PieceType::King as i32
                    + self.queens_black.count_ones() as i32 * PieceType::Queen as i32
                    + self.rooks_black.count_ones() as i32 * PieceType::Rook as i32
                    + self.bishops_black.count_ones() as i32 * PieceType::Bishop as i32
                    + self.knights_black.count_ones() as i32 * PieceType::Knight as i32
                    + self.pawns_black.count_ones() as i32 * PieceType::Pawn as i32
            }
            Color::White => {
                self.king_white.count_ones() as i32 * PieceType::King as i32
                    + self.queens_white.count_ones() as i32 * PieceType::Queen as i32
                    + self.rooks_white.count_ones() as i32 * PieceType::Rook as i32
                    + self.bishops_white.count_ones() as i32 * PieceType::Bishop as i32
                    + self.knights_white.count_ones() as i32 * PieceType::Knight as i32
                    + self.pawns_white.count_ones() as i32 * PieceType::Pawn as i32
            }
        }
    }

    /// A utility method for generating a `BitBoard` from a FEN string\
    /// * `fen` - a `&str` representing the board token of a FEN string\
    /// * `returns` - a `BitBoard` as generated from the FEN token
    pub fn from_fen_string(fen: &str) -> Self {
        let mut position: u64 = 0x80000000_00000000;
        let mut board = BitBoard {
            pawns_white: 0x0,
            rooks_white: 0x0,
            knights_white: 0x0,
            bishops_white: 0x0,
            queens_white: 0x0,
            king_white: 0x0,
            pawns_black: 0x0,
            rooks_black: 0x0,
            knights_black: 0x0,
            bishops_black: 0x0,
            queens_black: 0x0,
            king_black: 0x0,
        };

        for c in fen.chars() {
            match c {
                'P' => board.pawns_white |= position,
                'R' => board.rooks_white |= position,
                'N' => board.knights_white |= position,
                'B' => board.bishops_white |= position,
                'Q' => board.queens_white |= position,
                'K' => board.king_white |= position,

                'p' => board.pawns_black |= position,
                'r' => board.rooks_black |= position,
                'n' => board.knights_black |= position,
                'b' => board.bishops_black |= position,
                'q' => board.queens_black |= position,
                'k' => board.king_black |= position,
                '1'..='8' => position >>= c.to_digit(10).unwrap() - 1,
                _ => position <<= 1,
            }
            position >>= 1;
        }

        board
    }

    /// A utility method generating a FEN string representation of this `BitBoard`
    /// * `returns` - a `String` representing the board token of a string in FEN
    pub fn to_fen_string(&self) -> String {
        let mut s = String::new();
        let board = self.to_board();

        for row in board {
            let mut spaces: u8 = 0;
            for c in row {
                if c == 'P'
                    || c == 'N'
                    || c == 'B'
                    || c == 'R'
                    || c == 'Q'
                    || c == 'K'
                    || c == 'p'
                    || c == 'n'
                    || c == 'b'
                    || c == 'r'
                    || c == 'q'
                    || c == 'k'
                {
                    if spaces > 0 {
                        s.push((spaces + 48) as char);

                        spaces = 0;
                    }
                    s.push(c);
                } else {
                    spaces += 1;
                }
            }
            if spaces > 0 {
                s.push((spaces + 48) as char);
            }
            s.push('/');
        }
        s.pop();

        s
    }

    /// **Debugging** A utility method generating a `String` representation of this `BitBoard`
    /// * `returns` - a `String` representing the board
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let board = self.to_board();

        for row in board {
            s.push_str(&String::from_iter(row.iter()));
            s.push('\n');
        }

        s
    }

    /// A utility method creating a 2D `char` array representation of this `BitBoard`
    /// * `returns` - a `[[char; 8]; 8]` 2D array representing the board
    fn to_board(&self) -> [[char; 8]; 8] {
        let mut board = [['.'; 8]; 8];
        let bitboards = [
            (self.pawns_white, 'P'),
            (self.rooks_white, 'R'),
            (self.knights_white, 'N'),
            (self.bishops_white, 'B'),
            (self.queens_white, 'Q'),
            (self.king_white, 'K'),
            (self.pawns_black, 'p'),
            (self.rooks_black, 'r'),
            (self.knights_black, 'n'),
            (self.bishops_black, 'b'),
            (self.queens_black, 'q'),
            (self.king_black, 'k'),
        ];

        for (piece_map, piece_type) in bitboards {
            for i in 0..64 {
                if piece_map & (1 << i) != 0 {
                    let r = 7 - (i / 8);
                    let c = 7 - (i % 8);
                    board[r][c] = piece_type;
                }
            }
        }

        board
    }

    /// A utility method returning the bitboard representing the placement\
    /// of this `PieceType` of this `Color`\
    /// * `color` - the `Color` of the pieces\
    /// * `pieces` - the `PieceType` of the pieces
    #[allow(dead_code)]
    pub fn get_bitboard(&self, color: Color, piece: PieceType) -> u64 {
        match color {
            Color::White => match piece {
                PieceType::Pawn => self.pawns_white,
                PieceType::Knight => self.knights_white,
                PieceType::Bishop => self.bishops_white,
                PieceType::Rook => self.rooks_white,
                PieceType::Queen => self.queens_white,
                PieceType::King => self.king_white,
                PieceType::Super => 0,
            },
            Color::Black => match piece {
                PieceType::Pawn => self.pawns_black,
                PieceType::Knight => self.knights_black,
                PieceType::Bishop => self.bishops_black,
                PieceType::Rook => self.rooks_black,
                PieceType::Queen => self.queens_black,
                PieceType::King => self.king_black,
                PieceType::Super => 0,
            },
        }
    }
}
