use std::fmt;
use colored::{ColoredString, Colorize};
use strum::IntoEnumIterator;

use crate::{square};
use crate::position::{
    board::types_and_structs::Board,
    color::Color, 
    coordinates::types_and_structs::{Column, Row}};

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {

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
                board_str.push_str(" ");
            }
            board_str.push_str(".\n");
        }
        write!(f, "{}", board_str)
    }
}