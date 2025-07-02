pub struct Coords {
    col: Column,
    row: Row,
}

#[derive(EnumIter)]
enum Row {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

#[derive(EnumIter)]
enum Column{
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}