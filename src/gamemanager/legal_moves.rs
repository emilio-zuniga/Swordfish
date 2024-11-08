//! This module handles filtering of pseudolegal moves, and returns only legal moves from any game state.

use crate::{
    bitboard::*,
    gamemanager::pseudolegal_moves,
    types::{Color, MoveType, PieceType, Square},
};

use super::GameManager;

impl GameManager {
    /// Returns all legal moves possible from this GameManager's state.
    pub fn legal_moves(&self, color: Color) {
        let mut legal_moves: Vec<(PieceType, Square, Square, MoveType, BitBoard)> = vec![];

        let (friendly_pieces, enemy_pieces) = match color {
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

                (friendly_pieces, enemy_pieces)
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

                (friendly_pieces, enemy_pieces)
            }
        };

        let (friendly_king, enemy_king) = match color {
            Color::Black => (self.bitboard.king_black, self.bitboard.king_white),
            Color::White => (self.bitboard.king_white, self.bitboard.king_black),
        };

        // First get all the pseudolegal moves.
        let pslm = pseudolegal_moves::pseudolegal_moves(
            Color::Black,
            self.bitboard,
            &self.movetable,
            &self.castling_rights,
            &self.en_passant_target,
            self.halfmoves,
            self.fullmoves,
        );

        // ASSERT: We will never have Super moves in the pseudolegal moves vector.
        debug_assert!(pslm
            .iter()
            .all(|(piecetype, _, _, _)| *piecetype != PieceType::Super));

        // A pseudolegal move may be illegal iff it leaves the king in check.
        // For each possible move, check legality.
        for (piecetype, from, to, movetype) in pslm {
            // Test for king safety against each enemy bitboard,
            // by grabbing all the moves a super piece can make
            // from the king's square.

            dbg!(&movetype);

            // Create a new bitboard here.
            let modified_bitboard = {
                match color {
                    Color::Black => match piecetype {
                        PieceType::Bishop => BitBoard {
                            bishops_black: (self.bitboard.bishops_black ^ from.to_u64())
                                | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::Rook => BitBoard {
                            rooks_black: (self.bitboard.rooks_black ^ from.to_u64()) | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::King => match movetype {
                            // Handle checks on king-side and queen-side castling differently!
                            MoveType::KingCastle => BitBoard {
                                // Check castling spaces for attacks.
                                king_black: (self.bitboard.king_black ^ from.to_u64())
                                    | to.to_u64(),
                                rooks_black: (self.bitboard.rooks_black ^ Square::H8.to_u64())
                                    | Square::F8.to_u64(),
                                ..self.bitboard
                            },
                            MoveType::QueenCastle => BitBoard {
                                king_black: (self.bitboard.king_black ^ from.to_u64())
                                    | to.to_u64(),
                                rooks_black: (self.bitboard.rooks_black ^ Square::A8.to_u64())
                                    | Square::C8.to_u64(),
                                ..self.bitboard
                            },
                            _ => BitBoard {
                                king_black: (self.bitboard.king_black ^ from.to_u64())
                                    | to.to_u64(),
                                ..self.bitboard
                            },
                        },
                        PieceType::Knight => BitBoard {
                            knights_black: (self.bitboard.knights_black ^ from.to_u64())
                                | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::Pawn => BitBoard {
                            pawns_black: (self.bitboard.pawns_black ^ from.to_u64()) | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::Queen => BitBoard {
                            queens_black: (self.bitboard.queens_black ^ from.to_u64())
                                | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::Super => {
                            unreachable!("We will never generate pseudolegal Super moves.")
                        }
                    },
                    Color::White => match piecetype {
                        PieceType::Bishop => BitBoard {
                            bishops_white: (self.bitboard.bishops_white ^ from.to_u64())
                                | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::Rook => BitBoard {
                            rooks_white: (self.bitboard.rooks_white ^ from.to_u64()) | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::King => match movetype {
                            // Handle checks on king-side and queen-side castling differently!
                            MoveType::KingCastle => BitBoard {
                                // Check castling spaces for attacks.
                                king_white: (self.bitboard.king_white ^ from.to_u64())
                                    | to.to_u64(),
                                rooks_white: (self.bitboard.rooks_white ^ Square::H1.to_u64())
                                    | Square::F1.to_u64(),
                                ..self.bitboard
                            },
                            MoveType::QueenCastle => BitBoard { ..self.bitboard },
                            _ => BitBoard {
                                king_white: (self.bitboard.king_white ^ from.to_u64())
                                    | to.to_u64(),
                                rooks_white: (self.bitboard.rooks_white ^ Square::A1.to_u64())
                                    | Square::C1.to_u64(),
                                ..self.bitboard
                            },
                        },
                        PieceType::Knight => BitBoard {
                            knights_white: (self.bitboard.knights_white ^ from.to_u64())
                                | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::Pawn => BitBoard {
                            pawns_white: (self.bitboard.pawns_white ^ from.to_u64()) | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::Queen => BitBoard {
                            queens_white: (self.bitboard.queens_white ^ from.to_u64())
                                | to.to_u64(),
                            ..self.bitboard
                        },
                        PieceType::Super => {
                            unreachable!("We will never generate pseudolegal Super moves.")
                        }
                    },
                }
            };

            println!("{}", modified_bitboard.to_string());

            // NOTE: Maybe pass a bitboard and movetable reference to a check_legality function? IDK...
            let super_moves = self
                .movetable
                .get_moves(color, PieceType::Super, friendly_king);

            // Just all the Super moves ORed together.
            let all_super_moves: u64 = super_moves
                .iter()
                .fold(0, |acc, ray| acc | ray.iter().fold(0, |acc2, &i| acc2 | i));

            if all_super_moves & enemy_pieces == 0 {
                // The king is not under threat
                // and this is guaranteed (?) to be a legal move.
                // Let's see.
                // * The move doesn't do anything funny. (MoveTable)
                // * The move respects the state of the game. (pseudolegal_moves)
                // * The move doesn't leave the king in check. (No way to reach an enemy piece.)

                // Push it and the modified bitboard to our list of moves.
                legal_moves.push((piecetype, from, to, movetype, modified_bitboard));
            } else {
                // ...he POTENTIALLY is.
                // Continue to check against each bitboard with moves from the corresponding piece type.
                // List shouldn't include Super variant.
                use PieceType::*;
                for piece in [King, Queen, Knight, Rook, Bishop, Pawn].iter() {
                    let moves = self
                        .movetable
                        .get_moves(color, piece.clone(), friendly_king);

                    let all_type_moves = moves.iter()
                .fold(0, |acc, ray| acc | ray.iter().fold(0, |acc2, &i| acc2 | i));

                    match piece {
                        Bishop => 
                    }
                }
            }
        }

        todo!()
    }
}
