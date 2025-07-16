use crate::{coords};
use crate::position::color::Color;
use crate::position::coordinates::{Coords, CoordsVec, Square, SquareVec};
use crate::position::board::Board;
use crate::position::coordinates::SquareVecEquivalent;

pub fn knight_reachable_squares(board:&Board, square:Square, color:Color) -> SquareVec {
    // Makes a vector of squares coordinates where a Knight can go, given its start squart, its color, and its board.
    // This function does not take in account the fact that the piece might be pined, since it's more efficient to calculate the reachable 
    // squares of the pieces that are not pinned only.
    let start_coords= coords!(square);
    let mut in_reach: CoordsVec= vec![(2,-1), (2,1), (-2,-1), (-2,1), (1,2), (-1,2), (1,-2), (-1,-2)].iter().map(|&m|coords!(m) + start_coords).collect();
    in_reach.retain(|&s| s.in_board());
    let mut squares_in_reach = in_reach.to_square_vec();
    squares_in_reach.retain(|&s| board.square_is_free_for_piece_of_color(s, color));
    squares_in_reach
}