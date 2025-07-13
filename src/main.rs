extern crate num_derive;

pub mod position;

use crate::position::{
    board::Board,
    coordinates::{
        Square, Row::*, Column::*},
    color::{
        Color::{
            White, Black}
    },
    moves::pawn_moves
};
fn main() {
    let mut board = Board::at_start_state();
    board.terminal_display();
    board.move_piece(square!(E,R2), square!(E,R4));
    board.terminal_display();
    let possibles = pawn_moves::pawn_reachable_squares(&board, square!(E,R2), White);
}