use crate::types::{MoveType, Square};
use crate::{
    enginemanager::Engine,
    gamemanager::GameManager,
    movetable::{noarc::NoArc, MoveTable},
};
use std::io::{self, BufRead};
use std::sync::atomic::AtomicBool;
use vampirc_uci::{UciMessage, UciMove, UciPiece};

pub fn communicate() {
    let mut flag = AtomicBool::new(false);

    let mut e: Engine = Engine {
        tbl: NoArc::new(MoveTable::default()),
        board: GameManager::default(),
        move_history: Vec::<UciMove>::new(),
        set_new_game: false,
    };

    for line in io::stdin().lock().lines() {
        let msg = vampirc_uci::parse_one(&line.unwrap());

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
                e.set_new_game = true;
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
                e.move_history = moves;

                for m in e.move_history {
                    let h_from = Square::from_str(&m.from.to_string()).unwrap();
                    let h_to = Square::from_str(&m.to.to_string()).unwrap();
                    let legal_moves = e.board.legal_moves(&e.tbl);
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

                    e.board = updated_data.4.clone();
                }

                e.set_new_game = false;
            }
            UciMessage::Go {
                time_control: _,
                search_control: _,
            } => {
                //engine start search on current position...
                //engine regularly sends info
            }
            UciMessage::Stop => {
                *flag.get_mut() = true;
                println!("bestmove (move name here)");
            }
            UciMessage::Quit => break,
            _ => eprintln!("Received message: {msg}"),
        }
    }
}
