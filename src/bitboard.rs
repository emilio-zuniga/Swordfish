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
            pawns_white: todo!(),
            rooks_white: u64,
            knights_white: u64,
            bishops_white: u64,
            queens_white: u64,
            king_white: u64,
            pawns_black: 0x00FF0000_00000000,
            rooks_black: 0xF1000000_00000000,
            knights_black: 0x82000000_00000000,
            bishops_black: 0x43000000_00000000,
            queens_black: 0x01000000_00000000,
            king_black: 0x01000000_00000000,
        }
    }
}

impl BitBoard {
    fn from_fen_string(fen: &str) -> Self {
        todo!()
    }

    fn to_fen_string(&self) -> String {
        todo!()
    }
}