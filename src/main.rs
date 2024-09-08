use bitboard::BitBoard;

mod gamemaster;
mod bitboard;

fn main() {
    //todo!();
    let board = BitBoard::default();
    println!("{}", board.to_string());
    println!("{}", board.to_fen_string());
}
