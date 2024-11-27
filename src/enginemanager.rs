use vampirc_uci::UciMove;

use crate::{gamemanager::GameManager, movetable::{noarc::NoArc, MoveTable}};

pub struct Engine {
    pub tbl: NoArc<MoveTable>,
    pub move_history: Vec<UciMove>,
    pub board: GameManager,
    pub set_new_game: bool,
}
