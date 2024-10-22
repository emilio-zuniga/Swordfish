use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

use seed::{prelude::*, *};

struct ChessUI {
    board: [[char; 8]; 8],
}

pub enum Msg {}

fn init(_: Url, _: &mut impl Orders<Msg>) -> ChessUI {
    let mut board = [['.'; 8]; 8];

    // Initialize pawns
    board[1] = ['P'; 8];  // White pawns
    board[6] = ['p'; 8];  // Black pawns

    // Initialize white pieces
    board[0][0] = 'R';  // Rook
    board[0][7] = 'R';  // Rook
    board[0][1] = 'N';  // Knight
    board[0][6] = 'N';  // Knight
    board[0][2] = 'B';  // Bishop
    board[0][5] = 'B';  // Bishop
    board[0][3] = 'Q';  // Queen
    board[0][4] = 'K';  // King

    // Initialize black pieces
    board[7][0] = 'r';  // Rook
    board[7][7] = 'r';  // Rook
    board[7][1] = 'n';  // Knight
    board[7][6] = 'n';  // Knight
    board[7][2] = 'b';  // Bishop
    board[7][5] = 'b';  // Bishop
    board[7][3] = 'q';  // Queen
    board[7][4] = 'k';  // King

    ChessUI { board }
}

fn update(_msg: Msg, _model: &mut ChessUI, _: &mut impl Orders<Msg>) {}

fn view(model: &ChessUI) -> Node<Msg> {
    div![
        // Outer container to center the board
        style! {
            St::Display => "flex",
            St::JustifyContent => "center",
            St::AlignItems => "center",
            St::Height => "100vh",  // Full viewport height
        },
        // Inner container for the chessboard
        div![
            style! {
                St::Display => "grid",
                St::GridTemplateColumns => "repeat(8, 50px)",
                St::Gap => "5px",
            },
            model.board.iter().enumerate().flat_map(|(row, pieces)| {
                pieces.iter().enumerate().map(move |(col, &piece)| {
                    let is_white_square = (row + col) % 2 == 0;
                    let background_color = if is_white_square { "white" } else { "gray" };
                    let piece_color = if piece.is_uppercase() { "black" } else { "white" };

                    div![
                        style! {
                            St::BackgroundColor => background_color,
                            St::Color => piece_color,
                            St::Width => px(50),
                            St::Height => px(50),
                            St::Display => "flex",
                            St::AlignItems => "center",
                            St::JustifyContent => "center",
                            St::FontSize => px(20),
                        },
                        piece.to_string()
                    ]
                }).collect::<Vec<_>>() // Collect the iterator to avoid borrowing issues
            }).collect::<Vec<_>>() // Collect the outer iterator
        ]
    ]
}


