use crate::bitboard;
use bitboard::BitBoard;

/// This is a representation of a chess game and the various states of each element.
pub struct GameManager {
    /*FEN Notes:
     * active color - get whose turn it is to move {w, b}
     * castling rights - castle-able sides {QKqk-}
     * possible En Passant targets - E.P. rules:
     *           1. capturing pawn must have adv 3 ranks to perform
     *           2. captured pawn must have moved 2 squares in one turn
     *           3. capture must be performed on turn immediately after
     *              the pawn being captured moves
     *           - if the above conditions are met, include the coordinate behind the
     *             pawn that just moved 2 spaces {a single coord on 4th or 5th rank}
     * halfmove clock - moves since the last piece capture/pawn adv {MAX 100}
     *           - game drawn when a counter reaches 100
     *           - increment if a player makes move that is not capture nor pawn move
     *           - reset to 0 if a player captures a piece or makes or pawn move
     * fullmove number - number of completed turns (increment when black moves)
     */
    bitboard: BitBoard,
    white_to_move: bool,
    castling_rights: String,
    en_passant_target: String,
    halfmoves: u32,
    fullmoves: u32,
}

impl Default for GameManager {
    /// Constructs a new `GameManager`, set to Chess's starting position
    fn default() -> Self {
        GameManager {
            bitboard: BitBoard::default(),
            white_to_move: true,
            castling_rights: String::from("KQkq"),
            en_passant_target: String::new(),
            halfmoves: 0,
            fullmoves: 1,
        }
    }
}

impl GameManager {
    /// A utility method for generating a new `GameManager` from a FEN string\
    /// * `fen` - a `&str` representing a game's state in FEN
    /// * `returns` - a `GameManager` as generated from the FEN
    pub fn from_fen_string(fen: &str) -> Self {
        let tokens: Vec<String> = fen.split_whitespace().map(str::to_string).collect();
        GameManager {
            //board space validation implemented at higher level (is_valid_fen())
            bitboard: BitBoard::from_fen_string(&tokens[0]),
            white_to_move: tokens[1] == "w",
            castling_rights: tokens[2].clone(),
            en_passant_target: tokens[3].clone(),
            halfmoves: tokens[4].parse().unwrap_or_default(),
            fullmoves: tokens[5].parse().unwrap_or_default(),
        }
    }
    
    /// A utility method generating a complete FEN string representation of the game
    /// * `returns` - a `String` representing the game state in FEN
    pub fn to_fen_string(&self) -> String {
        let mut s = self.bitboard.to_fen_string();
        s.push(' ');
        s.push(if self.white_to_move {'w'} else {'b'});
        s.push(' ');
        s.push_str(if self.castling_rights.is_empty() {"-"} else {&self.castling_rights});
        s.push(' ');
        s.push_str(if self.en_passant_target.is_empty() {"-"} else {&self.en_passant_target});
        s.push(' ');
        s.push_str(&self.halfmoves.to_string());
        s.push(' ');
        s.push_str(&self.fullmoves.to_string());

        s
    }
}