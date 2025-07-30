extern crate num_derive;
use std::env;

pub mod player;
pub mod game;
pub mod position;

use crate::game::Game;
use crate::player::structs_and_traits::Player;
use crate::position::board::types_and_structs::Board;
fn main() {

    // this method needs to be inside main() method
    unsafe {env::set_var("RUST_BACKTRACE", "1");}

    let p1 = Player::random();
    let p2 = Player::random();
    let mut game = Game::new(p1, p2 );
    use std::time::Instant;
    let now = Instant::now();
    let outcome = game.cli_play(100, false);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Result: {:?}\nFinal board:\n{} N turns:\n{}\ntime per turn: {}",
    outcome.state(), outcome.board, game.position.history.all_boards().len(), elapsed.as_secs_f64()/(game.position.history.all_boards().len() as f64))

}