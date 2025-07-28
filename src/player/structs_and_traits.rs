use crate::{position::{
    coup::Coup,
    Position}};

pub type Strategy = fn(for_pos: &mut Position) -> Coup;
pub struct Player {
    pub strategy: Strategy
}



