use crate::{
    bitboard,
    movetable::MoveTable,
    types::{Color, PieceType},
};
use bitboard::BitBoard;
use regex::Regex;

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
    movetable: MoveTable,
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
            movetable: MoveTable::default(),
            white_to_move: true,
            castling_rights: String::from("KQkq"),
            en_passant_target: String::new(),
            halfmoves: 0,
            fullmoves: 1,
        }
    }
}

#[allow(dead_code)]
impl GameManager {
    /// A utility method for generating a new `GameManager` from a FEN string\
    /// * `fen` - a `&str` representing a game's state in FEN
    /// * `returns` - a `GameManager` as generated from the FEN
    pub fn from_fen_string(fen: &str) -> Self {
        if Self::is_valid_fen(fen) {
            let tokens: Vec<String> = fen.split_whitespace().map(str::to_string).collect();
            GameManager {
                //board space validation implemented at higher level (is_valid_fen())
                bitboard: BitBoard::from_fen_string(&tokens[0]),
                movetable: MoveTable::default(),
                white_to_move: tokens[1] == "w",
                castling_rights: tokens[2].clone(),
                en_passant_target: tokens[3].clone(),
                halfmoves: tokens[4].parse().unwrap_or_default(),
                fullmoves: tokens[5].parse().unwrap_or_default(),
            }
        } else {
            GameManager::default()
        }
    }

    /// A utility method generating a complete FEN string representation of the game
    /// * `returns` - a `String` representing the game state in FEN
    pub fn to_fen_string(&self) -> String {
        let mut s = self.bitboard.to_fen_string();
        s.push(' ');
        s.push(if self.white_to_move { 'w' } else { 'b' });
        s.push(' ');
        s.push_str(if self.castling_rights.is_empty() {
            "-"
        } else {
            &self.castling_rights
        });
        s.push(' ');
        s.push_str(if self.en_passant_target.is_empty() {
            "-"
        } else {
            &self.en_passant_target
        });
        s.push(' ');
        s.push_str(&self.halfmoves.to_string());
        s.push(' ');
        s.push_str(&self.fullmoves.to_string());

        s
    }

    /// A utility function validating FEN strings
    /// * `returns` - a `bool` indicating whether or not the string follows FEN guidelines
    fn is_valid_fen(fen: &str) -> bool {
        let fen_regex_string = r"([PNBRQKpnbrqk1-8]{1,8}\/){7}[PNBRQKpnbrqk1-8]{1,8} [WBwb] ((K?Q?k?q)|(K?Q?kq?)|(K?Qk?q?)|(KQ?k?q?)|-) (([A-Ha-h][1-8])|-) \d+ \d+";
        let reggae = Regex::new(fen_regex_string).unwrap();
        let tokens: Vec<String> = fen.split_whitespace().map(str::to_string).collect();

        reggae.is_match(fen) && tokens.len() == 6 && {
            let ranks: Vec<String> = tokens[0].split('/').map(str::to_string).collect();
            for rank in ranks {
                let mut count: i32 = 8;
                if !rank.is_empty() {
                    for c in rank.chars() {
                        match c {
                            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q'
                            | 'k' => count -= 1,
                            '1'..='8' => count -= c.to_digit(10).unwrap() as i32,
                            _ => (),
                        }
                    }
                    if count != 0 {
                        return false;
                    }
                }
            }

            true
        }
    }

    // Implement fn get_board(piece: PieceType, color: Color) -> u64 {}

    pub fn pseudolegal_moves(&self, color: Color) -> () {
        let mut pseudolegal_moves: Vec<u64> = Vec::new();

        match color {
            Color::Black => {
                // For each black piece on the board, obtain its possible moves.
                // Each piece is a power of two, and we'll pop the powers of two with the function below.

                // Each power of two is passed to its respective MoveTable, and the resultant list of moves is matched against
                // the friendly_pieces integer. This way, invalid moves are filtered out.

                // This means our "pseudo-legal" moves include only valid moves, and moves that leave the king in check, or are not permitted by the rules of chess
                // for some reason besides intersection of pieces.

                // To get each black piece, pop each power of two for each piece type.
                let friendly_pieces = self.bitboard.pawns_black
                    | self.bitboard.rooks_black
                    | self.bitboard.knights_black
                    | self.bitboard.bishops_black
                    | self.bitboard.queens_black
                    | self.bitboard.king_black;
                let enemy_pieces = self.bitboard.pawns_white
                    | self.bitboard.rooks_white
                    | self.bitboard.knights_white
                    | self.bitboard.bishops_white
                    | self.bitboard.queens_white
                    | self.bitboard.king_white;

                let pawns = GameManager::powers_of_two(self.bitboard.pawns_black);
                let rooks = GameManager::powers_of_two(self.bitboard.rooks_black);
                let bishops = GameManager::powers_of_two(self.bitboard.bishops_black);
                let queens = GameManager::powers_of_two(self.bitboard.queens_black);
                let kings = GameManager::powers_of_two(self.bitboard.king_black);

                for p in pawns {
                    for m in self.movetable.moves(Color::Black, PieceType::Pawn, p) {
                        if m & friendly_pieces == 0 {
                            // ...then this move does not intersect any friendly pieces, and it can be played, ignoring king safety.
                            pseudolegal_moves.push(todo!())
                        }
                    }
                }

                for p in rooks {
                    if p & friendly_pieces == 0 {
                        // ...then this move does not intersect any friendly pieces, and it can be played, ignoring king safety.
                    }
                }

                for p in bishops {
                    if p & friendly_pieces == 0 {
                        // ...then this move does not intersect any friendly pieces, and it can be played, ignoring king safety.
                    }
                }

                for p in queens {
                    if p & friendly_pieces == 0 {
                        // ...then this move does not intersect any friendly pieces, and it can be played, ignoring king safety.
                    }
                }

                for p in kings {
                    if p & friendly_pieces == 0 {
                        // ...then this move does not intersect any friendly pieces, and it can be played, ignoring king safety.
                    }
                }

                todo!()
            }
            Color::White => {
                todo!()
            }
        }
        todo!()
    }

    pub fn powers_of_two(int: u64) -> Vec<u64> {
        let mut res = Vec::new();
        let mut i = 1_u64;
        while i <= int {
            if i & int != 0 {
                res.push(i);
            }
            i <<= 1;
        }
        res
    }

    pub fn legal_moves(&self, color: Color) -> () {
        match color {
            Color::Black => {
                let friendly_pieces = self.bitboard.pawns_black
                    | self.bitboard.rooks_black
                    | self.bitboard.knights_black
                    | self.bitboard.bishops_black
                    | self.bitboard.queens_black
                    | self.bitboard.king_black;
                let enemy_pieces = self.bitboard.pawns_white
                    | self.bitboard.rooks_white
                    | self.bitboard.knights_white
                    | self.bitboard.bishops_white
                    | self.bitboard.queens_white
                    | self.bitboard.king_white;

                let pseudolegal_moves = todo!();
            }
            Color::White => {
                let friendly_pieces = self.bitboard.pawns_white
                    | self.bitboard.rooks_white
                    | self.bitboard.knights_white
                    | self.bitboard.bishops_white
                    | self.bitboard.queens_white
                    | self.bitboard.king_white;
                let enemy_pieces = self.bitboard.pawns_black
                    | self.bitboard.rooks_black
                    | self.bitboard.knights_black
                    | self.bitboard.bishops_black
                    | self.bitboard.queens_black
                    | self.bitboard.king_black;
            }
        }
        todo!()
    }
}
