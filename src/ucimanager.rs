use crate::gamemanager::legal_moves::search::root_negamax;
use crate::types::{MoveType, Square};
use crate::{
    enginemanager::Engine,
    gamemanager::GameManager,
    movetable::{noarc::NoArc, MoveTable},
};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use vampirc_uci::{UciMessage, UciMove, UciPiece};

pub fn communicate() {
    let mut stop_engine = false;
    let start_search_flag = Arc::new(AtomicBool::new(false));
    let mut e: Engine = Engine {
        tbl: NoArc::new(MoveTable::default()),
        board: GameManager::default(),
        move_history: Vec::<UciMove>::new(),
        //set_new_game: false,
    };

    while !stop_engine {
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
                //For now, we'll reinitalize the engine's data
                //(minus movetable) each time we receive a
                //'position' command.
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
                search_control,
            } => {
                start_search_flag.store(true, Ordering::Relaxed);
                let flag = start_search_flag.clone();

                if let Some(timectl) = time_control {
                    match timectl {
                        vampirc_uci::UciTimeControl::Infinite => {
                            for depth in 0..20
                            /* or any big number */
                            {
                                root_negamax(depth, &e.board, &e.tbl, flag.clone());
                            }
                        }
                        vampirc_uci::UciTimeControl::MoveTime(time) => {}
                        vampirc_uci::UciTimeControl::Ponder => unimplemented!(),
                        vampirc_uci::UciTimeControl::TimeLeft { .. } => unimplemented!(),
                    }
                }
            }
            UciMessage::Stop => {
                start_search_flag.store(false, Ordering::Relaxed);
            }
            UciMessage::Quit => {
                start_search_flag.store(false, Ordering::Relaxed);
                stop_engine = true;
            }
            _ => {
                println!("Some other message was received.");
            }
        }
    }
}

fn make_move(board: &GameManager, tbl: &NoArc<MoveTable>, m: UciMove) -> GameManager {
    let h_from = Square::from_str(&m.from.to_string()).unwrap();
    let h_to = Square::from_str(&m.to.to_string()).unwrap();
    let legal_moves = board.legal_moves(tbl);
    let updated_data = legal_moves
        .iter()
        .find(|data| {
            data.1 == h_from
                && data.2 == h_to
                && match m.promotion {
                    Some(p) => match p {
                        UciPiece::Knight => {
                            if m.from.rank != m.to.rank {
                                //if the ranks are not the same
                                //then this was a promoting pawn capture
                                data.3 == MoveType::NPromoCapture
                            } else {
                                data.3 == MoveType::NPromotion
                            }
                        }
                        UciPiece::Bishop => {
                            if m.from.rank != m.to.rank {
                                //if the ranks are not the same
                                //then this was a promoting pawn capture
                                data.3 == MoveType::BPromoCapture
                            } else {
                                data.3 == MoveType::BPromotion
                            }
                        }
                        UciPiece::Rook => {
                            if m.from.rank != m.to.rank {
                                //if the ranks are not the same
                                //then this was a promoting pawn capture
                                data.3 == MoveType::RPromoCapture
                            } else {
                                data.3 == MoveType::RPromotion
                            }
                        }
                        UciPiece::Queen => {
                            if m.from.rank != m.to.rank {
                                //if the ranks are not the same
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
