use vampirc_uci::UciMove;

use crate::{gamemanager::GameManager, movetable::{noarc::NoArc, MoveTable}};

pub struct Engine {
    pub tbl: NoArc<MoveTable>,
    pub move_history: Vec<UciMove>,
    pub board: GameManager,
    pub set_new_game: bool,
}

impl Default for Engine {
    fn default() -> Engine{
        Engine {
            tbl: NoArc::new(MoveTable::default()),
            board: GameManager::default(),
            move_history: Vec::<UciMove>::new(),
            set_new_game: false,
        }
    }
}

