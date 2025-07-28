extern crate num_derive;

pub mod player;
pub mod game;
pub mod position;

use crate::game::Game;
use crate::player::structs_and_traits::Player;
use crate::position::board::types_and_structs::Board;
fn main() {


    let p1 = Player::random();
    let p2 = Player::random();
    let mut game = Game::new(p1, p2 );
    let outcome = game.cli_play(999, true);
    println!("{:?}\n{}",outcome.state(), outcome.board)
/*     let mut board = Board::at_start_state();

    println!("{board}");

    let moves = board.all_moves();
    println!("Legal moves: {}", vec2str(&moves));

    match Coup::try_from(("A2-A3", &mut board, Color::Black)) {
        Ok(coup) => println!("{coup}"),
        Err(statement) => println!("{statement}")
    } */
}