use crate::{position::{board::types_and_structs::Board, coordinates::types_and_structs::Column, coup::{Coup, CoupKind::*}, history::History}, square};

pub mod board;
pub mod color;
pub mod coordinates;
pub mod pieces;
pub mod history;
pub mod coup;
pub mod moves;

pub struct Position {
    board: Board,
    history: History
}

impl Position {

    fn check_draw(&self) {
        todo!()
    }
}