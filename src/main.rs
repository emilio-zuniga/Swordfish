use gamemanager::{legal_moves::perft::perft, GameManager};
use movetable::{noarc::NoArc, MoveTable};

mod bitboard;
mod gamemanager;
mod movetable;
mod types;

fn main() {
    let gm = GameManager::default();
    let table = NoArc::new(MoveTable::default());
    let depth = 5;

    println!("Movecount at depth {}: {}", depth, perft(depth, gm, &table));
}