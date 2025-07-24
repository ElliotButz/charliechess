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

pub type BoardMap = HashMap<Square,Piece>;
#[allow(dead_code)]
pub struct Board {
    pub map: BoardMap,
    pub last_move: Coup,
    black_king_can_h_rook: bool,
    black_king_can_a_rook: bool,
    white_king_can_h_rook: bool,
    white_king_can_a_rook: bool,
    pub squares_with_pined_pieces : SquareVec,
    pub squares_with_pining_pieces: SquareVec   
}

impl Board { // Initiators and init helpers

    pub fn new() -> Board { // Initiator
        Board {
            map: BoardMap::with_capacity(64),
            last_move: Coup::coup_zero(),
            black_king_can_h_rook: false,
            black_king_can_a_rook: false,
            white_king_can_h_rook: false,
            white_king_can_a_rook: false,
            squares_with_pined_pieces  : SquareVec::with_capacity(8),
            squares_with_pining_pieces : SquareVec::with_capacity(8),
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

            piece_at_coords.insert(square!((col, R8)), piece!(Black, major_piece_kind)); // Black major pieces
            piece_at_coords.insert(square!((col, R7)), piece!(Black, Pawn)); // Black pawns
            piece_at_coords.insert(square!((col, R2)), piece!(White, Pawn)); // White pawns
            piece_at_coords.insert(square!((col, R1)), piece!(White, major_piece_kind)); // White major pieces

        };
        piece_at_coords
    }
}

impl Board { // Requesters
    
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.map.get(&square).copied()
    }

    pub fn color_of_piece_at(&self, square: Square) -> Option<Color> {
        if let Some(piece) = self.map.get(&square) {
            Some(piece.color)
        } else {
            None
        }     
    }

    pub fn targetables_and_stared_pieces(&self, squares: SquareVec, target_color: Color ) -> (SquareVec, Vec<Piece>) {

        let mut targetables = SquareVec::new();
        let mut stared:Vec<Piece> = Vec::new();

        for square in squares.iter() {
            let mut can_go = true;
            match self.piece_at(*square) {
                None => (),
                Some(piece) => {
                    if piece.color != target_color { can_go = false }
                    stared.push(piece);
                } 
            }
            // println!("{}", can_go);
            if can_go {targetables.push(*square)}
        }
    (targetables, stared)
    }

    pub fn square_is_free(&self, square: Square) -> bool {
        match self.piece_at(square) {
            None => true,
            _ => false
        }
    }

    pub fn square_is_free_for_piece_of_color(&self, square: Square, color: Color) -> bool {
        match self.color_of_piece_at(square) { // Exclude coords of ally pieces
            None => true,
            Some(piece_color) => ! (piece_color == color)
        }
    }

    pub fn piece_checks_king(&self, piece_coords: Square) -> bool {
        let _piece = self.piece_at(piece_coords).unwrap();
        // TODO
        return false
    }

    pub fn step_til_piece(&self, start: Square, step: Coords) -> (SquareVec, Option<Piece>) {
        // Makes a vector [start + n * step, with n in 1..=N, with N the first n for which start + n * step contains a piece ]
        // Returns the Option<found_piece> too.
        let cstart: Coords = start.into();
        let mut found_piece: Option<Piece> = None;
        let mut in_path = CoordsVec::with_capacity(8);
        let mut n_steps: i8 = 1;  
        loop {
            let target: Coords = cstart + step * n_steps ;
            if target.not_in_board() {break}
            else {
                in_path.push(target);
                match self.piece_at(target.into()) {
                    Some(piece) => {found_piece = Some(piece); break },
                    None => ()
                }
             }
            n_steps += 1;
        }
        return (in_path.to_square_vec(), found_piece) 
    }

    pub fn step_through_piece(&self, start: Square, step: Coords) -> (SquareVec, Vec<Piece>) {
        // Makes a vector [start + n * step | n in 1..=N, N being the last n for which start + n * is in board]
        // Returns a vector of found pieces too.
        let cstart: Coords = start.into();
        let mut found_pieces:Vec<Piece> = Vec::with_capacity(7);
        let mut in_path = CoordsVec::with_capacity(7);
        let mut n_steps: i8 = 1;  
        loop {
            let target: Coords = cstart + step * n_steps ;
            if target.not_in_board() {break}
            else {
                in_path.push(target);
                match self.piece_at(target.into()) {
                    Some(piece) => {found_pieces.push(piece)},
                    None => ()
                }
             }
            n_steps += 1;
        }
        return (in_path.to_square_vec(), found_pieces) 
    }

    pub fn step_in_directions_til_piece(&self, start: Square, directions: Vec<(i8, i8)>) -> (SquareVec, Vec<Piece>) {

        let mut in_all_paths = SquareVec::with_capacity(8);
        let mut found_pieces = Vec::with_capacity(directions.len());

        for &direction in directions.iter() {
            let (mut in_path, found_piece) = self.step_til_piece(start, direction.into());
            in_all_paths.append(&mut in_path);
            match found_piece {
                Some(piece) => found_pieces.push(piece),
                None => ()
            }
        }
        (in_all_paths, found_pieces)
    }


    pub fn step_til_target(&self, start: Square, step: Coords, target_color: Color) -> (SquareVec, Option<Piece>) {
        // Makes a vector [start + n * step, with n in 1..=N, with N the first n for which start + n * step contains a piece ].
        // If the found piece is not of the target color, the last element of the vector is excluded.
        // Returns the Option<found_piece> too, so that one know which piece blocks the path of another one.
        let (mut in_path, found_piece) = self.step_til_piece(start, step);
        if in_path.len() == 0 { return (in_path, found_piece)}
        match found_piece {
            Some(piece) => {
                if piece.color != target_color {
                    in_path.pop();
                }
            }
            None => ()
        }
        // println!("blip");
        return (in_path, found_piece)
    }

    pub fn step_in_directions_til_target(&self, start: Square, directions: Vec<(i8, i8)>, target_color: Color) -> (SquareVec, Vec<Piece>) {

        let mut in_all_paths = SquareVec::new();
        let mut found_pieces = Vec::with_capacity(directions.len());

        for &direction in directions.iter() {
            let (mut in_path, found_piece) = self.step_til_target(start, direction.into(), target_color);
            in_all_paths.append(&mut in_path);
            match found_piece {
                Some(piece) => found_pieces.push(piece),
                None => ()
            }
        }
        (in_all_paths, found_pieces)
    }

    pub fn step_in_directions_trough_target(&self, start: Square, directions: Vec<(i8, i8)>, target_color: Color) -> (SquareVec, Vec<Piece>) {

        let mut in_all_paths = SquareVec::new();
        let mut found_pieces = Vec::with_capacity(directions.len()*5);

        for &direction in directions.iter() {
            let (mut in_path, found_piece) = self.step_til_target(start, direction.into(), target_color);
            in_all_paths.append(&mut in_path);
            match found_piece {
                Some(piece) => found_pieces.push(piece),
                None => ()
            }
        }
        (in_all_paths, found_pieces)
    }

}

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


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {

        let mut board_str = String::from(".  .  .  .  .  .  .  .  .\n");
        for row in Row::iter().rev(){
            for col in Column::iter(){

                let case_color:Color = square!((col, row)).get_color();
                let piece_char = match &self.piece_at(square!((col, row))) {
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
        write!(f, "{}", board_str)
    }

}