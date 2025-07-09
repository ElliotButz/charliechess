use crate::{coords, piece};
use crate::position::color::{Color,Color::{White, Black}};
use crate::position::coordinates::{Coords, Row, Row::*, Column, Column::*, CoordsVec};
use crate::position::pieces::{Piece, PieceKind, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};
use crate::position::board::BoardMap;

fn pawn_reachable_squares(boardmap:&BoardMap, start:&Coords, color:&Color) -> CoordsVec{
    let direction: i8 = color.as_direction();
    let mut in_reach = CoordsVec::with_capacity(4);
    
    let is_at_start: bool = match (start.row, color) {
        (R2, White) | (R7, Black) => true,
        _ => false
    };

    let (start_row_idx, start_col_idx) = start.to_colrow_idx();
    if is_at_start {
        let two_forward = Coords::from_colrow_idx(start_col_idx+2*direction, start_col_idx);
    }
    
    in_reach
} 