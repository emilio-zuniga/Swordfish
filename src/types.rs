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
    Both,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
/// An `enum` to represent moves.
pub enum MoveType {
    Normal,
    Capture,
    Promotion(PieceType), //desired promotion piece type
    Castle,
}