use std::i8;
use strum_macros::EnumIter;
use num_derive; 

#[derive(EnumIter, PartialEq, Eq, Hash, Clone, Copy, num_derive::FromPrimitive, num_derive::ToPrimitive, Debug)]
pub enum Row {
    R1=1,
    R2=2,
    R3=3,
    R4=4,
    R5=5,
    R6=6,
    R7=7,
    R8=8,
}

#[derive(EnumIter, PartialEq, Hash, Eq,  Clone, Copy,  num_derive::FromPrimitive, num_derive::ToPrimitive, Debug)]
pub enum Column{
    A=1,
    B=2,
    C=3,
    D=4,
    E=5,
    F=6,
    G=7,
    H=8,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Square { pub col: Column, pub row: Row,}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coords { pub col: i8, pub row:i8}

pub type SquareVec = Vec<Square>;
pub type CoordsVec = Vec<Coords>;