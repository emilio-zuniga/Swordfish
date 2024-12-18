use crate::{
    bitboard,
    movetable::{noarc::NoArc, MoveTable},
    types::{CastlingRecord, Color, MoveType, PieceType, Square},
};
use bitboard::BitBoard;
use pseudolegal_moves::pseudolegal_moves;
use regex::Regex;

pub mod legal_moves;
pub mod pseudolegal_moves;

/// This is a representation of a chess game and the various states of each element.
#[derive(Clone)]
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
    pub white_to_move: bool,
    pub castling_rights: CastlingRecord,
    pub en_passant_target: Option<Square>,
    pub halfmoves: u32,
    pub fullmoves: u32,
}

impl Default for GameManager {
    /// Constructs a new `GameManager` set to startpos.
    fn default() -> Self {
        GameManager {
            bitboard: BitBoard::default(),
            white_to_move: true,
            castling_rights: CastlingRecord::default(),
            en_passant_target: None,
            halfmoves: 0,
            fullmoves: 1,
        }
    }
}

impl GameManager {
    /// A utility method for generating a new `GameManager` from a FEN string\
    /// * `fen` - a `&str` representing a game's state in FEN
    /// * `returns` - a `GameManager` as generated from the FEN
    pub fn from_fen_str(fen: &str) -> Self {
        if Self::is_valid_fen(fen) {
            let tokens: Vec<String> = fen.split_whitespace().map(str::to_string).collect();
            GameManager {
                //board space validation implemented at higher level (is_valid_fen())
                bitboard: BitBoard::from_fen_string(&tokens[0]),
                white_to_move: tokens[1] == "w",
                castling_rights: CastlingRecord::try_from(tokens[2].as_str())
                    .expect("We expect FEN strings to be well-formed."),
                en_passant_target: Square::from_str(tokens[3].as_str()),
                halfmoves: tokens[4].parse().unwrap_or_default(),
                fullmoves: tokens[5].parse().unwrap_or_default(),
            } // TODO: Remove subscript element access to handle malformed FEN strings.
        } else {
            eprintln!("WARNING: Malformed FEN string; defaulting to startpos.");
            GameManager::default()
        }
    }

    /// A utility method generating a complete FEN string representation of the game
    /// * `returns` - a `String` representing the game state in FEN
    pub fn to_fen_string(&self) -> String {
        let cstlng_rights = format!("{}", self.castling_rights);
        let mut s = self.bitboard.to_fen_string();
        s.push(' ');
        s.push(if self.white_to_move { 'w' } else { 'b' });
        s.push(' ');
        s.push_str(if self.castling_rights.are_none() {
            "-"
        } else {
            cstlng_rights.as_str()
        });
        s.push(' ');
        s.push_str(
            match &self.en_passant_target {
                Some(s) => s.to_str(),
                None => "-",
            }
        );
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

    /// Returns a bitmask of all the pieces attacked by the given color on this GameManager's state.
    /// TODO, BUG: Needs to be more careful of pawn moves. Pawns' forward moves cannot capture.
    pub fn attacked_by(&self, tbl: &NoArc<MoveTable>, color: Color) -> u64 {
        let moves = pseudolegal_moves(
            color,
            self.bitboard,
            self.castling_rights,
            self.en_passant_target,
            self.halfmoves,
            self.fullmoves,
            tbl,
        );

        use crate::types::MoveType::*;
        use crate::types::PieceType::*;

        let movefilter: &dyn for<'a, 'b> Fn(&'a &'b (PieceType, Square, Square, MoveType)) -> bool =
            &|mv: &&(PieceType, Square, Square, MoveType)| {
                if mv.3 == DoublePawnPush
                    || mv.0 == Pawn
                        && (mv.3 == QuietMove
                            || mv.3 == BPromotion
                            || mv.3 == RPromotion
                            || mv.3 == NPromotion
                            || mv.3 == QPromotion)
                {
                    return false; // Rule it out if it is a pawn push.
                } else if mv.0 == King && (mv.3 == KingCastle || mv.3 == QueenCastle) {
                    return false; // Rule it out if it is a castling move.
                } else {
                    return true;
                }
            };

        moves
            .iter()
            .filter(movefilter) // Isn't a pawn or isn't a pawn's quiet move.
            .map(|(_, _, to, _)| to.to_u64())
            .fold(0_u64, |acc, v| acc | v)
    }
}

#[cfg(test)]
mod test {
    use super::GameManager;
    use crate::{
        gamemanager::pseudolegal_moves::*,
        movetable::{noarc::NoArc, MoveTable},
        types::Color,
    };

    #[test]
    fn check_fen_strings() {
        let tests = [
            "r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2",
            "8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3",
            "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2",
            "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2",
            "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2",
            "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9",
            "2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4",
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",
            "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1",
            "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1",
            "5k2/8/8/8/8/8/8/4K2R w K - 0 1",
            "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1",
            "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",
            "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",
            "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1",
            "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1",
            "4k3/1P6/8/8/8/8/K7/8 w - - 0 1",
            "8/P1k5/K7/8/8/8/8/8 w - - 0 1",
            "K1k5/8/P7/8/8/8/8/8 w - - 0 1",
            "8/k1P5/8/1K6/8/8/8/8 w - - 0 1",
            "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1",
        ];

        for fen in tests {
            let game = GameManager::from_fen_str(fen);
            let generated_fen = game.to_fen_string();

            assert_eq!(fen, generated_fen);
        }
    }

    #[test]
    fn check_psl_moves_1() {
        let game_manager = GameManager::default();
        let moves = pseudolegal_moves(
            Color::Black,
            game_manager.bitboard,
            game_manager.castling_rights,
            game_manager.en_passant_target,
            game_manager.halfmoves,
            game_manager.fullmoves,
            &NoArc::new(MoveTable::default()),
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
            game_manager.castling_rights,
            game_manager.en_passant_target,
            game_manager.halfmoves,
            game_manager.fullmoves,
            &NoArc::new(MoveTable::default()),
        );

        assert_eq!(
            moves.iter().count(),
            20 /* 20 valid moves at start of game. */
        );
    }
}
