#![allow(dead_code, unused_variables, unused_mut)]
use crate::{bitboard, movetable::MoveTable};
use bitboard::BitBoard;
use regex::Regex;

pub mod legal_moves;
pub mod pseudolegal_moves;

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
    pub bitboard: BitBoard,
    pub movetable: MoveTable,
    pub white_to_move: bool,
    pub castling_rights: String,
    pub en_passant_target: String,
    pub halfmoves: u32,
    pub fullmoves: u32,
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

    pub fn powers_of_two(int: u64) -> Vec<u64> {
        let mut res = Vec::new();
        let mut i = 1_u64;
        while i <= int && i != 0 {
            if i & int != 0 {
                debug_assert!(i.is_power_of_two());
                res.push(i);
            }
            i <<= 1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::GameManager;
    use crate::{gamemanager::pseudolegal_moves::*, types::Color};

    #[test]
    fn check_psl_moves_1() {
        let game_manager = GameManager::default();
        let moves = pseudolegal_moves(
            Color::Black,
            game_manager.bitboard,
            &game_manager.movetable,
            &game_manager.castling_rights,
            &game_manager.en_passant_target,
            game_manager.halfmoves,
            game_manager.fullmoves,
        );

        assert_eq!(
            moves.iter().count(),
            20 /* 20 valid moves at start of game. */
        );
    }

    #[test]
    fn check_psl_moves_2() {
        let game_manager = GameManager::default();
        let moves = pseudolegal_moves(
            Color::White,
            game_manager.bitboard,
            &game_manager.movetable,
            &game_manager.castling_rights,
            &game_manager.en_passant_target,
            game_manager.halfmoves,
            game_manager.fullmoves,
        );

        dbg!(&moves);
        assert_eq!(
            moves.iter().count(),
            20 /* 20 valid moves at start of game. */
        );
    }
}
