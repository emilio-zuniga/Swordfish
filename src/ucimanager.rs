use std::io::{self, BufRead};
use vampirc_uci::{UciFen, UciMessage, UciMove, UciSquare}; //MessageList

pub fn uci_tester() {
    println!("Startin' here");

    for line in io::stdin().lock().lines() {
        let msg = vampirc_uci::parse_one(&line.unwrap());

        if msg != UciMessage::Quit {
            println!("Received message: {}", msg);
        } else {
            break;
        }
    }

    println!("We made it out the loop");
}

pub fn uci_test() {
    let fen: UciFen = UciFen(String::from("k7/8/8/4n3/8/3N4/RN6/K7 w - - 0 1"));
    let move_list: Vec<UciMove> = vec![UciMove::from_to(UciSquare::from('e', 2), UciSquare::from('e', 4))];
    let msg: UciMessage = UciMessage::Position { startpos:false, fen: Some(fen), moves: move_list};
    
    println!("UCI Message: {msg}");
}
