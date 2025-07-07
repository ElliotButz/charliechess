extern crate num_derive;
use num_traits::ToPrimitive;


pub mod position;

use crate::position::board::Board;

fn main() {
    let board = Board::at_start_state();
    board.terminal_display();
}