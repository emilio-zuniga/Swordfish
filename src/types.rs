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

/* From the Chess Programming Wiki (can be represented with 4 bit integer):
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
/// An `enum` to represent moves.
pub enum MoveType {
    Normal,
    Capture,
    Promotion(PieceType), //desired promotion piece type
    Castle,
}