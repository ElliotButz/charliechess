extern crate num_derive;

pub mod position;
use crate::position::coordinates::displayers::vec2str;
use crate::position::coordinates::types_and_structs::{Row::*, Column::*};
use crate::position::board::types_and_structs::Board;
use crate::position::moves::basic_piece_moves::*;
use crate::position::moves::possible_moves_enumeration::{all_moves, update_castle_rights};
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
    board.move_piece(square!((B,R1)), square!((C,R3)));
    board.move_piece(square!((A,R5)), square!((A,R4)));
    board.move_piece(square!((B,R2)), square!((B,R4)));
    board.move_piece(square!((G,R8)), square!((F,R6)));
    board.move_piece(square!((F,R1)), square!((B,R5)));
    update_castle_rights(&mut board);


    println!("{board}");

    let (white_moves, black_moves) = all_moves(&board);
    println!("White moves: {}", vec2str(&white_moves));
    println!("Black moves: {}", vec2str(&black_moves));

    for (&square, piece) in board.map.iter() {
        let (moves, pieces_in_sight) = basic_moves_for_piece_at_square(&board, square);
        println!("{} {} {} {} {}", square, piece.color, piece.kind, vec2str(&moves), vec2str(&pieces_in_sight))
    }

}