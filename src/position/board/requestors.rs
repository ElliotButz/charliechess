use ordered_hash_map::OrderedHashMap;

use crate::position::coordinates::types_and_structs::{Coords, CoordsVec, Row, Square, SquareVec};
use crate::position::coordinates::converters::{to_square_vec};
use crate::position::color::Color;
use crate::position::coup::{Coup, CoupKind};
use crate::position::pieces::{Piece, PieceKind::{Bishop, King, Queen, Tower, Pawn, Knight}};
use crate::position::board::types_and_structs::Board;
use crate::position::basic_piece_moves::{basic_moves_for_piece_at_square};

impl Board { // Requesters


    pub fn try_add_coup(&self, vec_coup: &mut Vec<Coup>, coup: Coup) {
        if !self.would_check(coup, self.player_to_play) {vec_coup.push(coup)}

    }
    
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

        for &square in squares.iter() {
            let mut can_go = true;
            match self.opt_piece_at(square) {
                None => (),
                Some(piece) => {
                    if piece.color != target_color { can_go = false }
                    stared.push(piece);
                } 
            }
            // println!("{}", can_go);
            if can_go {targetables.push(square)}
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

        let mut in_all_paths = SquareVec::with_capacity(13);
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
        self.all_watched_by(opponent_color).contains(&square)
    }

    pub fn all_watched_by(&self, player: Color) -> SquareVec {
        let mut all_watched = Vec::with_capacity(64);

        for (_watcher, watched_squares) in match player {
            Color::Black => &self.black_watchers_and_watched,
            Color::White => &self.white_watchers_and_watched
        } { 
            all_watched.extend(watched_squares);
        }
        all_watched
    }

    pub fn is_legal(&mut self, coup: Coup) -> bool {
        let resultant_board = self.simulate_coup(coup);

        resultant_board.possible_moves.contains(&coup)
    }


    pub fn simulate_coup(&self, coup: Coup) -> Self {
        let mut simulated = self.clone();
        simulated.execute(coup);
        simulated
    }

    fn would_check(&self, coup: Coup, player_color: Color) -> bool {
        let sim = self.simulate_coup(coup);
        sim.is_checked(player_color)
    }

    pub fn is_checked(&self, player_color: Color) -> bool {
        let king_square: Square = self.squares_with(crate::piece!(player_color, King))[0];
        self.all_watched_by(player_color.the_other()).contains(&king_square)
    }

}