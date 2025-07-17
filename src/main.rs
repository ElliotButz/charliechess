extern crate num_derive;

pub mod position;

use crate::position::{
    board::Board, color::Color, coordinates::{Column, Row, Square}, moves::*
};
fn main() {
    let mut board = Board::at_start_state();
    println!("{board}");
    board.move_piece(square!((Column::E,Row::R2)), square!((Column::E,Row::R4)));
    println!("{board}");
    println!("White Pawn E2 :{:?}", pawn_moves::pawn_reachable_squares(&board, square!((Column::E,Row::R2)), Color::White));
    println!("White knight B1: {:?}", knight_moves::knight_reachable_squares(&board, square!((Column::B,Row::R1)), Color::White));
    println!("White bishop F1: {:?}", bishop_moves::bishop_reachable_squares(&board, square!((Column::F,Row::R1)), Color::White));
    println!("White tower D5: {:?}", tower_moves::tower_reachable_squares(&board, square!((Column::D,Row::R5)), Color::White));
    println!("White queen D5: {:?}", queen_moves::queen_reachable_squares(&board, square!((Column::D,Row::R5)), Color::White));
    println!("White king D5: {:?}", king_moves::king_reachable_squares(&board, square!((Column::D,Row::R5)), Color::White));
    println!("Black pawn E2: {:?}", pawn_moves::pawn_reachable_squares(&board, square!((Column::E,Row::R2)),     Color::Black));
    println!("Black knight B1: {:?}", knight_moves::knight_reachable_squares(&board, square!((Column::B,Row::R1)), Color::Black));
    println!("Black bishop F1: {:?}", bishop_moves::bishop_reachable_squares(&board, square!((Column::F,Row::R1)), Color::Black));
    println!("Black tower D5: {:?}", tower_moves::tower_reachable_squares(&board, square!((Column::D,Row::R5)),   Color::Black));
    println!("Black queen D5: {:?}", queen_moves::queen_reachable_squares(&board, square!((Column::D,Row::R5)),   Color::Black));
    println!("Black king D5: {:?}", king_moves::king_reachable_squares(&board, square!((Column::D,Row::R5)),     Color::Black));

}