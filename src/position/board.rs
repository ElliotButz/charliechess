use std::collections::HashMap;

use crate::position::color::Color::{White, Black};
use crate::position::coordinates::{Coords, Row, Column};
use crate::position::pieces::{Piece, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};

pub struct Board {
    piece_by_coord: HashMap<Coords,Piece>,
}

impl Board {

    pub fn new() -> Board{

        let inital_state: HashMap<Coords,Piece>= HashMap::new();

        for column in Column::iter(){ // Make pawns lines
            let row2 = Row::R2;
            let row7 = Row::R7;
            inital_state.insert(Coords{column, row2}, Piece::new(White, Pawn));
            inital_state.insert(Coords{column, row7}, Piece::new(Black, Pawn));
        };
        Board{piece_by_coord:inital_state}
    }

    pub fn piece_at_coords(&self, coords: Coords)-> &Option<&Piece> {
        let piece = &self.piece_by_coord.get(coords);
        return &piece;
    }
}