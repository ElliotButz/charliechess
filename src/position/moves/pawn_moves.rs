use crate::{idxcoords};
use crate::position::color::{Color,Color::{White, Black}};
use crate::position::coordinates::{coord, Square, Row::*, SquareVec, CoordsVec, Coords, CoordsVecEquivalent};
use crate::position::board::Board;
use crate::position::coup::Coup;


pub fn pawn_reachable_squares(board:&Board, square:Square, color:Color) -> SquareVec {
    // Makes a vector of squares coordinates where a pawn can go, given its start squart, its color, and the board to which the pawn belongs.
    // This function does not take in account the fact that the pawn might be pined, since it's more efficient to calculate the reachable 
    // squares of the pieces that are not pinned only.
    let direction: i8 = color.as_direction();
    let (square_rowidx, square_colidx) = square.to_coord_couple();
    let mut in_reach = CoordsVec::with_capacity(4);

    // One square forward 
    let next_square: Coords = idxcoords!(square_colidx, square_rowidx + direction); // square in front of the pawn.
    let next_square_is_free: bool = board.square_is_free(Square::from_coords(next_square));
    if  next_square_is_free { in_reach.push(next_square) } ;

    // Normal take
    for colshift in [-1i8,1i8] {
        let potential_target_square = next_square + idxcoords!(colshift, 0) ;
        match board.color_of_piece_at(Square::from_coords(potential_target_square)) {
            Some(target_piece_color) => {if color != target_piece_color {in_reach.push(potential_target_square)};},
            None => (),
        }
    }

    match (square.row, color) { 
        // If pawn is at its start pos, it can take 2 steps forward TODO :CHECK SQUARE IS EMPTY
        (R2, White) | (R7, Black) => {
            if next_square_is_free 
            && board.square_is_free(Square::from_coords(idxcoords!(square_colidx, square_rowidx + 2*direction)))
            {in_reach.push(idxcoords!(square_colidx, square_rowidx+2*direction))}; },
        // Pawn no jutsu: En Passant
        (R5, White) | (R4, Black) => { 
            let last_move:Coup = board.last_move;
            let last_move_colidx = coord(last_move.start.col);
            let col_idx_diff = ( last_move_colidx - square_colidx ).abs();
            if  last_move.is_pawn_double_step() && col_idx_diff==1 { // If 
                in_reach.push(idxcoords!(square_colidx, square_rowidx + direction))
            }
        },
        _ => {}
    };

    in_reach.to_coords_vec()
} 
