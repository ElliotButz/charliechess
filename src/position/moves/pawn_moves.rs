extern crate unwrap;
use unwrap::unwrap;

use crate::{coords};
use crate::position::color::{Color,Color::{White, Black}};
use crate::position::board::Board;
use crate::position::coup::Coup;
use crate::position::coordinates::types_and_structs::{Square, SquareVec, Row::*};
use crate::position::pieces::Piece;

pub fn reachable_squares(board:&Board, start:Square, color:Color) -> (SquareVec, Vec<Piece>) {
    // Makes a vector of squares coordinates where a pawn can go, given its start squart, its color, and the board to which the pawn belongs.
    // This function does not take in account the fact that the pawn might be pined, since it's more efficient to calculate the reachable 
    // squares of the pieces that are not pinned only.
    let direction: i8 = color.as_direction();
    let (start_colidx, _start_rowidx) = start.into();
    let mut in_reach = SquareVec::with_capacity(4);
    let mut found_pieces: Vec<Piece> = Vec::with_capacity(3); // There are max 3 pieces that can block the movement of a pawn or get attacked by him.
    
    if start.row == R1 || start.row == R8 { return (in_reach, found_pieces) }

    // One square forward 
    let next_square: Square = start + (0, direction); // square in front of the pawn.
    let next_square_is_free: bool = board.square_is_free(next_square);
    if next_square_is_free { in_reach.push(next_square) } else {
        found_pieces.push(unwrap!(board.piece_at(next_square), "Expected a piece at {}, found None.", next_square))} ;

    // Normal take
    for colshift in [-1i8,1i8] {
        if (coords!(next_square) + (colshift, 0)).not_in_board() {continue}
        let potential_target_square = next_square + (colshift, 0) ;
        match board.color_of_piece_at(potential_target_square) {
            Some(target_piece_color) => {
                if target_piece_color != color {
                    in_reach.push(potential_target_square);
                    found_pieces.push(
                        unwrap!(board.piece_at(potential_target_square), "Expected a piece at {}, found None.", potential_target_square)
                    );
                };
            },
            None => (),
        }
    }

    match (start.row, color) { 
        // Pawn no jutsu: Double step !
        (R2, White) | (R7, Black) => {
            let double_step_target = start + (0, 2*direction) ;

            if next_square_is_free { 
                if board.square_is_free(double_step_target) { in_reach.push(double_step_target) }
                else { found_pieces.push(
                    unwrap!(
                        board.piece_at(double_step_target),
                        "Expected a piece at {}, found None.", double_step_target))
                    }
            }

        },
        // Pawn no jutsu: En Passant !
        (R5, White) | (R4, Black) => { 
            let last_move:Coup = board.last_move;
            let last_move_colidx = last_move.start.col as i8;
            let col_idx_diff = ( last_move_colidx - start_colidx ).abs();

            if  last_move.is_pawn_double_step() && col_idx_diff==1 { // If the last moove is a double step from a pawn on an adjacent column, En Passant is possible.
                let en_passant_target = start + (last_move_colidx as i8, direction) ;
                in_reach.push(en_passant_target);
                found_pieces.push(unwrap!(board.piece_at(en_passant_target), "Expected a piece at {}, found None.", en_passant_target));
            }
        },
        _ => {}
    };

    (in_reach, found_pieces)
} 