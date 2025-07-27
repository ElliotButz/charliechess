use std::error::Error;
use std::fmt;

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

// str > Column,  Row

#[derive(Debug)]
pub enum CharColRowError { UnrecognizedInput(char) }

impl fmt::Display for CharColRowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnrecognizedInput(c) => write!(f, "Failed to make column or row from input {c}. input should be in [1-8] or [aA-hH]")
        }
    }
}
impl Error for CharColRowError {}

impl TryFrom <char> for Column {
    type Error = CharColRowError;
    fn try_from(col_char: char) -> Result<Self, Self::Error> {
        match col_char.to_ascii_uppercase() {
            'A' => Ok(Column::A),
            'B' => Ok(Column::B),
            'C' => Ok(Column::C),
            'D' => Ok(Column::D),
            'E' => Ok(Column::E),
            'F' => Ok(Column::F),
            'G' => Ok(Column::G),
            'H' => Ok(Column::H),
            other => Err(CharColRowError::UnrecognizedInput(other))
        }
    }
}

impl TryFrom <char> for Row {
    type Error = CharColRowError;
    fn try_from(col_char: char) -> Result<Self, Self::Error> {
        match col_char.to_ascii_uppercase() {
            '1' => Ok(Row::R1),
            '2' => Ok(Row::R2),
            '3' => Ok(Row::R3),
            '4' => Ok(Row::R4),
            '5' => Ok(Row::R5),
            '6' => Ok(Row::R6),
            '7' => Ok(Row::R7),
            '8' => Ok(Row::R8),
            other => Err(CharColRowError::UnrecognizedInput(other))
        }
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

#[derive(Debug)]
pub enum StrSquareError { NoColumn, NoRow, CharColRowError(CharColRowError)}

impl From<CharColRowError> for StrSquareError {
    fn from(err: CharColRowError) -> Self {
        StrSquareError::CharColRowError(err)
    }
}

impl fmt::Display for StrSquareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoColumn => write!(f, "Empty input"),
            Self::NoRow => write!(f, "Input should be 2-sized."),
            Self::CharColRowError(err) => write!(f, "Failed to read input Row or Column: {}",err),
        }
    }
}

impl TryFrom <&str> for Square {
    type Error = StrSquareError;
    fn try_from(str_square: &str) -> Result< Square, Self::Error> {
        let mut chars_square = str_square.chars();
        let input_col_char = chars_square.next().ok_or( StrSquareError::NoColumn)?;
        let input_row_char = chars_square.next().ok_or( StrSquareError::NoRow)?;
        let input_col = Column::try_from(input_col_char)?; 
        let input_row = Row::try_from(input_row_char)?; 
        Ok(Square {
            col: input_col,
            row: input_row
        })
    }
}