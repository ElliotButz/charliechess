use crate::{
    Board,
    position::{color::Color, coordinates::types_and_structs::SquareVec}};
pub trait OpenToColor {
    fn open_to_color(&mut self, board: &Board, color: Color);
}
impl OpenToColor for SquareVec {
    fn open_to_color(&mut self, board: &Board, color: Color) {
        self.retain(|&square| board.square_is_free_for_piece_of_color(square, color))
    }
}