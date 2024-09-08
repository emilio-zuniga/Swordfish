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
    /// **Utility** - A utility method for generating BitBoards from a FEN String
    fn from_fen_string(fen: &str) -> Self {
        todo!()
    }

    /// **Utility** - A utility method for creating a FEN String from a BitBoard
    fn to_fen_string(&self) -> String {
        //FEN Notes:
        //active color - get whose turn it is to move {w, b}
        //castling rights - castle-able sides {QKqk-}
        //possible En Passant targets - E.P. rules:
        //          1. capturing pawn must have adv 3 ranks to perfor
        //          2. captured pawn must have moved 2 squares in one turn
        //          3. capture must be performed on turn immediately after 
        //             the pawn being captured moves
        //          - if the above conditions are met, include the coordinate
        //            behind the pawn that just moved 2 spaces 
        //            {a single coord on 4th or 5th rank}
        //halfmove clock - moves since the last piece capture/pawn adv {MAX 100}
        //          - game drawn when a counter reaches 100
        //fullmove number - number of completed turns (increments when black moves) {probably u32}
        
        //All of the above will likely be implemented externally

        let mut s = String::new();
        let board = self.to_board();

        for row in board {
            s.push_str(&String::from_iter(row.iter()));
            s.push_str("\n");
        }

        s
    }

    /// **Utility** - A utility method for creating a 2D array representation from a bitboard
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