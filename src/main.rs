use gamemanager::GameManager;

mod gamemanager;
mod bitboard;

fn main() {
    //Test FENs:
    /* r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2
     * 
     */
    let fen = "r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2";
    let g = GameManager::from_fen_string(fen);
}
