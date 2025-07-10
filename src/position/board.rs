use std::collections::HashMap;
use strum::IntoEnumIterator;
use colored::{ColoredString, Colorize};

use crate::position::coup::Coup;
use crate::{coords, piece};
use crate::position::color::{Color,Color::{White, Black}};
use crate::position::coordinates::{Coords, Row, Row::*, Column, Column::*, CoordsVec};
use crate::position::pieces::{Piece, PieceKind, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};

pub type BoardMap = HashMap<Coords,Piece>;
pub struct Board {
    map: BoardMap,
    last_move: Option<Coup>,
    black_king_has_moved: bool,
    black_a_tower_has_moved: bool,
    black_h_tower_has_moved: bool,
    white_king_has_moved: bool,
    white_a_tower_has_moved: bool,
    white_h_tower_has_moved: bool,
    squares_with_pined_pieces : CoordsVec,
    squares_with_pining_pieces: CoordsVec   
    // TODO: Add history features : black_king_has_move, black_Hrook_has_moved...
}

impl Board { // Initiators and init helpers

    pub fn new() -> Board { // Initiator
        Board {
            map: BoardMap::with_capacity(64),
            last_move: None,
            black_king_has_moved:    false,
            black_a_tower_has_moved: false,
            black_h_tower_has_moved: false,
            white_king_has_moved:    false,
            white_a_tower_has_moved: false,
            white_h_tower_has_moved: false,
            squares_with_pined_pieces  : CoordsVec::with_capacity(12),
            squares_with_pining_pieces : CoordsVec::with_capacity(12),
        }
    }

    pub fn from_boardmap(piece_by_coords:BoardMap) -> Board{ // Initiator
        let mut board = Board::new();
        board.map = piece_by_coords;
        board
    }

    pub fn at_start_state() -> Board { // Initiator
        Board::from_boardmap(Board::make_start_state())
    }

    pub fn make_start_state() -> BoardMap { // init Helper

        let mut piece_at_coords= BoardMap::with_capacity(64);
        for col in Column::iter(){
            
            let major_piece_kind:PieceKind = match col {
                A | H => Tower,
                B | G => Knight,
                C | F => Bishop,
                D => Queen,
                E => King,
            };

            piece_at_coords.insert(coords!(col, R8), piece!(Black, major_piece_kind)); // Black major pieces
            piece_at_coords.insert(coords!(col, R7), piece!(Black, Pawn)); // Black pawns
            piece_at_coords.insert(coords!(col, R2), piece!(White, Pawn)); // White pawns
            piece_at_coords.insert(coords!(col, R1), piece!(White, major_piece_kind)); // White major pieces

        };
        piece_at_coords
    }
}

impl Board { // Requesters
    pub fn piece_at_coords(&self, coords: Coords) -> Option<Piece> {
        self.map.get(&coords).copied()
    }

    pub fn color_of_piece_at(&self, square: &Coords) -> Option<Color> {
        if let Some(piece) = self.map.get(square) {
            Some(piece.color)
        } else {
            None
        }     
    }

    pub fn piece_checks_king(&self, piece_coords: &Coords) -> bool {
        // TODO
        return false
    }
}

impl Board { // Editors

    fn extract_piece_of_square(&mut self, square:&Coords) -> Piece {
        // Remove a Piece from a square and return it.
        self.extract_optionnal_piece_of_square(square).expect("Tried to extract Piece from an emtpy square in boardmap.")
    }

    fn extract_optionnal_piece_of_square(&mut self, square:&Coords) -> Option<Piece> {
        // Remove an Option<Piece> from a square and return it. It will be None if no Piece was on the square.
        self.map.remove(square)
    }

    fn add_piece_at_coords(&mut self,  coords: &Coords, piece: Piece) {
        self.map.insert(*coords, piece);
    }

    pub fn move_piece(&mut self, start_square: &Coords, target_square: &Coords) -> Coup {
    /*
    1: Extract the Piece at start_square (it is displaced: Piece),
    2: Extract the possible Piece at target_square (it is taken : Option<Piece>),
    3: Place the Piece displaced from start_square at target_square,
    4: Return the coup. Shall be used to update board.
    */
        let displaced: Piece = self.extract_piece_of_square(start_square);
        let taken: Option<Piece> = self.extract_optionnal_piece_of_square(target_square);
        self.add_piece_at_coords(target_square, displaced);
        let coup = Coup {
            start: *start_square,
            end  : *target_square,
            piece: displaced,
            taken: taken,
            checks: self.piece_checks_king(target_square)
        };
        coup
    }

}


impl Board {
    pub fn terminal_display(&self) {

        let mut board_str = String::from(".  .  .  .  .  .  .  .  .\n");
        for row in Row::iter().rev(){
            for col in Column::iter(){

                let case_color:Color = coords!(col, row).get_color();
                let piece_char = match &self.piece_at_coords(coords!(col, row)) {
                    Some(piece_at_pos) => piece_at_pos.as_char(),
                    None => ' '
                };
                let piece_str = String::from(piece_char);
                let piece_on_case_str: ColoredString = match case_color {
                    White => piece_str.on_color("white"),
                    Black => piece_str.on_color("grey")
                };
                board_str.push_str(".");
                board_str.push_str(&piece_on_case_str);
                board_str.push_str(" ");
            }
            board_str.push_str(".\n");
        }
        println!("{}",board_str);
    }
}