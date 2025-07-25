use crate::position::color::Color;
use crate::position::board::types_and_structs::Board;
use crate::position::coordinates::{
    converters::SquareVecEquivalent,
    types_and_structs::{Coords, CoordsVec, Square, SquareVec}};
use crate::position::pieces::Piece;

pub fn reachable_squares(board:&Board, start:Square, color:Color) -> (SquareVec, Vec<Piece>) {
    // Makes a vector of squares coordinates where a King can go, given its start squart, its color, and its board.
    // This function does not take in account the fact that the piece might be pined, since it's more efficient to calculate the reachable 
    // squares of the pieces that are not pinned only.
    let cstart =Coords::from(start);
    let steps: Vec<(i8, i8)> = vec![(-1,0), (1,0), (0,-1), (0,1), (-1,-1), (1,1), (1,-1), (-1,1)];
    let mut coords_in_reach: CoordsVec = steps.iter().map(|&step|cstart + step).collect();
    coords_in_reach.retain(|&target| target.in_board());
    board.targetables_and_stared_pieces(
        coords_in_reach.to_square_vec(),
        color.the_other()
    )
}