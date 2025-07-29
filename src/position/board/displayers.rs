use std::fmt;
use colored::{ColoredString, Colorize};
use strum::IntoEnumIterator;

use crate::position::coordinates::types_and_structs::Square;
use crate::{square};
use crate::position::{
    board::types_and_structs::Board,
    color::Color, 
    coordinates::types_and_structs::{Column, Row}};

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {

        let last =  self.last_move;
        let startend = vec![last.start, last.end];

        let mut board_str = String::from(".  .  .  .  .  .  .  .  .\n");
        for row in Row::iter().rev(){
            for col in Column::iter(){

                let case_color:Color = square!((col, row)).get_color();
                let piece_char = match &self.opt_piece_at(square!((col, row))) {
                    Some(piece_at_pos) => piece_at_pos.as_char(),
                    None => ' '
                };
                let piece_str = String::from(piece_char);
                let piece_on_case_str: ColoredString = match case_color {
                    Color::White => piece_str.on_color("white"),
                    Color::Black => piece_str.on_color("grey")
                };
                board_str.push_str(".");
                board_str.push_str(&piece_on_case_str);
                if startend.contains(&Square {col, row } ) {board_str.push_str("_");} else {board_str.push_str(" ");}
            }
            board_str.push_str(".\n");
        }
        write!(f, "{}", board_str)
    }
}

impl Board {
    pub fn to_fen_map(&self) -> String {
        let mut fen = String::new();
        let mut empty_squares = 0;

        for row in Row::iter().rev() {
            for col in Column::iter() {
                let square = square!((col, row));
                match self.opt_piece_at(square) {
                    Some(piece) => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        let piece_char = piece.as_char();
                        fen.push(piece_char);
                    }
                    None => empty_squares += 1,
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
                empty_squares = 0;
            }
            if row != Row::R8 {
                fen.push('/');
            }
        }
        fen
    }
    
}
