use std::io::{self, BufRead};
use vampirc_uci::{UciFen, UciMessage, UciMove, UciSquare};

pub fn communicate() {
    println!("Waiting for messages...");

    for line in io::stdin().lock().lines() {
        let msg = vampirc_uci::parse_one(&line.unwrap());

        match msg {
            UciMessage::Uci => {
                println!("id name Swordfish");
                println!("id author Emilio Zuniga, Ethan Barry, Eric Oliver, Grace Kizer, & Zachary Wilson");
                // if we want configurable options, then we would list them here
                println!("uciok");
            }
            UciMessage::IsReady => {
                // a function should initalize the movetable for the engine
                // as well as set up any internal parameters
                println!("readyok")
            }
            UciMessage::UciNewGame => {
                // a function call here should clear the move history
                // and starting FEN of any ongoing game
                // - the history should just be set to an empty vec
                // - the saved inital FEN should be reset to a None value (we will need to create this)
                // then, the engine should wait for the GUI
                // to send the inital position
                // - that position should be saved as the starting FEN
            }
            UciMessage::Position {
                startpos,
                fen,
                moves,
            } => {
                
                // if the saved inital position is a None value,
                // then, set the above position as the inital position
                // - if startpos is given as true, then just use standard board's FEN
                // - otherwise, grab the given FEN
                
                // if so, setup default position
                // if not, check moves & resume from last move
            }
            UciMessage::Go {
                time_control,
                search_control,
            } => {
                match time_control {
                    _ => (),
                }
                match search_control {
                    _ => todo!(),
                }
                // engine actually does thinking/computing here
                // sends info regularly until "stop" message is received
            }
            UciMessage::Stop => {
                // engine should print info about last depth searched to
                // then, send "bestmove 'move'" (we may include ponder here)
                todo!()
            }
            UciMessage::Quit => todo!(), //engine should shutdown

            _ => println!("Received message: {msg}"),
        }
    }

    println!("We made it out the loop");
}

fn crate_investigation() {
    let fen: UciFen = UciFen(String::from("k7/8/8/4n3/8/3N4/RN6/K7 w - - 0 1"));
    let move_list: Vec<UciMove> = vec![UciMove::from_to(
        UciSquare::from('e', 2),
        UciSquare::from('e', 4),
    )];
    let msg: UciMessage = UciMessage::Position {
        startpos: false,
        fen: Some(fen),
        moves: move_list,
    };

    println!("UCI Message: {msg}");
}
