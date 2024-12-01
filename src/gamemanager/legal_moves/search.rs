use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use rayon::prelude::*;

use crate::types::{Move, MoveType, PieceType, Square};

use super::{GameManager, MoveTable, NoArc};

/// A Negamax search routine that runs in parallel.
pub fn root_negamax(
    depth: u16,
    gm: &GameManager,
    tbl: &NoArc<MoveTable>,
    flag: Arc<AtomicBool>,
) -> (Move, GameManager) {
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
                -negamax(depth, -beta, -alpha, mv.3, &mv.4, tbl, flag.clone()),
                ((mv.0, mv.1, mv.2, mv.3), mv.4),
            )
        })
        .collect();

    let mut scored_moves: Vec<(i32, (Move, GameManager))> = labelled_scores
        .into_iter()
        .map(|(s, movetuple)| (s, (movetuple.0, movetuple.1)))
        .collect();

    if depth % 2 == 0 {
        scored_moves.sort_by(|a, b| a.0.cmp(&b.0));
    } else {
        scored_moves.sort_by(|a, b| a.0.cmp(&b.0));
    }

    let best = scored_moves
        .into_iter()
        .inspect(|m| println!("{}: {}{}", m.0, m.1 .0 .1.to_str(), m.1 .0 .2.to_str()))
        .last()
        .expect("Should be a move here!");

    println!("Best score: {}", best.0);
    best.1
}

fn negamax(
    depth: u16,
    mut alpha: i32,
    beta: i32,
    movetype: MoveType,
    gm: &GameManager,
    tbl: &NoArc<MoveTable>,
    flag: Arc<AtomicBool>,
) -> i32 {
    if flag.load(Ordering::Relaxed) == false || depth == 0 {
        // NOTE: Call quiesence search on the current position regardless of
        // depth if the flag "continue searching" is false. We can't stop
        // immediately without throwing out the work at this depth entirely,
        // and I'm not that good at concurrent programs to make that work.
        capture_search(alpha, beta, movetype, gm, tbl)
    } else {
        let moves = gm.legal_moves(tbl);

        if moves.len() == 0 {
            return i32::MIN + 1; // Return value of node.
        }

        let mut score = i32::MIN + 1;
        for mv in moves {
            // Call negamax and negate it's return value. Enemy's alpha is our -beta & v.v.
            score = score.max(-negamax(
                depth - 1,
                -beta,
                -alpha,
                mv.3,
                &mv.4,
                tbl,
                flag.clone(),
            ));
            alpha = alpha.max(score);
            if alpha >= beta {
                break;
            }
        }
        score
    }
}

fn capture_search(
    mut alpha: i32,
    beta: i32,
    movetype: MoveType,
    gm: &GameManager,
    tbl: &NoArc<MoveTable>,
) -> i32 {
    let mut eval = gm.evaluate(movetype);

    if eval >= beta {
        beta
    } else {
        alpha = alpha.max(eval);

        let captures: Vec<(PieceType, Square, Square, MoveType, GameManager)> = gm
            .legal_moves(tbl)
            .into_iter()
            .filter(|m| {
                use MoveType::*;
                match m.3 {
                    Capture | NPromoCapture | BPromoCapture | RPromoCapture | QPromoCapture
                    | EPCapture => true,
                    _ => false,
                }
            })
            .collect();

        for capture in captures {
            eval = -capture_search(-beta, -alpha, capture.3, &capture.4, tbl);
            if eval >= beta {
                return beta;
            }
            alpha = alpha.max(eval);
        }
        alpha
    }
}
