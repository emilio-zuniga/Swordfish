mod bitboard;
mod gamemanager;
mod movetable;
mod hashing;
mod types;

use std::time::Instant;

use gamemanager::{legal_moves::perft::perft, GameManager};
use movetable::{noarc::NoArc, MoveTable};


fn main() {
    let table = NoArc::new(MoveTable::default());
    let board = GameManager::default();

    let now = Instant::now();
    let nodes = perft(1, board, &table);
    let later = now.elapsed();
    println!("Elapsed: {:.4?}", later);
    println!("Nodes searched: {}", nodes);
}