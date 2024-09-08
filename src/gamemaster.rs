use crate::bitboard;
use bitboard::BitBoard;

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
    //GameManager was created to keep track of data about a game not
    //represented by a bitboard, such castling rights, player's turn
    //to move, and other data relevant to the game
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

impl GameManager {
    fn from_fen_string(fen: &str) -> Self {
        todo!();
        //will need to pass the board representation part of the
        //string to BitBoard::from_fen_string
    }

    fn to_fen_string(&self) -> String {
        todo!();
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
    }
}