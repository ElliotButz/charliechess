use crate::{position::{board::types_and_structs::Board, history::History}};

pub mod board;
pub mod color;
pub mod coordinates;
pub mod pieces;
pub mod history;
pub mod coup;
pub mod basic_piece_moves;

pub struct Position {
    board: Board,
    history: History
}

impl Position {

    fn check_draw(&self) {
        todo!()
    }
}