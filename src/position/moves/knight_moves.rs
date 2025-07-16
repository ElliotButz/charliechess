use crate::position::color::Color;
use crate::position::coordinates::{Coords, CoordsVec, Square, SquareVec};
use crate::position::board::Board;
use crate::position::pieces::Piece;
use crate::position::coordinates::SquareVecEquivalent;

pub fn knight_reachable_squares(board:&Board, start: Square, color:Color) -> (SquareVec, Vec<Piece>) {
    // Makes a vector of squares coordinates where a Knight can go, given its start squart, its color, and its board.
    // This function does not take in account the fact that the piece might be pined, since it's more efficient to calculate the reachable 
    // squares of the pieces that are not pinned only.

    let jumps: Vec<(i8, i8)> = vec![(2,-1), (2,1), (-2,-1), (-2,1), (1,2), (-1,2), (1,-2), (-1,-2)];
    let mut coords_in_reach: CoordsVec = jumps.iter().map(|&jump|Coords::from(start + jump)).collect();
    coords_in_reach.retain(|&target| target.in_board());
    board.targetables_and_stared_pieces(
        coords_in_reach.to_square_vec(),
        color.the_other()
    )
}