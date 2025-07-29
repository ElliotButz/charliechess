use crate::position::color::Color;
use crate::position::coordinates::types_and_structs::{Square, SquareVec};
use crate::position::board::types_and_structs::Board;
use crate::position::pieces::Piece;

pub fn reachable_squares(board:&Board, start:Square, color:Color) -> (SquareVec, Vec<Piece>) {
    // Makes a vector of squares coordinates where a Tower can go, given its start squart, its color, and its board.
    // This function does not take in account the fact that the piece might be pined, since it's more efficient to calculate the reachable 
    // squares of the pieces that are not pinned only.
    board.step_in_directions_til_target(start, vec![(-1,0), (1,0), (0,-1), (0,1)], color.the_other())
}

pub fn watched_squares(board: &Board, start: Square) -> SquareVec {
    // squares where the king can go if it is not blocked
    let directions: Vec<(i8, i8)> = vec![
                ( 0, 1),
        (-1, 0),/*Tower*/ ( 1, 0),
                ( 0,-1)
        ];
    board.step_in_directions_til_piece(start, directions).0
    }