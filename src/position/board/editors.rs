use std::collections::HashMap;
use strum::IntoEnumIterator;
use colored::{ColoredString, Colorize};
use std::fmt;

use crate::position::coup::Coup;
use crate::{square, piece};
use crate::position::coordinates::types_and_structs::{Column, Column::*, Row, Row::*, Square, SquareVec, Coords, CoordsVec};
use crate::position::coordinates::converters::SquareVecEquivalent;
use crate::position::color::{Color,Color::{White, Black}};
use crate::position::pieces::{Piece, PieceKind, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};
use crate::position::board::types_and_structs::Board;


impl Board { // Editors

    fn extract_piece_of_square(&mut self, square: Square) -> Piece {
        // Remove a Piece from a square and return it.
        self.extract_optionnal_piece_of_square(square).expect("Tried to extract Piece from an emtpy square in boardmap.")
    }

    fn extract_optionnal_piece_of_square(&mut self, square: Square) -> Option<Piece> {
        // Remove an Option<Piece> from a square and return it. It will be None if no Piece was on the square.
        self.map.remove(&square)
    }

    fn add_piece_at_coords(&mut self,  coords: Square, piece: Piece) {
        self.map.insert(coords, piece);
    }

    pub fn move_piece(&mut self, start_square: Square, target_square: Square) {
    /*
    1: Extract the Piece at start_square (it is displaced: Piece),
    2: Extract the possible Piece at target_square (it is taken : Option<Piece>),
    3: Place the Piece displaced from start_square at target_square,
    4: Update board.
    */
        let displaced: Piece = self.extract_piece_of_square(start_square);
        let taken: Option<Piece> = self.extract_optionnal_piece_of_square(target_square);
        self.add_piece_at_coords(target_square, displaced);
        self.last_move = Coup {
            start: start_square,
            end  : target_square,
            piece: displaced,
            taken: taken,
            checks: self.piece_checks_king(target_square)
        };
    }

}