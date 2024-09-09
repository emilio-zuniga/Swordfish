use crate::bitboard;
use bitboard::BitBoard;

/// This is a representation of a chess game and the various states of each element.
pub struct GameManager {
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

        //will need to call bitboard.to_fen() and append data
        //GM has stored to the end of it
    bitboard: BitBoard,
    white_to_move: bool,
    castling_rights: String,
    en_passant_targets: Vec<String>,
    halfmoves: u32,
    fullmoves: u32,
}

impl Default for GameManager {
    /// Constructs a new `GameMaster`, set to Chess's starting position
    fn default() -> Self {
        GameManager {
            bitboard: BitBoard::default(),
            white_to_move: true,
            castling_rights: String::new(),
            en_passant_targets: Vec::new(),
            halfmoves: 0,
            fullmoves: 0,
        }
    }
}

impl GameManager {
    /// A utility method for generating a new `GameMaster` from a FEN string\
    /// * `fen` - a `&str` representing a game's state in FEN
    /// * `returns` - a `GameMaster` as generated from the FEN
    fn from_fen_string(fen: &str) -> Self {
        /* split fen into tokens
         * grab board - pass it into BitBoard::from_fen_string();
         * match next token - white_to_move = true if "w", false if "b", otherwise, default to true
         * castling_rights = next toke 
         * 
         */
        todo!();
        //will need to pass the board representation part of the
        //string to BitBoard::from_fen_string
    }

    fn to_fen_string(&self) -> String {
        todo!();
    }
}