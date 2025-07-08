extern crate num_derive;

pub mod position;
use crate::position::{
    board::Board,
    coordinates::{
        Coords, Row::*, Column::*}
};
fn main() {
    let mut board = Board::at_start_state();
    board.terminal_display();
    board.move_piece(&coords!(E,R2), &coords!(E,R4));
    board.terminal_display();
}