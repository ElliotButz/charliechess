extern crate num_derive;

pub mod position;

use crate::position::{
    board::Board,
    coordinates::{Square, Row, Column},
    color::Color,
    moves::pawn_moves
};
fn main() {
    let mut board = Board::at_start_state();
    board.terminal_display();
    board.move_piece(square!((Column::E,Row::R2)), square!((Column::E,Row::R4)));
    board.terminal_display();
    pawn_moves::pawn_reachable_squares(&board, square!((Column::E,Row::R2)), Color::White);
}