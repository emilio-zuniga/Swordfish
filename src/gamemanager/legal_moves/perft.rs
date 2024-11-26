use crate::types::Square;

use super::{GameManager, MoveTable, NoArc};
use rayon::prelude::*;

pub fn perft(depth: u16, maxdepth: u16, gm: GameManager, tbl: &NoArc<MoveTable>) -> u64 {
    if depth == maxdepth {
        1
    } else {
        gm.legal_moves(tbl)
            .into_par_iter()
            .map(|mv| perft(depth + 1, maxdepth, mv.4, tbl))
            .sum::<u64>()
    }
}

pub fn printing_perft(depth: u16, maxdepth: u16, gm: GameManager, tbl: &NoArc<MoveTable>) {
    for mv in gm.legal_moves(tbl) {
        if mv.1 == Square::E1 && mv.2 == Square::C1 {
            println!("--- MOVE E1C1:");
            printing_perft(depth + 1, maxdepth, mv.4, tbl);
            println!("--- END  E1C1.");
        } else if mv.1 == Square::A6 && mv.2 == Square::B5 {
            println!("--- MOVE A6B5:");
            printing_perft(depth + 1, maxdepth, mv.4, tbl);
            println!("--- END  A6B5.");
        } else {
            println!(
                "{}{}: {}",
                mv.1.to_str().to_ascii_lowercase(),
                mv.2.to_str().to_ascii_lowercase(),
                perft(depth, maxdepth, mv.4, tbl)
            )
        }
    }
}
