use crate::types::Move;

use super::GameManager;
use rayon::prelude::*;

pub fn perft(depth: u16, maxdepth: u16, mv: Move, gm: GameManager) {
    if depth > maxdepth {
        return;
    }
    let mvlst = gm.legal_moves();
    let count = mvlst.iter().count();
    if depth == maxdepth {
        //let s = format!("{}{}: ", mv.1.to_str(), mv.2.to_str()).to_ascii_lowercase();
        //println!("{}", s);
        //eprintln!("MOVE AT DEPTH {depth}");
        //println!("{s}{}", mvlst.iter().count());
    }
    mvlst
        .into_par_iter()
        .for_each(|(pc, from, to, mvtp, modgm)| {
            perft(depth + 1, maxdepth, (pc, from, to, mvtp.clone()), modgm)
        });
}
