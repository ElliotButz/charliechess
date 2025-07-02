use std::collections::HashMap;

use crate::position::color::Color::{white, black};
use crate::position::coordinates::{Coords, Row, Column};
use crate::position::piece::Piece;
use crate::position::piece::PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King};


pub struct Board {
    piece_by_coord: Hashmap::<Coors:Piece>
}

impl Board {

    pub fn new(&self) -> Board {
        for col in Column::iter(): // Make pawns lines
            &self.piece_by_coord.insert(Coords {col, Row::R2 }, Piece::new(white, Pawn ))
            &self.piece_by_coord.insert(Coords {col, Row::R7 }, Piece::new(black, Pawn ))
    }

    fn piece_at_coords(&self, coords: Coords)->Option<Piece> {
        &self.piece_by_coord.get(coords);
    }
}




