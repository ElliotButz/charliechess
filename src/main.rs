extern crate num_derive;

pub mod position;

use crate::position::{
    board::Board, color::Color, coordinates::{Column, Row, Square}, moves::{tower_moves, bishop_moves, knight_moves, pawn_moves}
};
fn main() {
    let mut board = Board::at_start_state();
    board.terminal_display();
    board.move_piece(square!((Column::E,Row::R2)), square!((Column::E,Row::R4)));
    board.terminal_display();
    pawn_moves::pawn_reachable_squares(&board, square!((Column::E,Row::R2)), Color::White);
    knight_moves::knight_reachable_squares(&board, square!((Column::B,Row::R1)), Color::White);
    bishop_moves::bishop_reachable_squares(&board, square!((Column::F,Row::R1)), Color::White);
    tower_moves::tower_reachable_squares(&board, square!((Column::D,Row::R5)), Color::White);

}