use std::io::{self, BufRead};
use vampirc_uci::UciMessage; //MessageList

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
