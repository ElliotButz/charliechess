use std::collections::HashMap;

use crate::position::{
    board::types_and_structs::Board, coordinates::types_and_structs::{Square,SquareVec}, pieces::{Piece, PieceKind::*}
};

pub mod pawn_moves;
pub mod knight_moves;
pub mod bishop_moves;
pub mod tower_moves;
pub mod queen_moves;
pub mod king_moves;

pub fn basic_moves_for_piece_at_square(board: &Board, square: Square) -> (SquareVec, Vec<Piece>) {
    // Returns the squares a piece can target and the pieces in sight. Castle is not treated.
    // Expect a piece at input square.
    let piece = board.piece_at(square);
    basic_moves_for_piece_from_square(board, square, piece)
}

pub fn basic_moves_for_piece_from_square(board: &Board, square: Square, piece: Piece) -> (SquareVec, Vec<Piece>) {
    // Return a Vec of the square that a piece could reach from a given square, and the pieces it would have in sight.
    // Do not need the piece to really be at the square. Castle is not treated.
    match piece.kind{
        Pawn   =>   pawn_moves::reachable_squares(board, square, piece.color),
        Tower  =>  tower_moves::reachable_squares(board, square, piece.color),
        Knight => knight_moves::reachable_squares(board, square, piece.color),
        Bishop => bishop_moves::reachable_squares(board, square, piece.color),
        King   =>   king_moves::reachable_squares(board, square, piece.color),
        Queen  =>  queen_moves::reachable_squares(board, square, piece.color),
    }
}