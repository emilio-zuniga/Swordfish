use std::io::{self, BufRead};
use vampirc_uci::{UciFen, UciMessage, UciMove, UciSquare}; //MessageList

pub fn uci_tester() {
    println!("Waiting for messages...");

    for line in io::stdin().lock().lines() {
        let msg = vampirc_uci::parse_one(&line.unwrap());

        match msg {
            UciMessage::Uci => {
                println!("id name Swordfish");
                println!("id author Emilio Zuniga, Ethan Barry, Eric Oliver, Grace Kizer, & Zachary Wilson");
                //if we want configurable options, then we would list them following this comment
                println!("uciok");
            }
            UciMessage::IsReady => {
                //initalize movetable & setup everything else, then respond back with "readyok"
                println!("readyok")
            },
            UciMessage::UciNewGame => {
                //initialize save state for new game & wait for position
                //first value that is read in will be 
            }
            UciMessage::Position { startpos, fen, moves } => {
                if startpos {
                    // check to see if ucinewgame was previously called
                    // if so, setup default position
                    // if not, check moves & resume from last move
                } else {
                    // check to see if ucinewgame was previously called
                    // if so, setup FEN encoded position
                    // if not, check moves & resume from last move
                }
            }
            UciMessage::Go { time_control, search_control } => {
                match time_control {
                    _ => (),
                }
                match search_control {
                    _ => todo!(),
                }
                //engine actually does thinking/computing here
                //sends info regularly until "stop" message is received
            }
            UciMessage::Stop => {
                //engine should print info about last depth searched to
                //then, send "bestmove 'move'" (we may include ponder here)
                todo!()
            },
            UciMessage::Quit => todo!(), //engine should shutdown

            _ => println!("Received message: {msg}")
        }
    }
    
    println!("We made it out the loop");
}

fn uci_test() {
    let fen: UciFen = UciFen(String::from("k7/8/8/4n3/8/3N4/RN6/K7 w - - 0 1"));
    let move_list: Vec<UciMove> = vec![UciMove::from_to(UciSquare::from('e', 2), UciSquare::from('e', 4))];
    let msg: UciMessage = UciMessage::Position { startpos:false, fen: Some(fen), moves: move_list};
    
    println!("UCI Message: {msg}");
}

fn test_process() {
    println!("Starting a process...");
    loop {}
}
