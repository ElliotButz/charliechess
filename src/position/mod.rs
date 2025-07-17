use crate::position::{
    board::Board,
    coordinates::{Square, SquareVec},
    moves::*,
    pieces::{Piece, PieceKind::*}
};

pub mod board;
pub mod color;
pub mod coordinates;
pub mod pieces;
pub mod history;
pub mod coup;
pub mod moves;

pub fn moves_for_piece_at_square(board: &Board, square: Square) -> (SquareVec, Vec<Piece>) {
    let piece = board.piece_at(square).expect("Square is empty, can not enumerate moves for piece at square.");
    match piece.kind{
        Pawn   =>     pawn_moves::pawn_reachable_squares(board, square, piece.color),
        Tower  =>   tower_moves::tower_reachable_squares(board, square, piece.color),
        Knight => knight_moves::knight_reachable_squares(board, square, piece.color),
        Bishop => bishop_moves::bishop_reachable_squares(board, square, piece.color),
        King   =>     king_moves::king_reachable_squares(board, square, piece.color),
        Queen  =>   queen_moves::queen_reachable_squares(board, square, piece.color),
    }
}