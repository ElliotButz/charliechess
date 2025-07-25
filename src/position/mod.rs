use std::collections::HashMap;

use crate::position::{
    board::types_and_structs::Board, coordinates::types_and_structs::{Square,SquareVec}, coup::Coup, moves::*, pieces::{Piece, PieceKind::*}
};

pub mod board;
pub mod color;
pub mod coordinates;
pub mod pieces;
pub mod history;
pub mod coup;
pub mod moves;

pub fn moves_for_piece_at_square(board: &Board, square: Square) -> (SquareVec, Vec<Piece>) {
    // Returns the squares a piece can target and the pieces in sight.
    let piece = board.opt_piece_at(square).expect("Square is empty, can not enumerate moves for piece at square.");
    match piece.kind{
        Pawn   =>   pawn_moves::reachable_squares(board, square, piece.color),
        Tower  =>  tower_moves::reachable_squares(board, square, piece.color),
        Knight => knight_moves::reachable_squares(board, square, piece.color),
        Bishop => bishop_moves::reachable_squares(board, square, piece.color),
        King   =>   king_moves::reachable_squares(board, square, piece.color),
        Queen  =>  queen_moves::reachable_squares(board, square, piece.color),
    }
}

pub fn all_moves(board: &Board) -> Vec<Coup> {
    // Returns all possible moves (aka coups) for both player.
    /* collect here castel  determinants*/
    let mut moves: Vec<Coup> = Vec::new(); 
    for (&square, &_piece) in board.map.iter() { 
        let (targetable_squares, _pieces_in_sight ) = moves_for_piece_at_square(board, square);
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
/* 
pub fn castle_possibilities(board:&Board) -> HashMap<str, bool> {
    todo!()
} */