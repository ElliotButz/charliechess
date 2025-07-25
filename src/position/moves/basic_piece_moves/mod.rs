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