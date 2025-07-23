use std::fmt;

use crate::position::coordinates::types_and_structs::{Column, Row, Square, CoordsVec, SquareVec};

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
            Self::H => write!(f, "H"),
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_value = *self as i8;
        write!(f, "{}", row_value)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.col, self.row)
    }
}

pub fn vec2str<T: std::fmt::Display>(vector: &Vec<T>) -> String {
    if vector.is_empty() {
        return "[]".to_string();
    }
    
    let mut out_str = String::from("[");
    for (i, item) in vector.iter().enumerate() {
        if i > 0 {
            out_str.push_str(", ");
        }
        out_str.push_str(&format!("{item}"));
    }
    out_str.push(']');
    out_str       
}