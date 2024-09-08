mod bitboard;
use bitboard::BitBoard;

fn main() {
    let b = BitBoard::default();
    println!("{}",b.to_fen_string());
}
