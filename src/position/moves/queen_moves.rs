use crate::position::color::Color;
use crate::position::coordinates::types_and_structs::{Square, SquareVec};
use crate::position::board::Board;
use crate::position::pieces::Piece;

pub fn reachable_squares(board:&Board, start:Square, color:Color) -> (SquareVec, Vec<Piece>) {
    // Makes a vector of squares coordinates where a Queen can go, given its start squart, its color, and its board.
    // This function does not take in account the fact that the piece might be pined, since it's more efficient to calculate the reachable 
    // squares of the pieces that are not pinned only.
    board.step_in_directions_til_target(start, vec![(-1,0), (1,0), (0,-1), (0,1), (-1,-1), (1,1), (1,-1), (-1,1)], color.the_other())
}