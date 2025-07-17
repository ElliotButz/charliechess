extern crate num_derive;

pub mod position;

use crate::position::{board::Board, coordinates::{Column::*, Row::*, Square}};
fn main() {
    let mut board = Board::at_start_state();
    board.move_piece(square!((E,R2)), square!((E,R4)));
    board.move_piece(square!((E,R7)), square!((E,R5)));
    board.move_piece(square!((G,R1)), square!((F,R3)));
    board.move_piece(square!((B,R8)), square!((C,R6)));
    board.move_piece(square!((D,R2)), square!((D,R4)));
    board.move_piece(square!((E,R5)), square!((D,R4)));
    board.move_piece(square!((F,R3)), square!((D,R4)));
    board.move_piece(square!((A,R7)), square!((A,R5)));

    println!("{board}");

    for (square, piece) in board.map.iter() {
        println!("{:?} {:?} {:?}", square, piece.color, piece.kind);

        println!("{:?} {:?} {:?} {:?}", square, piece.color, piece.kind, position::moves_for_piece_at_square(&board, *square))
    }


}