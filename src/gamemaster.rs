/// This is a representation of a chess game and the various states of each element.
pub struct GameManager {
    castle_kingside_white: bool,
    castle_queenside_white: bool,
    castle_kingside_black: bool,
    castle_queenside_black: bool,
    //choose data type {String, Vec}? for En Passant targets
    white_to_move: bool,
    halfmoves: u32,
    total_turns: u32,
    bitboard: BitBoard,
}

impl Default for GameManager {
    fn default() -> Self {
        GameManager {
            castle_kingside_white: true,
            castle_queenside_white: true,
            castle_kingside_black: true,
            castle_queenside_black: true,
            //initialize to none
            white_to_move: true,
            halfmoves: 0,
            total_turns: 0,
            bitboard: BitBoard::default(),
        }
    }
}