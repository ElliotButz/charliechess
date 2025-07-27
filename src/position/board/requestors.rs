use ordered_hash_map::OrderedHashMap;

use crate::position::coordinates::types_and_structs::{Square, SquareVec, Coords, CoordsVec};
use crate::position::coordinates::converters::{to_square_vec};
use crate::position::color::Color;
use crate::position::coup::{Coup, CoupKind};
use crate::position::pieces::{Piece, PieceKind::{Bishop, King, Queen, Tower, Pawn, Knight}};
use crate::position::board::types_and_structs::Board;
use crate::position::basic_piece_moves::{basic_moves_for_piece_at_square, basic_moves_for_piece_from_square};

impl Board { // Requesters

    pub fn all_moves(&mut self) -> (Vec<Coup>,Vec<Coup>) {
        // Returns all possible moves (aka coups) for white and black player.

        self.update_info();

        let mut white_moves: Vec<Coup> = Vec::new();
        let mut black_moves: Vec<Coup> = Vec::new(); 

        for (&square, &_piece) in self.map.iter().filter(
            |(square, _piece)| !self.squares_with_pined_pieces.contains(square) 
        ) { 
            let (targetable_squares, _pieces_in_sight ) = basic_moves_for_piece_at_square(self, square);
            for &target_square in targetable_squares.iter() {
                let mover_piece = self.piece_at(square);
                let coup = Coup {
                    start: square,
                    end: target_square,
                    piece: mover_piece,
                    taken: self.opt_piece_at(target_square),
                    kind: CoupKind::Normal
                };
                match &mover_piece.color {
                    Color::White => white_moves.push(coup),
                    Color::Black => black_moves.push(coup),
                };
            }
        };

        if self.black_can_h_castle { black_moves.push(Coup::black_h_castle()) };
        if self.black_can_a_castle { black_moves.push(Coup::black_a_castle()) };
        if self.white_can_h_castle { white_moves.push(Coup::white_h_castle()) };
        if self.white_can_a_castle { white_moves.push(Coup::white_a_castle()) };



        (white_moves, black_moves)}
    
    pub fn opt_piece_at(&self, square: Square) -> Option<Piece> {
        self.map.get(&square).copied()
    }

    pub fn piece_at(&self, square: Square) -> Piece {
        self.map.get(&square).copied().expect("Expected piece at {square}, found none.")
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
            match self.opt_piece_at(*square) {
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
        match self.opt_piece_at(square) {
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
        let _piece = self.opt_piece_at(piece_coords).unwrap();
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
                match self.opt_piece_at(target.into()) {
                    Some(piece) => {found_piece = Some(piece); break },
                    None => ()
                }
             }
            n_steps += 1;
        }
        return (to_square_vec(&in_path), found_piece) 
    }

    pub fn step_through_piece(&self, start: Square, step: Coords) -> OrderedHashMap<Square, Piece> {
        // Makes a vector [start + n * step | n in 1..=N, N being the last n for which start + n * is in board]
        // Returns a vector of found pieces too.
        let cstart: Coords = start.into();
        let mut squares_and_pieces = OrderedHashMap::new();
        let mut n_steps: i8 = 1;  
        loop {
            let target: Coords = cstart + step * n_steps ;
            if target.not_in_board() {break}
            else {
                let target_square = target.into();
                match self.opt_piece_at(target_square) {
                    Some(piece) => { squares_and_pieces.insert(target_square, piece); },
                    None => ()
                }
             }
            n_steps += 1;
        }
        return squares_and_pieces
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

    pub fn squares_with(&self, piece: Piece) -> Vec<Square> {
        let mut squares = Vec::new(); 
        for (&square, &at_square) in self.map.iter() {
            if at_square == piece { squares.push(square); }
        }
        squares
    }

    pub fn square_is_in_sight_of_opponent(&self, square: Square, opponent_color: Color) -> bool {
        // Check if square is targetable by opponent piece. Checks for Pawn in a second time.

        // Check for all pieces, expect Pawn
        let c_square: Coords = square.into();

        [Queen, Tower, Bishop, Knight].iter().any(|&kind|
        {
            let piece = Piece { kind: kind, color: opponent_color}; 
            let (_sighters_possible_squares, sighters) = basic_moves_for_piece_from_square(self, square, piece);
            sighters.contains(&piece)
        } )  
        ||
        { // Check for pawn
            let direction_in_which_is_sighter = opponent_color.the_other().as_direction();
            to_square_vec(&vec![
                c_square + ( 1, direction_in_which_is_sighter),
                c_square + (-1, direction_in_which_is_sighter)
                ]).iter().any(|&potential_pawn_sighter_square|
                {
                    match self.opt_piece_at(potential_pawn_sighter_square) {
                        Some(piece) => {
                            piece == Piece {kind: Pawn, color: opponent_color}},
                        None => false
                    }
                } )
        }
        || { // Check for King, but whithout using bascic_moves_for_piece_from_square to avoid recursive call.
            let king_coords: Coords = self.squares_with(crate::piece!(opponent_color, King))[0].into();
            let steps: Vec<(i8, i8)> = vec![(-1,0), (1,0), (0,-1), (0,1), (-1,-1), (1,1), (1,-1), (-1,1)];
            steps.iter().any(|&step| {
                let target: Coords = king_coords + step;
                target == c_square && self.square_is_free(target.into())
            })
        }

    }


}