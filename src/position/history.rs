use std::collections::HashMap;

use crate::position::{
    board::types_and_structs::{Board}, coup::{Coup}
};


#[derive(Clone, Copy)]
pub struct Turn {
    pub white_coup: Coup,
    pub black_coup: Option<Coup> // None if game ends after white's move.
} 

#[derive(Clone)]
pub struct History {
    turns: Vec<Turn>
}

impl History {
    
    pub fn new() -> History {
        History{turns: Vec::new()}
    }

    pub fn add_coup(&mut self, coup: Coup) {

        let mut need_new_turn = false;
        // If there is a turn in turns...
        if let Some(last_turn) = self.turns.last_mut() {
            match last_turn.black_coup.is_some() {
                // if it has a black coup, we'll need to create a new turn;
                true => need_new_turn = true,
                 // if it has no black coup yet, the current coup should be added as a black coup;
                false => last_turn.black_coup = Some(coup),
            }
        } 
        // If there is no turn in turns, we need to add one.
        else { need_new_turn = true }

        if need_new_turn { self.turns.push( Turn {white_coup: coup, black_coup: None}) }
    }

    pub fn nth_turn(&self, n:usize) -> Option<Turn> {
        match 1 < n && n < self.turns.len() {
            false => None,
            true => Some(self.turns[n-1].clone()),
        }
    }

    pub fn all_boards(&self) -> Vec<Board> {

        let mut all_boards: Vec<Board> = Vec::new();
        let mut board_at_coup = Board::at_start_state();

        for turn in &self.turns {
            board_at_coup.execute(turn.white_coup);
            all_boards.push(board_at_coup.clone());

            if let Some(black_coup) = turn.black_coup {
                board_at_coup.execute(black_coup);
                all_boards.push(board_at_coup.clone());
            } else {
                break
            }
        }
        all_boards
    }

    pub fn show(&self) {
        for board in self.all_boards() {
            println!("{board}")
        }
    }

    pub fn max_times_board_occured(&self) -> usize {
        let all_boards:Vec<String> = self.all_boards().iter().map(|board| board.to_fen_map()).collect();
        let mut pos_counter: HashMap<String, usize> = HashMap::new();
        for map in all_boards {
            *pos_counter.entry(map).or_default() += 1;
        }
        match pos_counter.values().max() {
            Some(&n) => n,
            None => 0
        }    
    }

    pub fn n_turns(&self) -> usize {
        self.turns.len()
    }

}