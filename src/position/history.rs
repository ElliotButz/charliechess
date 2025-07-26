use crate::position::{
    coup::{Coup},
    color::Color::*
};


#[derive(Clone, Copy)]
pub struct Turn {
    pub white_coup: Coup,
    pub black_coup: Option<Coup> // None if game ends after white's move.
} 
pub struct History {
    turns: Vec<Turn>
}

#[allow(dead_code)]
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

    pub fn as_turn_serie(&self) -> Vec<Turn> {
        let mut turn_serie: Vec<Turn> = Vec::new();
        for n in (1..self.turns.len()).step_by(2) {
            let white_coup: Coup = self.nth_turn(n).expect("All white coups in history are expected to be of type Coup, found non-Coup.");
            let black_coup: Option<Coup> = self.nth_turn(n+1);
            let turn = Turn{white_coup, black_coup}; // In a turn, White have played for sure but not Black.
            turn_serie.push(turn)
        }
        turn_serie 
    }

}