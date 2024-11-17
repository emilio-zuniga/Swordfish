use crate::types::Move;

use super::GameManager;

pub fn perft(depth: u16, maxdepth: u16, mv: Move, gm: GameManager) {
    if depth >= maxdepth + 1 {
        return;
    }
    println!("SEARCHING AT DEPTH {depth}/{maxdepth}.");
    println!(
        "{:?} from {} to {} as {:?}",
        mv.0,
        mv.1.to_str(),
        mv.2.to_str(),
        mv.3
    );
    let mvlst = gm.legal_moves();
    mvlst.into_iter().for_each(|(pc, from, to, mvtp, modgm)| {
        perft(depth + 1, maxdepth, (pc, from, to, mvtp.clone()), modgm)
    });
}
