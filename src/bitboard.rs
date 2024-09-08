/// This is a representation of the board. Each piece gets a [`u64`] integer.
pub struct BitBoard {
    pawns_white: u64,
    rooks_white: u64,
    knights_white: u64,
    bishops_white: u64,
    queens_white: u64,
    king_white: u64,
    pawns_black: u64,
    rooks_black: u64,
    knights_black: u64,
    bishops_black: u64,
    queens_black: u64,
    king_black: u64,
}

impl Default for BitBoard {
    fn default() -> Self {
        // Return a default BitBoard, i.e. a normal starting game.
        // Let's assemble one by bits for now. Later, we'll just use FEN.
        // Assume black starts at the top of the board. Every two hexadecimal digits
        // represents one row. Top rows are in the high bits.
        // BLANK: 0b0000000000000000
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
    /// A utility method for generating a `BitBoard` from a FEN string
    fn from_fen_string(fen: &str) -> Self {
        todo!()
    }

    /// A utility method generating a FEN string representation of this `BitBoard`
    fn to_fen_string(&self) -> String {
        let mut s = String::new();
        let board = self.to_board();

        s
    }

    /// **Debuggin** A utility method generating a `String` representation of this `BitBoard`
    fn to_string(&self) -> String {
        let mut s = String::new();
        let board = self.to_board();

        for row in board {
            s.push_str(&String::from_iter(row.iter()));
            s.push_str("\n");
        }

        s
    }

    /// A utility method creating a 2D `char` array representation of this `BitBoard`
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
            (self.king_black, 'k')
        ];
        
        for (piece_map, piece_type) in bitboards {
            for i in 0..64 {
                if piece_map & (1 << i) != 0 {
                    let r = 7 - (i/8);
                    let c = 7 - (i%8);
                    board[r][c] = piece_type;
                }
            }
        }

        board
    }
}