use crate::gamemanager::legal_moves::search::root_negamax;
use crate::types::{MoveType, Square};
use crate::{
    enginemanager::Engine,
    gamemanager::GameManager,
    movetable::{noarc::NoArc, MoveTable},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread, u16};
use vampirc_uci::{UciMessage, UciMove, UciPiece};

pub fn communicate(
    mut e: Engine,
    search_flag: Arc<AtomicBool>,
    best_move: Arc<Mutex<(Square, Square, MoveType)>>,
) {
    loop {
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        let msg = vampirc_uci::parse_one(&text);

        match msg {
            UciMessage::Uci => {
                println!("id name Swordfish");
                println!("id author Emilio Zuniga, Ethan Barry, Eric Oliver, Grace Kizer, & Zachary Wilson");
                println!("uciok");
            }
            UciMessage::IsReady => {
                println!("readyok")
            }
            UciMessage::UciNewGame => {
                //e.set_new_game = true;
            }
            UciMessage::Position {
                startpos,
                fen,
                moves,
            } => {
                // For now, we'll reinitalize the engine's data
                // (minus movetable) each time we receive a
                // 'position' command.
                if startpos {
                    e.board = GameManager::default();
                } else {
                    e.board = GameManager::from_fen_str(fen.unwrap().as_str());
                }
                e.move_history = moves.clone();

                for m in moves {
                    e.board = make_move(&e.board, &e.tbl, m);
                }

                //e.set_new_game = false;
            }
            UciMessage::Go {
                time_control,
                search_control: _,
            } => {
                search_flag.store(true, Ordering::Relaxed);
                {
                    let mut lock = best_move.lock().unwrap();
                    *lock = (Square::A1, Square::A1, MoveType::QuietMove); // Reinitialize best_move.
                } // Lock dropped here.
                let flag = search_flag.clone();
                let best_move = best_move.clone();
                let table = NoArc::new(MoveTable::default()); // TODO: Really hurts to create a whole new table...
                let gm = e.board.clone();

                if let Some(timectl) = time_control {
                    match timectl {
                        vampirc_uci::UciTimeControl::Infinite => {
                            thread::spawn(move || {
                                root_negamax(u16::MAX, gm, &table, flag, best_move);
                            });
                        }
                        vampirc_uci::UciTimeControl::MoveTime(t) => {
                            let flag_clone = flag.clone();
                            let loop_flag_clone = flag.clone();
                            let best_move_clone = best_move.clone();
                            thread::spawn(move || {
                                let mut depth = 1;
                                while loop_flag_clone.load(Ordering::Relaxed) {
                                    root_negamax(depth, gm.clone(), &table, flag.clone(), best_move.clone());
                                    depth += 1;
                                }

                                {
                                    let lock = best_move_clone.lock().unwrap();
                                    use MoveType::*;
                                    let promo = match lock.2 {
                                        QPromotion | QPromoCapture => "q",
                                        RPromotion | RPromoCapture => "r",
                                        BPromotion | BPromoCapture => "b",
                                        NPromotion | NPromoCapture => "n",
                                        _ => "",
                                    };
                                    let outstr = format!(
                                        "bestmove {}{}{}",
                                        lock.0.to_str(),
                                        lock.1.to_str(),
                                        promo
                                    );
                                    println!("{}", outstr);
                                }
                            });
                            thread::spawn(move || {
                                thread::sleep(Duration::from_millis(t.num_milliseconds() as u64));
                                flag_clone.store(false, Ordering::Relaxed);
                            });
                        }
                        vampirc_uci::UciTimeControl::Ponder => unimplemented!(),
                        vampirc_uci::UciTimeControl::TimeLeft {
                            white_time,
                            black_time,
                            white_increment: _,
                            black_increment: _,
                            moves_to_go: _,
                        } => {
                            let flag_clone = flag.clone();
                            let loop_flag_clone = flag.clone();
                            let best_move_clone = best_move.clone();
                            thread::spawn(move || {
                                let mut depth = 1;
                                while loop_flag_clone.load(Ordering::Relaxed) {
                                    root_negamax(depth, gm.clone(), &table, flag.clone(), best_move.clone());
                                    depth += 1;
                                }

                                {
                                    let lock = best_move_clone.lock().unwrap();
                                    use MoveType::*;
                                    let promo = match lock.2 {
                                        QPromotion | QPromoCapture => "q",
                                        RPromotion | RPromoCapture => "r",
                                        BPromotion | BPromoCapture => "b",
                                        NPromotion | NPromoCapture => "n",
                                        _ => "",
                                    };
                                    let outstr = format!(
                                        "bestmove {}{}{}",
                                        lock.0.to_str(),
                                        lock.1.to_str(),
                                        promo
                                    );
                                    println!("{}", outstr);
                                }
                            });
                            let time_remaining = if e.board.white_to_move {
                                let time = white_time.unwrap().num_milliseconds().abs();
                                Duration::from_millis(time as u64 / 20)
                            } else {
                                let time = black_time.unwrap().num_milliseconds().abs();
                                Duration::from_millis(time as u64 / 20)
                            };
                            thread::spawn(move || {
                                thread::sleep(time_remaining);
                                flag_clone.store(false, Ordering::Relaxed);
                            });
                        }
                    }
                }
            }
            UciMessage::Stop => {
                search_flag.store(false, Ordering::Relaxed);
                {
                    let lock = best_move.lock().unwrap();
                    use MoveType::*;
                    let promo = match lock.2 {
                        QPromotion | QPromoCapture => "q",
                        RPromotion | RPromoCapture => "r",
                        BPromotion | BPromoCapture => "b",
                        NPromotion | NPromoCapture => "n",
                        _ => "",
                    };
                    let outstr =
                        format!("bestmove {}{}{}", lock.0.to_str(), lock.1.to_str(), promo);
                    println!("{}", outstr);
                }
            }
            UciMessage::Quit => {
                search_flag.store(false, Ordering::Relaxed);
                break;
            }
            _ => {
                println!("Some other message was received.");
            }
        }
    } // End of the input loop. UCI terminates.
}

fn make_move(board: &GameManager, tbl: &NoArc<MoveTable>, m: UciMove) -> GameManager {
    let h_from = Square::from_str(&m.from.to_string()).unwrap();
    let h_to = Square::from_str(&m.to.to_string()).unwrap();
    let legal_moves = board.legal_moves(&tbl);
    let updated_data = legal_moves
        .iter()
        .find(|data| {
            data.1 == h_from
                && data.2 == h_to
                && match m.promotion {
                    Some(p) => match p {
                        UciPiece::Knight => {
                            if m.from.file != m.to.file {
                                //if the files are not the same
                                //then this was a promoting pawn capture
                                data.3 == MoveType::NPromoCapture
                            } else {
                                data.3 == MoveType::NPromotion
                            }
                        }
                        UciPiece::Bishop => {
                            if m.from.file != m.to.file {
                                //if the files are not the same
                                //then this was a promoting pawn capture
                                data.3 == MoveType::BPromoCapture
                            } else {
                                data.3 == MoveType::BPromotion
                            }
                        }
                        UciPiece::Rook => {
                            if m.from.file != m.to.file {
                                //if the files are not the same
                                //then this was a promoting pawn capture
                                data.3 == MoveType::RPromoCapture
                            } else {
                                data.3 == MoveType::RPromotion
                            }
                        }
                        UciPiece::Queen => {
                            if m.from.file != m.to.file {
                                //if the files are not the same
                                //then this was a promoting pawn capture
                                data.3 == MoveType::QPromoCapture
                            } else {
                                data.3 == MoveType::QPromotion
                            }
                        }
                        _ => panic!("We should never promote to a Pawn or King"),
                    },
                    None => true,
                }
        })
        .unwrap();

    updated_data.4.clone()
}
