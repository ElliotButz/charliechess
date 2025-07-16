use crate::position::pieces::Piece;
use crate::{coords};
use crate::position::color::Color;
use crate::position::coordinates::{Coords, CoordsVec, Square, SquareVec};
use crate::position::board::Board;
use crate::position::coordinates::SquareVecEquivalent;

pub fn bishop_reachable_squares(board:&Board, start:Square, color:Color) -> SquareVec {
    // Makes a vector of squares coordinates where a Knight can go, given its start squart, its color, and its board.
    // This function does not take in account the fact that the piece might be pined, since it's more efficient to calculate the reachable 
    // squares of the pieces that are not pinned only.
    let directions: Vec<(i8, i8)>= vec![(-1,-1), (1,1), (1,-1), (-1,1)];
    let mut in_reach= CoordsVec::with_capacity(14);
    
    for &dir in directions.iter() {
        let (mut in_path, _found_piece) = board.step_til_piece_of_color(start, dir.into(), color.the_other());
        in_reach.append(&mut in_path);
    }

    in_reach.to_square_vec()
}