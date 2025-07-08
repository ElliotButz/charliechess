extern crate num_derive;
pub mod position;

use crate::position::board::Board;

fn main() {
    let board = Board::at_start_state();
    board.terminal_display();
}