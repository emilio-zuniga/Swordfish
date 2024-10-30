//! This is the general module for generating pseudolegal moves. It re-exports several submodules
//! which handle individual piece types. This also includes the function [`pseudolegal_moves`], which
//! pulls all the modules together, and generates the pseudolegal moves for an arbitrary piece, on an
//! arbitrary bitboard.

mod bishops;
mod kings;
mod knights;
mod pawns;
mod queens;
mod rooks;

use crate::{
    bitboard::BitBoard,
    gamemanager::GameManager,
    movetable::MoveTable,
    types::{Color, Move},
};

/// Returns a [`Vec`] of pseudolegal moves encoded as a [`Move`](Move) type,
/// where the `Move` type is an alias for `(PieceType, Square, Square, MoveType)`,
/// declared in `types.rs`.
pub fn pseudolegal_moves(
    color: Color,
    bitboard: BitBoard,
    movetable: &MoveTable,
    castling_rights: &str,
    en_passant_target: &str,
    halfmoves: u32,
    fullmoves: u32,
) -> Vec<Move> {
    let mut pseudolegal_moves: Vec<Move> = Vec::new();

    match color {
        Color::Black => {
            // For each black piece on the board, obtain its possible moves.
            // Each piece is a power of two, and we'll pop the powers of two with the function below.

            // Each power of two is passed to its respective MoveTable, and the resultant list of moves is matched against
            // the friendly_pieces integer. This way, invalid moves are filtered out.

            // This means our "pseudo-legal" moves include only valid moves, and moves that leave the king in check, or are not permitted by the rules of chess
            // for some reason besides intersection of pieces.

            let friendly_pieces = bitboard.pawns_black
                | bitboard.knights_black
                | bitboard.bishops_black
                | bitboard.rooks_black
                | bitboard.queens_black
                | bitboard.king_black;
            let enemy_pieces = bitboard.pawns_white
                | bitboard.knights_white
                | bitboard.bishops_white
                | bitboard.rooks_white
                | bitboard.queens_white
                | bitboard.king_white;

            // To get each black piece, pop each power of two for each piece type.
            let pawns = GameManager::powers_of_two(bitboard.pawns_black);
            let knights = GameManager::powers_of_two(bitboard.knights_black);
            //let rooks = GameManager::powers_of_two(self.bitboard.rooks_black);
            //let bishops = GameManager::powers_of_two(self.bitboard.bishops_black);
            let queens = GameManager::powers_of_two(bitboard.queens_black);
            //let kings = GameManager::powers_of_two(self.bitboard.king_black);

            let mut pawn_pseudo_legal_moves = pawns::pseudolegal_pawn_moves(
                color,
                movetable,
                pawns,
                friendly_pieces,
                enemy_pieces,
                en_passant_target,
            );
            pseudolegal_moves.append(&mut pawn_pseudo_legal_moves);

            /*
            let mut knight_pseudo_legal_moves =
                self.pseudolegal_knight_moves(color, knights, friendly_pieces, enemy_pieces);
            pseudolegal_moves.append(&mut knight_pseudo_legal_moves);
            */
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

            let mut queen_pseudo_legal_moves = queens::pseudolegal_queen_moves(
                color,
                movetable,
                queens,
                friendly_pieces,
                enemy_pieces,
            );
            pseudolegal_moves.append(&mut queen_pseudo_legal_moves);

            /*
            let mut king_pseudo_legal_moves =
                self.pseudolegal_king_moves(color, kings, friendly_pieces, enemy_pieces);
            pseudolegal_moves.append(&mut king_pseudo_legal_moves);
             */
        }
        Color::White => {
            let friendly_pieces = bitboard.pawns_white
                | bitboard.knights_white
                | bitboard.bishops_white
                | bitboard.rooks_white
                | bitboard.queens_white
                | bitboard.king_white;

            let enemy_pieces = bitboard.pawns_black
                | bitboard.knights_black
                | bitboard.bishops_black
                | bitboard.rooks_black
                | bitboard.queens_black
                | bitboard.king_black;

            let pawns = GameManager::powers_of_two(bitboard.pawns_white);
            let knights = GameManager::powers_of_two(bitboard.knights_white);
            //let bishops = GameManager::powers_of_two(self.bitboard.bishops_white);
            //let rooks = GameManager::powers_of_two(self.bitboard.rooks_white);
            let queens = GameManager::powers_of_two(bitboard.queens_white);
            //let kings = GameManager::powers_of_two(self.bitboard.king_white);

            let mut pawn_pseudo_legal_moves = pawns::pseudolegal_pawn_moves(
                color,
                movetable,
                pawns,
                friendly_pieces,
                enemy_pieces,
                en_passant_target,
            );
            pseudolegal_moves.append(&mut pawn_pseudo_legal_moves);

            let mut knight_pseudo_legal_moves = knights::pseudolegal_knight_moves(
                color,
                movetable,
                knights,
                friendly_pieces,
                enemy_pieces,
            );
            pseudolegal_moves.append(&mut knight_pseudo_legal_moves);

            //Add the rest of the piece movements here
        }
    }

    println!(
        "Number of moves across all {} piece types recorded: {}",
        match color {
            Color::Black => "Black",
            Color::White => "White",
        },
        pseudolegal_moves.len()
    );

    pseudolegal_moves
}
