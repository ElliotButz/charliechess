use std::collections::HashMap;

use crate::position::{
    board::types_and_structs::Board, coordinates::types_and_structs::{Square, SquareVec}, coup::Coup, moves::basic_piece_moves::*, pieces::{Piece, PieceKind::*}
};
pub fn all_moves(board: &Board) -> Vec<Coup> {
    // Returns all possible moves (aka coups) for both player.
    let mut moves: Vec<Coup> = Vec::new(); 
    for (&square, &_piece) in board.map.iter() { 
        let (targetable_squares, _pieces_in_sight ) = basic_moves_for_piece_at_square(board, square);
        for &target_square in targetable_squares.iter() { 
            moves.push( 
                Coup {
                    start: square,
                    end: target_square,
                    piece: board.piece_at(square),
                    taken: board.opt_piece_at(target_square)
                })
        }
    };
    moves
}

pub fn threatened_squares(board: &Board) -> SquareVec {
    // Return all threatened squares, tipically to determine castle rights. 
    all_moves(board).iter().map(|coup| coup.end).collect()
}
/* 
pub fn castle_possibilities(board:&Board) -> HashMap<str, bool> {
    todo!()
} */