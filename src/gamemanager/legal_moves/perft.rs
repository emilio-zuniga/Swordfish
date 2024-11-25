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
