extern crate num_derive;

pub mod position;

use crate::position::color::Color;
use crate::position::coordinates::displayers::vec2str;
//use crate::position::coordinates::types_and_structs::{Row::*, Column::*};
use crate::position::board::types_and_structs::Board;
use crate::position::coup::Coup;
fn main() {
    let mut board = Board::at_start_state();
/*     board.move_piece(square!((E,R2)), square!((E,R4)));
    board.move_piece(square!((E,R7)), square!((E,R5)));
    board.move_piece(square!((G,R1)), square!((F,R3)));
    board.move_piece(square!((B,R8)), square!((C,R6)));
    board.move_piece(square!((D,R2)), square!((D,R4)));
    board.move_piece(square!((E,R5)), square!((D,R4)));
    board.move_piece(square!((F,R3)), square!((D,R4)));
    board.move_piece(square!((A,R7)), square!((A,R5)));
    board.move_piece(square!((B,R1)), square!((C,R3)));
    board.move_piece(square!((A,R5)), square!((A,R4)));
    board.move_piece(square!((B,R2)), square!((B,R4)));
    board.move_piece(square!((G,R8)), square!((F,R6))); */

    println!("{board}");

    let (white_moves, black_moves) = board.all_moves();
    println!("White moves: {}", vec2str(&white_moves));
    println!("Black moves: {}", vec2str(&black_moves));

    println!("{}", Coup::try_from(("h7-h6", &mut board, Color::Black)).unwrap());
}