use   crate::position::coordinates::types_and_structs::CoordsVec;

#[macro_export]
macro_rules! col {
    ($input:expr) => { crate::position::coordinates::types_and_structs::Column::from($input) }       
}

#[macro_export]
macro_rules! row {
    ($input:expr) => { crate::position::coordinates::types_and_structs::Row::from($input) }       
}

#[macro_export]
macro_rules! square {
    ($input:expr) => { crate::position::coordinates::types_and_structs::Square::from($input) }       
}

#[macro_export]
macro_rules! coords {
    ($input:expr) => { crate::position::coordinates::types_and_structs::Coords::from($input) } 
}

pub fn from_checked_i8 <T: num_traits::FromPrimitive>(value: i8) -> T {
    num_traits::FromPrimitive::from_i8(value)
        .expect(&format!("failed to make {} out of {}", std::any::type_name::<T>(), value))
}

pub trait ExcludeOutOfBoard {
    fn exclude_out_of_board(&mut self) ;
}

impl ExcludeOutOfBoard for CoordsVec{
    fn exclude_out_of_board(&mut self) {
        self.retain(|&coords| coords.in_board())
    }
}