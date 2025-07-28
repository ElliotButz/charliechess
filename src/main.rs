extern crate num_derive;

pub mod position;

use crate::position::color::Color;
use crate::position::coordinates::displayers::vec2str;
//use crate::position::coordinates::types_and_structs::{Row::*, Column::*};
use crate::position::board::types_and_structs::Board;
use crate::position::coup::Coup;
fn main() {

    let mut board = Board::at_start_state();

    println!("{board}");

    let moves = board.all_moves();
    println!("Legal moves: {}", vec2str(&moves));

    match Coup::try_from(("A2-A3", &mut board, Color::Black)) {
        Ok(coup) => println!("{coup}"),
        Err(statement) => println!("{statement}")
    }

    println!("{}", board.to_fen_map())
}