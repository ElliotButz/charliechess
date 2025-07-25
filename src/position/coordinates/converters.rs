use crate::{col, row};
use crate::position::coordinates::types_and_structs::{Column, Row, Square, Coords, SquareVec, CoordsVec};
use crate::position::coordinates::initiators::from_checked_i8;


// i8 <> Row <> Column
impl From <i8> for Row {
    fn from(elt: i8) -> Self {
        from_checked_i8(elt)
    }
}

impl From <i8> for Column {
    fn from(elt: i8) -> Self {
        from_checked_i8(elt)
    }
}

impl From <Row> for i8 {
    fn from(r: Row) -> Self {
        r as i8
    }
}

impl From <Column> for i8 {
    fn from(c: Column) -> Self {
        c as i8
    }
}

// (i8, i8) <> Coords <> Square
impl From <Square> for Coords {
    fn from(square: Square) -> Self {
        Coords { col: square.col.into(), row: square.row.into() }
    }
}

impl From <Coords> for Square {
    fn from(coordinates: Coords) -> Self {
        assert!(coordinates.in_board(), "Tried to create Square from Coords that are out of board.");
        Square  {col: col!(coordinates.col), row: row!(coordinates.row) }
    }
}

impl From <(i8, i8)> for Coords {
    fn from(coord_couple: (i8, i8)) -> Self {
        Coords{ col: coord_couple.0, row: coord_couple.1 }
    }
}

impl From <(i8, i8)> for Square {
    fn from(coord_couple: (i8, i8)) -> Self {
        Square::from(Coords::from(coord_couple))
    }
}

impl From <Coords> for (i8, i8) {
    fn from(coords: Coords) -> Self {
        (coords.col as i8, coords.row as i8)
    }
}

impl From <Square> for (i8, i8) {
    fn from(square: Square) -> Self {
        Coords::from(square).into()
    }
}

impl From <(Column, Row)> for Square {
    fn from(cr: (Column, Row)) -> Self {
        Square { col: cr.0 , row: cr.1 }
    }
}

impl From <(Column, Row)> for Coords {
    fn from(cr: (Column, Row)) -> Self {
        Coords::from(Square::from(cr))
    }
}

pub fn to_coords_vec<T> (input_vec: &Vec<T>) -> CoordsVec where T: Into<Coords>, T: Copy {
    input_vec.iter().map(|&x| x.into()).collect()
}

pub fn to_square_vec<T> (input_vec: &Vec<T>) -> SquareVec where T: Into<Coords>, T: Copy {
    let mut as_coords_vec = to_coords_vec(input_vec);
    as_coords_vec.retain(|coord:&Coords| coord.in_board());
    as_coords_vec.iter().map(|&x| x.into()).collect()
}

pub fn col_as_square_vec (colidx: i8) -> SquareVec {
    (1..=8).map(|rowidx:i8|(colidx, rowidx).into()).collect()
}

pub fn row_as_square_vec (rowidx: i8) -> SquareVec {
    (1..=8).map(|colidx:i8|(colidx, rowidx).into()).collect()
}
