use std::fmt::Display;

/// An `enum` to represent which type the piece is. This provides indexing for our hash table of moves.
/// It also carries the evaluation weights of the pieces, which influence our eval. fn.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum PieceType {
    King,
    Queen = 1000,
    Rook = 500,
    Bishop = 320,
    Knight = 300,
    Pawn = 100,
    Super,
}

pub type Move = (PieceType, Square, Square, MoveType);

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// An `enum` to represent the color of a piece.
pub enum Color {
    Black,
    White,
}

/// An `enum` representing a single coordinate of a chess board
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Square {
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
}

impl Square {
    /// A function that generates a `Square` coordinate from a `u64`.\
    /// * `coordinate`` - a `u64` representing a single square on a chess board \
    /// * `returns` - `Some(Square)` iff `coordinate` is represented by some power of 2; otherwise `None`
    pub fn from_u64(coordinate: u64) -> Option<Square> {
        match coordinate {
            0x80000000_00000000 => Some(Square::A8),
            0x40000000_00000000 => Some(Square::B8),
            0x20000000_00000000 => Some(Square::C8),
            0x10000000_00000000 => Some(Square::D8),
            0x08000000_00000000 => Some(Square::E8),
            0x04000000_00000000 => Some(Square::F8),
            0x02000000_00000000 => Some(Square::G8),
            0x01000000_00000000 => Some(Square::H8),

            0x00800000_00000000 => Some(Square::A7),
            0x00400000_00000000 => Some(Square::B7),
            0x00200000_00000000 => Some(Square::C7),
            0x00100000_00000000 => Some(Square::D7),
            0x00080000_00000000 => Some(Square::E7),
            0x00040000_00000000 => Some(Square::F7),
            0x00020000_00000000 => Some(Square::G7),
            0x00010000_00000000 => Some(Square::H7),

            0x00008000_00000000 => Some(Square::A6),
            0x00004000_00000000 => Some(Square::B6),
            0x00002000_00000000 => Some(Square::C6),
            0x00001000_00000000 => Some(Square::D6),
            0x00000800_00000000 => Some(Square::E6),
            0x00000400_00000000 => Some(Square::F6),
            0x00000200_00000000 => Some(Square::G6),
            0x00000100_00000000 => Some(Square::H6),

            0x00000080_00000000 => Some(Square::A5),
            0x00000040_00000000 => Some(Square::B5),
            0x00000020_00000000 => Some(Square::C5),
            0x00000010_00000000 => Some(Square::D5),
            0x00000008_00000000 => Some(Square::E5),
            0x00000004_00000000 => Some(Square::F5),
            0x00000002_00000000 => Some(Square::G5),
            0x00000001_00000000 => Some(Square::H5),

            0x00000000_80000000 => Some(Square::A4),
            0x00000000_40000000 => Some(Square::B4),
            0x00000000_20000000 => Some(Square::C4),
            0x00000000_10000000 => Some(Square::D4),
            0x00000000_08000000 => Some(Square::E4),
            0x00000000_04000000 => Some(Square::F4),
            0x00000000_02000000 => Some(Square::G4),
            0x00000000_01000000 => Some(Square::H4),

            0x00000000_00800000 => Some(Square::A3),
            0x00000000_00400000 => Some(Square::B3),
            0x00000000_00200000 => Some(Square::C3),
            0x00000000_00100000 => Some(Square::D3),
            0x00000000_00080000 => Some(Square::E3),
            0x00000000_00040000 => Some(Square::F3),
            0x00000000_00020000 => Some(Square::G3),
            0x00000000_00010000 => Some(Square::H3),

            0x00000000_00008000 => Some(Square::A2),
            0x00000000_00004000 => Some(Square::B2),
            0x00000000_00002000 => Some(Square::C2),
            0x00000000_00001000 => Some(Square::D2),
            0x00000000_00000800 => Some(Square::E2),
            0x00000000_00000400 => Some(Square::F2),
            0x00000000_00000200 => Some(Square::G2),
            0x00000000_00000100 => Some(Square::H2),

            0x00000000_00000080 => Some(Square::A1),
            0x00000000_00000040 => Some(Square::B1),
            0x00000000_00000020 => Some(Square::C1),
            0x00000000_00000010 => Some(Square::D1),
            0x00000000_00000008 => Some(Square::E1),
            0x00000000_00000004 => Some(Square::F1),
            0x00000000_00000002 => Some(Square::G1),
            0x00000000_00000001 => Some(Square::H1),
            _ => None,
        }
    }

    /// A function that generates the `u64` representation of a `Square`.\
    /// * `returns` - a `u64` indicating the position given by the `Square`
    pub fn to_u64(&self) -> u64 {
        match &self {
            Square::A8 => 0x80000000_00000000,
            Square::B8 => 0x40000000_00000000,
            Square::C8 => 0x20000000_00000000,
            Square::D8 => 0x10000000_00000000,
            Square::E8 => 0x08000000_00000000,
            Square::F8 => 0x04000000_00000000,
            Square::G8 => 0x02000000_00000000,
            Square::H8 => 0x01000000_00000000,

            Square::A7 => 0x00800000_00000000,
            Square::B7 => 0x00400000_00000000,
            Square::C7 => 0x00200000_00000000,
            Square::D7 => 0x00100000_00000000,
            Square::E7 => 0x00080000_00000000,
            Square::F7 => 0x00040000_00000000,
            Square::G7 => 0x00020000_00000000,
            Square::H7 => 0x00010000_00000000,

            Square::A6 => 0x00008000_00000000,
            Square::B6 => 0x00004000_00000000,
            Square::C6 => 0x00002000_00000000,
            Square::D6 => 0x00001000_00000000,
            Square::E6 => 0x00000800_00000000,
            Square::F6 => 0x00000400_00000000,
            Square::G6 => 0x00000200_00000000,
            Square::H6 => 0x00000100_00000000,

            Square::A5 => 0x00000080_00000000,
            Square::B5 => 0x00000040_00000000,
            Square::C5 => 0x00000020_00000000,
            Square::D5 => 0x00000010_00000000,
            Square::E5 => 0x00000008_00000000,
            Square::F5 => 0x00000004_00000000,
            Square::G5 => 0x00000002_00000000,
            Square::H5 => 0x00000001_00000000,

            Square::A4 => 0x00000000_80000000,
            Square::B4 => 0x00000000_40000000,
            Square::C4 => 0x00000000_20000000,
            Square::D4 => 0x00000000_10000000,
            Square::E4 => 0x00000000_08000000,
            Square::F4 => 0x00000000_04000000,
            Square::G4 => 0x00000000_02000000,
            Square::H4 => 0x00000000_01000000,

            Square::A3 => 0x00000000_00800000,
            Square::B3 => 0x00000000_00400000,
            Square::C3 => 0x00000000_00200000,
            Square::D3 => 0x00000000_00100000,
            Square::E3 => 0x00000000_00080000,
            Square::F3 => 0x00000000_00040000,
            Square::G3 => 0x00000000_00020000,
            Square::H3 => 0x00000000_00010000,

            Square::A2 => 0x00000000_00008000,
            Square::B2 => 0x00000000_00004000,
            Square::C2 => 0x00000000_00002000,
            Square::D2 => 0x00000000_00001000,
            Square::E2 => 0x00000000_00000800,
            Square::F2 => 0x00000000_00000400,
            Square::G2 => 0x00000000_00000200,
            Square::H2 => 0x00000000_00000100,

            Square::A1 => 0x00000000_00000080,
            Square::B1 => 0x00000000_00000040,
            Square::C1 => 0x00000000_00000020,
            Square::D1 => 0x00000000_00000010,
            Square::E1 => 0x00000000_00000008,
            Square::F1 => 0x00000000_00000004,
            Square::G1 => 0x00000000_00000002,
            Square::H1 => 0x00000000_00000001,
        }
    }

    /// A function that generates a `Square` coordinate from a `str`.\
    /// * `coordinate`` - a `str` representing a coordinate on the chess board \
    /// * `returns` - `Some(Square)` iff `coordinate` matches [A-Ha-h]{1}[1-8]{1}; otherwise `None`
    pub fn from_str(coordinate: &str) -> Option<Square> {
        match coordinate {
            "A8" | "a8" => Some(Square::A8),
            "B8" | "b8" => Some(Square::B8),
            "C8" | "c8" => Some(Square::C8),
            "D8" | "d8" => Some(Square::D8),
            "E8" | "e8" => Some(Square::E8),
            "F8" | "f8" => Some(Square::F8),
            "G8" | "g8" => Some(Square::G8),
            "H8" | "h8" => Some(Square::H8),

            "A7" | "a7" => Some(Square::A7),
            "B7" | "b7" => Some(Square::B7),
            "C7" | "c7" => Some(Square::C7),
            "D7" | "d7" => Some(Square::D7),
            "E7" | "e7" => Some(Square::E7),
            "F7" | "f7" => Some(Square::F7),
            "G7" | "g7" => Some(Square::G7),
            "H7" | "h7" => Some(Square::H7),

            "A6" | "a6" => Some(Square::A6),
            "B6" | "b6" => Some(Square::B6),
            "C6" | "c6" => Some(Square::C6),
            "D6" | "d6" => Some(Square::D6),
            "E6" | "e6" => Some(Square::E6),
            "F6" | "f6" => Some(Square::F6),
            "G6" | "g6" => Some(Square::G6),
            "H6" | "h6" => Some(Square::H6),

            "A5" | "a5" => Some(Square::A5),
            "B5" | "b5" => Some(Square::B5),
            "C5" | "c5" => Some(Square::C5),
            "D5" | "d5" => Some(Square::D5),
            "E5" | "e5" => Some(Square::E5),
            "F5" | "f5" => Some(Square::F5),
            "G5" | "g5" => Some(Square::G5),
            "H5" | "h5" => Some(Square::H5),

            "A4" | "a4" => Some(Square::A4),
            "B4" | "b4" => Some(Square::B4),
            "C4" | "c4" => Some(Square::C4),
            "D4" | "d4" => Some(Square::D4),
            "E4" | "e4" => Some(Square::E4),
            "F4" | "f4" => Some(Square::F4),
            "G4" | "g4" => Some(Square::G4),
            "H4" | "h4" => Some(Square::H4),

            "A3" | "a3" => Some(Square::A3),
            "B3" | "b3" => Some(Square::B3),
            "C3" | "c3" => Some(Square::C3),
            "D3" | "d3" => Some(Square::D3),
            "E3" | "e3" => Some(Square::E3),
            "F3" | "f3" => Some(Square::F3),
            "G3" | "g3" => Some(Square::G3),
            "H3" | "h3" => Some(Square::H3),

            "A2" | "a2" => Some(Square::A2),
            "B2" | "b2" => Some(Square::B2),
            "C2" | "c2" => Some(Square::C2),
            "D2" | "d2" => Some(Square::D2),
            "E2" | "e2" => Some(Square::E2),
            "F2" | "f2" => Some(Square::F2),
            "G2" | "g2" => Some(Square::G2),
            "H2" | "h2" => Some(Square::H2),

            "A1" | "a1" => Some(Square::A1),
            "B1" | "b1" => Some(Square::B1),
            "C1" | "c1" => Some(Square::C1),
            "D1" | "d1" => Some(Square::D1),
            "E1" | "e1" => Some(Square::E1),
            "F1" | "f1" => Some(Square::F1),
            "G1" | "g1" => Some(Square::G1),
            "H1" | "h1" => Some(Square::H1),
            _ => None,
        }
    }

    /// A function that generates the `str` representation of a `Square`.\
    /// * `returns` - a `&str` in the format [A-H]{1}[1-8]{1} indicating the position given by the `Square`
    pub fn to_str(&self) -> &str {
        match self {
            Square::A8 => "a8",
            Square::B8 => "b8",
            Square::C8 => "c8",
            Square::D8 => "d8",
            Square::E8 => "e8",
            Square::F8 => "f8",
            Square::G8 => "g8",
            Square::H8 => "h8",

            Square::A7 => "a7",
            Square::B7 => "b7",
            Square::C7 => "c7",
            Square::D7 => "d7",
            Square::E7 => "e7",
            Square::F7 => "f7",
            Square::G7 => "g7",
            Square::H7 => "h7",

            Square::A6 => "a6",
            Square::B6 => "b6",
            Square::C6 => "c6",
            Square::D6 => "d6",
            Square::E6 => "e6",
            Square::F6 => "f6",
            Square::G6 => "g6",
            Square::H6 => "h6",

            Square::A5 => "a5",
            Square::B5 => "b5",
            Square::C5 => "c5",
            Square::D5 => "d5",
            Square::E5 => "e5",
            Square::F5 => "f5",
            Square::G5 => "g5",
            Square::H5 => "h5",

            Square::A4 => "a4",
            Square::B4 => "b4",
            Square::C4 => "c4",
            Square::D4 => "d4",
            Square::E4 => "e4",
            Square::F4 => "f4",
            Square::G4 => "g4",
            Square::H4 => "h4",

            Square::A3 => "a3",
            Square::B3 => "b3",
            Square::C3 => "c3",
            Square::D3 => "d3",
            Square::E3 => "e3",
            Square::F3 => "f3",
            Square::G3 => "g3",
            Square::H3 => "h3",

            Square::A2 => "a2",
            Square::B2 => "b2",
            Square::C2 => "c2",
            Square::D2 => "d2",
            Square::E2 => "e2",
            Square::F2 => "f2",
            Square::G2 => "g2",
            Square::H2 => "h2",

            Square::A1 => "a1",
            Square::B1 => "b1",
            Square::C1 => "c1",
            Square::D1 => "d1",
            Square::E1 => "e1",
            Square::F1 => "f1",
            Square::G1 => "g1",
            Square::H1 => "h1",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MoveType {
    QuietMove,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    Capture,
    EPCapture,
    NPromotion,
    BPromotion,
    RPromotion,
    QPromotion,
    NPromoCapture,
    BPromoCapture,
    RPromoCapture,
    QPromoCapture,
}

impl MoveType {
    /// A function that generates the `str` representation of a `MoveType`.\
    /// * `returns` - a `&str` in the following 4-bit format:
    /// 1st bit: promotion
    /// 2nd bit: capture
    /// 3rd bit: special 1
    /// 4th bit: special 0
    #[allow(dead_code)]
    pub fn to_str(&self) -> &str {
        match self {
            MoveType::QuietMove => "0000",
            MoveType::DoublePawnPush => "0001",
            MoveType::KingCastle => "0010",
            MoveType::QueenCastle => "0011",
            MoveType::Capture => "0100",
            MoveType::EPCapture => "0101",
            MoveType::NPromotion => "1000",
            MoveType::BPromotion => "1001",
            MoveType::RPromotion => "1010",
            MoveType::QPromotion => "1011",
            MoveType::NPromoCapture => "1100",
            MoveType::BPromoCapture => "1101",
            MoveType::RPromoCapture => "1110",
            MoveType::QPromoCapture => "1111",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CastlingRights {
    Kingside,
    Queenside,
    Both,
    Neither,
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Both => "kq",
                Self::Kingside => "k",
                Self::Queenside => "q",
                Self::Neither => "",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastlingRecord {
    pub white: CastlingRights,
    pub black: CastlingRights,
}

impl CastlingRecord {
    /// Returns true if there are no castling rights remaining, else returns false.
    pub fn are_none(&self) -> bool {
        use CastlingRights::*;
        self.black == Neither && self.white == Neither
    }

    /// Returns true if the input string representation intersects the current state.
    /// ## Panics
    /// - if the input is not a single character string in ["K", "Q", "k", "q"].
    pub fn contains(&self, s: &str) -> bool {
        match s {
            "K" => {
                if self.white == CastlingRights::Both || self.white == CastlingRights::Kingside {
                    true
                } else {
                    false
                }
            }
            "Q" => {
                if self.white == CastlingRights::Both || self.white == CastlingRights::Queenside {
                    true
                } else {
                    false
                }
            }
            "k" => {
                if self.black == CastlingRights::Both || self.black == CastlingRights::Kingside {
                    true
                } else {
                    false
                }
            }
            "q" => {
                if self.black == CastlingRights::Both || self.black == CastlingRights::Queenside {
                    true
                } else {
                    false
                }
            }
            "-" => {
                if self.black == CastlingRights::Neither && self.white == CastlingRights::Neither {
                    true
                } else {
                    false
                }
            }
            other => {
                eprintln!("Caught str {other}");
                unreachable!("We will never have malformed FEN input at this point.");
            }
        }
    }
}

impl Display for CastlingRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blackstr = format!("{}", self.black).to_ascii_lowercase();
        let whitestr = format!("{}", self.white).to_ascii_uppercase();
        if whitestr.is_empty() && blackstr.is_empty() {
            write!(f, "-")
        } else {
            write!(f, "{}{}", whitestr, blackstr)
        }
    }
}

impl Into<String> for CastlingRecord {
    fn into(self) -> String {
        format!("{self}")
    }
}

impl Default for CastlingRecord {
    fn default() -> Self {
        use CastlingRights::*;
        Self {
            black: Both,
            white: Both,
        }
    }
}

impl TryFrom<&str> for CastlingRecord {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use CastlingRights::*;
        match value.trim() {
            "KQkq" => Ok(Self {
                black: Both,
                white: Both,
            }),
            "KQk" => Ok(Self {
                black: Kingside,
                white: Both,
            }),
            "KQq" => Ok(Self {
                black: Queenside,
                white: Both,
            }),
            "KQ" => Ok(Self {
                white: Both,
                black: Neither,
            }),
            "Kkq" => Ok(Self {
                black: Both,
                white: Kingside,
            }),
            "Qkq" => Ok(Self {
                black: Both,
                white: Queenside,
            }),
            "kq" => Ok(Self {
                white: Neither,
                black: Both,
            }),
            "Kk" => Ok(Self {
                black: Kingside,
                white: Kingside,
            }),
            "Kq" => Ok(Self {
                white: Kingside,
                black: Queenside,
            }),
            "Qk" => Ok(Self {
                white: Queenside,
                black: Kingside,
            }),
            "Qq" => Ok(Self {
                black: Queenside,
                white: Queenside,
            }),
            "K" => Ok(Self {
                black: Neither,
                white: Kingside,
            }),
            "k" => Ok(Self {
                black: Kingside,
                white: Neither,
            }),
            "Q" => Ok(Self {
                black: Neither,
                white: Queenside,
            }),
            "q" => Ok(Self {
                white: Neither,
                black: Queenside,
            }),
            "-" => Ok(Self {
                black: Neither,
                white: Neither,
            }),
            _ => Err(format!("Invalid castling rights {}!", value)),
        }
    }
}
