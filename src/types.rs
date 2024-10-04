/// An `enum` to represent which type the piece is. This provides indexing for our hash table of moves.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum PieceType {
    Queen,
    Rook,
    Bishop,
    Knight,
    King,
    Pawn,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
/// An `enum` to represent the color of a piece.
pub enum Color {
    Black,
    White,
}

/// An `enum` representing a single coordinate of a chess board
#[allow(dead_code)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

/* For Make/Unmake Move, likely create enum (or represend w 4 bit integer):
 * quiet moves
 * double pawn push
 * king castle
 * queen castle
 * captures
 * en-passant capture
 * pawn promotion to knight
 * pawn promotion to bishop
 * pawn promotion to rook
 * pawn promotion to queen
 * pawn capture & promotion to knight
 * pawn capture & promotion to bishop
 * pawn capture & promotion to rook
 * pawn capture & promotion to queen
 */