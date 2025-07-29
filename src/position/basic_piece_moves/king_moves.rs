use crate::position::color::Color;
use crate::position::board::types_and_structs::Board;
use crate::position::coordinates::converters::to_square_vec;
use crate::position::coordinates::displayers::vec2str;
use crate::position::coordinates::{
    types_and_structs::{Coords, CoordsVec, Square, SquareVec}};
use crate::position::pieces::Piece;

pub fn reachable_squares(board:&Board, start:Square, color:Color) -> (SquareVec, Vec<Piece>) {
    // Makes a vector of squares coordinates where a King can go, given its start squart, its color, and its board.
    let cstart =Coords::from(start);
    let steps: Vec<(i8, i8)> = vec![
        (-1, 1), ( 0, 1), ( 1, 1),
        (-1, 0), /*King*/ ( 1, 0),
        (-1,-1), ( 0,-1), ( 1,-1)
        ];
    let mut coords_in_reach: CoordsVec = steps.iter().map(|&step|cstart + step).collect();
    coords_in_reach.retain(|&target| target.in_board());
    let (candidate_targets, stared_pieces) = board.targetables_and_stared_pieces(
        to_square_vec(&coords_in_reach),
        color.the_other()
    );
    /* Bad idea, it prevent the king to take an opponent that would check him.
    candidate_targets.retain(|&candidate|
        board.square_is_in_sight_of_opponent(candidate, color.the_other())); */
    println!("{}, {}",color, vec2str(&candidate_targets));
    (candidate_targets, stared_pieces)
}