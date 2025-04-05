use super::{GameManager, MoveTable, NoArc};
use rayon::prelude::*;

pub fn perft(depth: u16, gm: GameManager, tbl: &NoArc<MoveTable>) -> u64 {
    if depth == 0 {
        1
    } else {
        gm.legal_moves(tbl)
            .into_par_iter()
            .map(|mv| perft(depth - 1, mv.4, tbl))
            .sum::<u64>()
    }
}

pub fn printing_perft(depth: u16, gm: GameManager, tbl: &NoArc<MoveTable>) {
    for mv in gm.legal_moves(tbl) {
        println!(
            "{}{}: {}",
            mv.1.to_str().to_ascii_lowercase(),
            mv.2.to_str().to_ascii_lowercase(),
            perft(depth - 1, mv.4, tbl)
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{gamemanager::GameManager, movetable::{noarc::NoArc, MoveTable}};
    use super::perft;

    #[test]
    fn check_legal_move_gen() {
        let table = NoArc::new(MoveTable::default());
        let depth = 4;
        let expected_positions = [
            (1_u64, 1_u64, 1_u64),
            (20_u64, 6_u64, 44_u64),
            (400_u64, 264_u64, 1486_u64),
            (8902_u64, 9467_u64, 62379_u64),
            (197281_u64, 422333_u64, 2103487_u64),
            (4865609_u64, 15833292_u64, 89941194_u64)
        ];

        //Initial Position
        let gm_1 = GameManager::default();
        let num_moves_1 = perft(depth, gm_1, &table);
        assert!(num_moves_1 == expected_positions[depth as usize].0);

        //Position 4 from Chess Programming Wiki
        let gm_2 = GameManager::from_fen_str("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        let num_moves_2 = perft(depth, gm_2, &table);
        assert!(num_moves_2 == expected_positions[depth as usize].1);
        

        //Position 5 from Chess Programming Wiki
        let gm_3 = GameManager::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 ");
        let num_moves_3 = perft(depth, gm_3, &table);
        assert!(num_moves_3 == expected_positions[depth as usize].2);
    }
}
