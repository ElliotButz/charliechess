use crate::{coords, piece};
use crate::position::color::{Color,Color::{White, Black}};
use crate::position::coordinates::{Coords, Row, Row::*, Column, Column::*};
use crate::position::pieces::{Piece, PieceKind, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};
use crate::position::board;
use crate::position::coup;

struct History {
    
}