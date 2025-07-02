use strum_macros::EnumIter; // 0.17.1

#[derive(Eq, Hash, PartialEq)]
pub struct Coords {
    col: Column,
    row: Row,
}

#[derive(EnumIter, PartialEq)]
pub enum Row {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

#[derive(EnumIter, PartialEq)]
pub enum Column{
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}