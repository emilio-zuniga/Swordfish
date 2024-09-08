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
    fn from_fen_string(fen: &str) -> Self {
        todo!()
        //it might be a good idea to create a game manager that handles
        //turn movement and captured pieces, as well as castling rights
        //and the like, only passing in the board representation of the FEN
        
    }

    fn to_fen_string(&self) -> String {
        todo!()
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


    }
}
