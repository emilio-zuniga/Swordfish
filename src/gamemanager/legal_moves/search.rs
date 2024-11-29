use rayon::prelude::*;

use crate::types::{Move, MoveType, PieceType, Square};

use super::{GameManager, MoveTable, NoArc};

/// A Negamax search routine that runs in parallel.
pub fn root_negamax(depth: u16, gm: GameManager, tbl: &NoArc<MoveTable>) -> (Move, GameManager) {
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
                -negamax(depth, -beta, -alpha, mv.3, &mv.4, tbl),
                ((mv.0, mv.1, mv.2, mv.3), mv.4),
            )
        })
        .collect();

    let mut scored_moves: Vec<(i32, (Move, GameManager))> = labelled_scores
        .into_iter()
        .map(|(s, movetuple)| (s, (movetuple.0, movetuple.1)))
        .collect();

    scored_moves.sort_by(|a, b| a.0.cmp(&b.0));

    // Little bit of debugging code.
    let _ = scored_moves
        .iter()
        .for_each(|m| println!("{}: {}{}", m.0, m.1 .0 .1.to_str(), m.1 .0 .2.to_str()));

    scored_moves.pop().expect("There'll be a move!").1
}

fn negamax(
    depth: u16,
    mut alpha: i32,
    beta: i32,
    movetype: MoveType,
    gm: &GameManager,
    tbl: &NoArc<MoveTable>,
) -> i32 {
    if depth == 0 {
        capture_search(alpha, beta, movetype, gm, tbl)
    } else {
        let moves = gm.legal_moves(tbl);

        if moves.len() == 0 {
            return i32::MIN + 1; // It's a pretty bad outcome to have no moves,
                                 // but stalemates shouldn't count so hard against us.
        }

        for mv in moves {
            // Call negamax and negate it's return value. Enemy's alpha is our -beta & v.v.
            let score = -negamax(depth - 1, -beta, -alpha, mv.3, &mv.4, tbl);

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }
        alpha
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
