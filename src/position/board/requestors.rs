use crate::position::coordinates::types_and_structs::{Square, SquareVec, Coords, CoordsVec};
use crate::position::coordinates::converters::SquareVecEquivalent;
use crate::position::color::Color;
use crate::position::pieces::Piece;
use crate::position::board::types_and_structs::Board;

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

}