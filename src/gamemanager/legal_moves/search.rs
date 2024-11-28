use rayon::prelude::*;

use crate::types::Move;

use super::{GameManager, MoveTable, NoArc};

/// A Negamax search routine that runs in parallel.
pub fn root_negamax(maxdepth: u16, gm: GameManager, tbl: &NoArc<MoveTable>) -> (Move, GameManager) {
    let moves = gm.legal_moves(tbl);

    if moves.len() == 0 {
        panic!("IDK how to handle checkmate or stalemate; help!");
    }

    let alpha = i32::MIN + 1;
    let beta = i32::MAX - 1;

    let labelled_scores: Vec<(i32, (Move, GameManager))> = moves
        .into_par_iter()
        .map(|mv| {
            (
                negamax(1, maxdepth, alpha, beta, &mv.4, tbl),
                ((mv.0, mv.1, mv.2, mv.3), mv.4),
            )
        })
        .collect();

    let mut scored_moves: Vec<(i32, (Move, GameManager))> = labelled_scores
        .into_iter()
        .map(|(s, movetuple)| (s, (movetuple.0, movetuple.1)))
        .collect();

    scored_moves.sort_by(|a, b| a.0.cmp(&b.0));

    scored_moves.pop().expect("There'll be a move!").1
}

fn negamax(
    depth: u16,
    maxdepth: u16,
    mut alpha: i32,
    beta: i32,
    gm: &GameManager,
    tbl: &NoArc<MoveTable>,
) -> i32 {
    if depth == maxdepth {
        gm.evaluate()
    } else {
        let moves = gm.legal_moves(tbl);
        if moves.len() == 0 {
            return i32::MIN + 1; // It's a pretty bad outcome to have no moves,
                                 // but stalemates shouldn't count so hard against us.
        }
        let mut best = i32::MIN + 1;
        for mv in moves {
            let value = -negamax(depth + 1, maxdepth, alpha, beta, &mv.4, tbl);
            best = i32::max(best, value);
            alpha = i32::max(alpha, best);
            if alpha >= beta {
                break;
            }
        }
        best
    }
}
