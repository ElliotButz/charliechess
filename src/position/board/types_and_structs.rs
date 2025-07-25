use std::collections::HashMap;

use crate::position::{
    color::Color,
    coup::Coup,
    coordinates::types_and_structs::{Square, SquareVec},
    pieces::{Piece}
};

pub type BoardMap = HashMap<Square,Piece>;
#[allow(dead_code)]
pub struct Board {
    pub map: BoardMap,
    pub player_to_play: Color,
    pub last_move: Coup,
    pub black_king_has_moved: bool,
    pub white_king_has_moved: bool,
    pub h_black_tower_has_moved: bool,
    pub a_black_tower_has_moved: bool,
    pub h_white_tower_has_moved: bool,
    pub a_white_tower_has_moved: bool,    

    pub black_king_can_h_rook: bool,
    pub black_king_can_a_rook: bool,
    pub white_king_can_h_rook: bool,
    pub white_king_can_a_rook: bool,
    pub squares_with_pined_pieces : SquareVec,
    pub squares_with_pining_pieces: SquareVec   
}