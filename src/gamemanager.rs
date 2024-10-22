#![allow(dead_code, unused_variables, unused_mut)]
use crate::{
    bitboard,
    movetable::MoveTable,
    types::{Color, MoveType, PieceType, Square},
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

    /// A method returning a list of pseudo-legal moves playable according to the
    /// information encoded in this instance of GameManager
    /// * `color` - the `Color` of the side to move
    /// * `returns` - a `Vec` of tuple representing playable moves:\
    ///     (the `PieceType` of the piece to move, the starting `Square`,
    ///     the target `Square`, and the `MoveType`)
    pub fn pseudolegal_moves(&self, color: Color) -> Vec<(PieceType, Square, Square, MoveType)> {
        let mut pseudolegal_moves: Vec<(PieceType, Square, Square, MoveType)> = Vec::new();

        match color {
            Color::Black => {
                // For each black piece on the board, obtain its possible moves.
                // Each piece is a power of two, and we'll pop the powers of two with the function below.

                // Each power of two is passed to its respective MoveTable, and the resultant list of moves is matched against
                // the friendly_pieces integer. This way, invalid moves are filtered out.

                // This means our "pseudo-legal" moves include only valid moves, and moves that leave the king in check, or are not permitted by the rules of chess
                // for some reason besides intersection of pieces.

                let friendly_pieces = self.bitboard.pawns_black
                    | self.bitboard.knights_black
                    | self.bitboard.bishops_black
                    | self.bitboard.rooks_black
                    | self.bitboard.queens_black
                    | self.bitboard.king_black;
                let enemy_pieces = self.bitboard.pawns_white
                    | self.bitboard.knights_white
                    | self.bitboard.bishops_white
                    | self.bitboard.rooks_white
                    | self.bitboard.queens_white
                    | self.bitboard.king_white;

                // To get each black piece, pop each power of two for each piece type.
                let pawns = GameManager::powers_of_two(self.bitboard.pawns_black);
                let knights = GameManager::powers_of_two(self.bitboard.knights_black);
                //let rooks = GameManager::powers_of_two(self.bitboard.rooks_black);
                //let bishops = GameManager::powers_of_two(self.bitboard.bishops_black);
                //let queens = GameManager::powers_of_two(self.bitboard.queens_black);
                //let kings = GameManager::powers_of_two(self.bitboard.king_black);

                let mut pawn_pseudo_legal_moves =
                    self.pseudolegal_pawn_moves(color, pawns, friendly_pieces, enemy_pieces);
                pseudolegal_moves.append(&mut pawn_pseudo_legal_moves);
                
                let mut knight_pseudo_legal_moves =
                    self.pseudolegal_knight_moves(color, knights, friendly_pieces, enemy_pieces);
                pseudolegal_moves.append(&mut knight_pseudo_legal_moves);



                /*
                let mut bishop_pseudo_legal_moves =
                    self.pseudolegal_bishop_moves(color, bishops, friendly_pieces, enemy_pieces);
                pseudolegal_moves.append(&mut bishop_pseudo_legal_moves);
                 */

                /*
                let mut rook_pseudo_legal_moves =
                    self.pseudolegal_rook_moves(color, rooks, friendly_pieces, enemy_pieces);
                pseudolegal_moves.append(&mut rook_pseudo_legal_moves);
                 */

                /*
                let mut queen_pseudo_legal_moves =
                    self.pseudolegal_queen_moves(color, queens, friendly_pieces, enemy_pieces);
                pseudolegal_moves.append(&mut queen_pseudo_legal_moves);
                 */

                /*
                let mut king_pseudo_legal_moves =
                    self.pseudolegal_king_moves(color, kings, friendly_pieces, enemy_pieces);
                pseudolegal_moves.append(&mut king_pseudo_legal_moves);
                 */
            }
            Color::White => {
                let friendly_pieces = self.bitboard.pawns_white
                    | self.bitboard.knights_white
                    | self.bitboard.bishops_white
                    | self.bitboard.rooks_white
                    | self.bitboard.queens_white
                    | self.bitboard.king_white;

                let enemy_pieces = self.bitboard.pawns_black
                    | self.bitboard.knights_black
                    | self.bitboard.bishops_black
                    | self.bitboard.rooks_black
                    | self.bitboard.queens_black
                    | self.bitboard.king_black;

                let pawns = GameManager::powers_of_two(self.bitboard.pawns_white);
                let knights = GameManager::powers_of_two(self.bitboard.knights_white);
                //let bishops = GameManager::powers_of_two(self.bitboard.bishops_white);
                //let rooks = GameManager::powers_of_two(self.bitboard.rooks_white);
                //let queens = GameManager::powers_of_two(self.bitboard.queens_white);
                //let kings = GameManager::powers_of_two(self.bitboard.king_white);

                let mut pawn_pseudo_legal_moves =
                    self.pseudolegal_pawn_moves(color, pawns, friendly_pieces, enemy_pieces);
                pseudolegal_moves.append(&mut pawn_pseudo_legal_moves);

                let mut knight_pseudo_legal_moves =
                    self.pseudolegal_knight_moves(color, knights, friendly_pieces, enemy_pieces);
                pseudolegal_moves.append(&mut knight_pseudo_legal_moves);
                
                //Add the rest of the piece movements here
            }
        }

        println!(
            "Number of moves accross all {} piece types recorded: {}",
            match color {
                Color::Black => "Black",
                Color::White => "White",
            },
            pseudolegal_moves.len()
        );

        pseudolegal_moves
    }

    /// A method returning a list of pseudo-legal pawn moves playable according to
    /// the information encoded in this instance of GameManager
    /// * `color` - the `Color` of the side to move
    /// * `pawn_locations` - a `Vec<u64>` containing a list of each pawn's location
    /// * `friendly_pieces` - a `u64` representing the current position of allied pieces
    /// * `enemy_pieces` - a `u64` representing the current position of enemy pieces
    /// * `returns` - a `Vec` of tuples representing playable pawn moves in the following form:\
    ///     (the `PieceType` of the piece to move, the starting `Square`,
    ///     the target `Square`, and the `MoveType`)
    fn pseudolegal_pawn_moves(
        &self,
        color: Color,
        pawn_locations: Vec<u64>,
        friendly_pieces: u64,
        enemy_pieces: u64,
    ) -> Vec<(PieceType, Square, Square, MoveType)> {
        let mut pawn_pseudo_legal_moves = Vec::new();

        //pawns can do:
        /* Quiet Move
         * Double Pawn Push
         * Capture
         * En Passant Capture
         * 4 Promotions
         * 4 Capture Promotions
         */
        let rank_1: u64 = 0x00000000_000000FF;
        let rank_2: u64 = 0x00000000_0000FF00;
        let rank_7: u64 = 0x00FF0000_00000000;
        let rank_8: u64 = 0xFF000000_00000000;

        match color {
            Color::Black => {
                for pawn in pawn_locations {
                    for r in self.movetable.get_moves(color, PieceType::Pawn, pawn) {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                                let from = Square::from_u64(pawn)
                                                .expect("Each u64 is a power of two");
                                let to = Square::from_u64(m)
                                                .expect("Each u64 is a power of two");

                                if (m & (pawn >> 8)) == m {
                                    // then this move is a type of pawn push
                                    if m & enemy_pieces == 0 {
                                        //then this push is not blocked
                                        if m & rank_1 == m {
                                            //then this move will either be:
                                            /* Promotion to Knight
                                             * Promotion to Bishop
                                             * Promotion to Rook
                                             * Promotion to Queen
                                             */

                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::NPromotion,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::BPromotion,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::RPromotion,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::QPromotion,
                                            ));
                                        } else {
                                            //just a normal pawn push

                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::QuietMove,
                                            ));
                                        }
                                    }
                                } else if (pawn & rank_7 == pawn) && (m & (pawn >> 16)) == m {
                                    //then this move is a double push
                                    if (friendly_pieces | enemy_pieces) & (m | (m << 8)) == 0 {
                                        //then there is in between the pawn and the fifth rank

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::DoublePawnPush,
                                        ));
                                    }
                                } else {
                                    //this move is a type of capture
                                    if m & enemy_pieces == m {
                                        //then this move is definitely a capture
                                        if m & rank_1 == m {
                                            //then this move is a promotion capture

                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::NPromoCapture,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::BPromoCapture,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::RPromoCapture,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::QPromoCapture,
                                            ));
                                        } else {
                                            //then this move is a standard capture

                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::Capture,
                                            ));
                                        }
                                    } else if match Square::from_str(&self.en_passant_target) {
                                        Some(coord) => m & coord.to_u64() == m,
                                        None => false,
                                    } {
                                        //then this move is an en passant capture

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::EPCapture,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Color::White => {
                for pawn in pawn_locations {
                    for r in self.movetable.get_moves(color, PieceType::Pawn, pawn) {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                                let from = Square::from_u64(pawn)
                                                .expect("Each u64 is a power of two");
                                let to = Square::from_u64(m)
                                                .expect("Each u64 is a power of two");
                                
                                if (m & (pawn << 8)) == m {
                                    // then this move is a type of pawn push
                                    if m & enemy_pieces == 0 {
                                        //then this push is not blocked
                                        if m & rank_8 == m {
                                            //then this move will either be:
                                            /* Promotion to Knight
                                             * Promotion to Bishop
                                             * Promotion to Rook
                                             * Promotion to Queen
                                             */

                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::NPromotion,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::BPromotion,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::RPromotion,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::QPromotion,
                                            ));
                                        } else {
                                            //just a normal pawn push

                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::QuietMove,
                                            ));
                                        }
                                    }
                                } else if (pawn & rank_2 == pawn) && (m & (pawn << 16)) == m {
                                    //then this move is a double push
                                    if (friendly_pieces | enemy_pieces) & (m | (m >> 8)) == 0 {
                                        //then there is in between the pawn and the fourth rank

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::DoublePawnPush,
                                        ));
                                    }
                                } else {
                                    //this move is a type of capture
                                    if m & enemy_pieces == m {
                                        //then this move is definitely a capture
                                        if m & rank_8 == m {
                                            //then this move is a promotion capture

                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::NPromoCapture,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::BPromoCapture,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::RPromoCapture,
                                            ));
                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::QPromoCapture,
                                            ));
                                        } else {
                                            //then this move is a standard capture

                                            pawn_pseudo_legal_moves.push((
                                                PieceType::Pawn,
                                                from,
                                                to,
                                                MoveType::Capture,
                                            ));
                                        }
                                    } else if match Square::from_str(&self.en_passant_target) {
                                        Some(coord) => m & coord.to_u64() == m,
                                        None => false,
                                    } {
                                        //then this move is an en passant capture

                                        pawn_pseudo_legal_moves.push((
                                            PieceType::Pawn,
                                            from,
                                            to,
                                            MoveType::EPCapture,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        pawn_pseudo_legal_moves
    }

    ///returned as (piece type, from square, to square, move type)
    fn pseudolegal_knight_moves(
        &self,
        color: Color,
        knight_locations: Vec<u64>,
        friendly_pieces: u64,
        enemy_pieces: u64,
    ) -> Vec<(PieceType, Square, Square, MoveType)> {
        let mut knight_pseudo_legal_moves = Vec::new();

        match color {
            Color::Black => {
                for knight in knight_locations {
                    for r in self
                        .movetable
                        .get_moves(Color::Black, PieceType::Knight, knight)
                    {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // The knight's move does not intersect any friendly pieces
                                
                                let from = Square::from_u64(knight).expect("Each u64 is a power of two");
                                let to = Square::from_u64(m).expect("Each u64 is a power of two");

                                if m & enemy_pieces != 0 {
                                    // It's a capture move if the destination is occupied by an enemy piece
                                    knight_pseudo_legal_moves.push((
                                        PieceType::Knight,
                                        from,
                                        to,
                                        MoveType::Capture,
                                    ));
                                } else {
                                    // It's a quiet move (no capture)
                                    knight_pseudo_legal_moves.push((
                                        PieceType::Knight,
                                        from,
                                        to,
                                        MoveType::QuietMove,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Color::White => {
                for knight in knight_locations {
                    for r in self
                        .movetable
                        .get_moves(Color::White, PieceType::Knight, knight)
                    {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // The knight's move does not intersect any friendly pieces
                                let from = Square::from_u64(knight).expect("Each u64 is a power of two");
                                let to = Square::from_u64(m).expect("Each u64 is a power of two");

                                if m & enemy_pieces != 0 {
                                    // It's a capture move if the destination is occupied by an enemy piece
                                    knight_pseudo_legal_moves.push((
                                        PieceType::Knight,
                                        from,
                                        to,
                                        MoveType::Capture,
                                    ));
                                } else {
                                    // It's a quiet move (no capture)
                                    knight_pseudo_legal_moves.push((
                                        PieceType::Knight,
                                        from,
                                        to,
                                        MoveType::QuietMove,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        knight_pseudo_legal_moves
    }

    /// Return all bishop moves from the given locations as `(PieceType, Square, Square, MoveType)`.
    fn pseudolegal_bishop_moves(
        &self,
        color: Color,
        bishop_locations: Vec<u64>,
        friendly_pieces: u64,
        enemy_pieces: u64,
    ) -> Vec<(PieceType, Square, Square, MoveType)> {
        let mut bishop_pseudo_legal_moves = Vec::new();

        match color {
            Color::Black => {
                for bishop in bishop_locations {
                    for r in self
                        .movetable
                        .get_moves(Color::Black, PieceType::Bishop, bishop)
                    {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                                todo!()
                            }
                        }
                    }
                }
            }
            Color::White => {
                for bishop in bishop_locations {
                    for r in self
                        .movetable
                        .get_moves(Color::White, PieceType::Bishop, bishop)
                    {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                                todo!()
                            }
                        }
                    }
                }
            }
        }

        bishop_pseudo_legal_moves
    }

    ///returned as (piece type, from square, to square, move type)
    fn pseudolegal_rook_moves(
        &self,
        color: Color,
        rook_locations: Vec<u64>,
        friendly_pieces: u64,
        enemy_pieces: u64,
    ) -> Vec<(PieceType, Square, Square, MoveType)> {
        let mut rook_pseudo_legal_moves = Vec::new();

        match color {
            Color::Black => {
                for rook in rook_locations {
                    for r in self.movetable.get_moves(Color::Black, PieceType::Rook, rook) {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                            }
                        }
                    }
                }
            }
            Color::White => {
                for rook in rook_locations {
                    for r in self.movetable.get_moves(Color::White, PieceType::Rook, rook) {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                            }
                        }
                    }
                }
            }
        }

        rook_pseudo_legal_moves
    }

    fn pseudolegal_queen_moves(
        &self,
        color: Color,
        queen_locations: Vec<u64>,
        friendly_pieces: u64,
        enemy_pieces: u64,
    ) -> Vec<(PieceType, Square, Square, MoveType)> {
        let mut queen_pseudo_legal_moves = Vec::new();

        match color {
            Color::Black => {
                for queen in queen_locations {
                    for r in self.movetable.get_moves(Color::Black, PieceType::Queen, queen) {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                            }
                        }
                    }
                }
            }
            Color::White => {
                for queen in queen_locations {
                    for r in self.movetable.get_moves(Color::White, PieceType::Queen, queen) {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                            }
                        }
                    }
                }
            }
        }

        queen_pseudo_legal_moves
    }

    ///returned as (piece type, from square, to square, move type)
    fn pseudolegal_king_moves(
        &self,
        color: Color,
        king_locations: Vec<u64>,
        friendly_pieces: u64,
        enemy_pieces: u64,
    ) -> Vec<(PieceType, Square, Square, MoveType)> {
        let mut king_pseudo_legal_moves = Vec::new();

        match color {
            Color::Black => {
                for king in king_locations {
                    for r in self.movetable.get_moves(Color::Black, PieceType::King, king) {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                            }
                        }
                    }
                }
            }
            Color::White => {
                for king in king_locations {
                    for r in self.movetable.get_moves(Color::White, PieceType::King, king) {
                        for m in r {
                            if m & friendly_pieces == 0 {
                                // ...then this move does not intersect any friendly pieces
                            }
                        }
                    }
                }
            }
        }

        king_pseudo_legal_moves
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

    // TODO: Implement move legality checks.
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

#[cfg(test)]
mod test {
    use super::GameManager;
    use crate::types::Color;

    #[test]
    fn check_psl_moves_1() {
        let game_manager = GameManager::default();
        let moves = game_manager.pseudolegal_moves(Color::Black);
        assert_eq!(
            moves.iter().count(),
            20 /* 20 valid moves at start of game. */
        );
    }
}
