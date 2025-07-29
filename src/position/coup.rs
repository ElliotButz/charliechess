use std::error::Error;
use std::fmt;
use regex::Regex;

use crate::position::board::types_and_structs::Board;
use crate::position::coordinates::types_and_structs::{Row, Column, Square};
use crate::position::pieces::{Piece,PieceKind};
use crate::position::color::Color;
use crate::square;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoupKind { Normal, Castle, Promotion(PieceKind) }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coup {
    pub start: Square,
    pub end  : Square,
    pub piece: Piece,
    pub taken: Option<Piece>,
    pub kind : CoupKind
    // pub checks: bool
}

impl Coup {

    pub fn is_pawn_double_step(&self) -> bool {
        self.piece.kind == PieceKind::Pawn &&
        (self.start.row as i8 - self.end.row as i8).abs() == 2
    }

    pub fn coup_zero() -> Self {
        Self {
            start: square!((Column::C, Row::R3)),
            end:   square!((Column::B, Row::R1)),
            piece: Piece {color: Color::White, kind: PieceKind::Knight},
            taken: None,
            kind : CoupKind::Normal
            // checks: false
        }
    }

    pub fn white_h_castle() -> Self {
        Self {
            start: square!((Column::E, Row::R1)),
            end: square!((Column::G, Row::R1)),
            piece: Piece {
                color: Color::White,
                kind: PieceKind::King,
            },
            taken: None,
            kind: CoupKind::Castle,
        }
    }

    pub fn white_a_castle() -> Self {
        Self {
            start: square!((Column::E, Row::R1)),
            end: square!((Column::C, Row::R1)),
            piece: Piece {
                color: Color::White,
                kind: PieceKind::King,
            },
            taken: None,
            kind: CoupKind::Castle,
        }
    }

    pub fn black_h_castle() -> Self {
        Self {
            start: square!((Column::E, Row::R8)),
            end: square!((Column::G, Row::R8)),
            piece: Piece {
                color: Color::Black,
                kind: PieceKind::King,
            },
            taken: None,
            kind: CoupKind::Castle,
        }
    }

    pub fn black_a_castle() -> Self {
        Self {
            start: square!((Column::E, Row::R8)),
            end: square!((Column::C, Row::R8)),
            piece: Piece {
                color: Color::Black,
                kind: PieceKind::King,
            },
            taken: None,
            kind: CoupKind::Castle,
        }
    }
}

impl fmt::Display for Coup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[derive(Debug)]
pub enum CoupError { EmptyStartSquare, IllegalCoup }
impl fmt::Display for CoupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyStartSquare => write!(f, "Start square is empty."),
            Self::IllegalCoup => write!(f, "Not among legal moves.")
        }
    }
}
impl Error for CoupError {} // This makes CoupError an Error so you can use it as an Error.

impl TryFrom<(Square, Square, bool, &mut Board)> for Coup {
    type Error = CoupError;
    
    fn try_from(start_end_castle_board: (Square, Square, bool, &mut Board)) -> Result<Self, Self::Error> {

        let (start, end, castle, board) = start_end_castle_board;
        // Piece exists ?
        let Some(moved_piece) = board.opt_piece_at(start) else { return Err(CoupError::EmptyStartSquare) } ;
        
        let coup = Coup {
            piece: moved_piece,
            start: start,
            end: end,
            taken: board.opt_piece_at(end),
            kind: match castle { true => CoupKind::Castle, false => CoupKind::Normal }
        }; 

        if ! board.is_legal(coup) { return Err(CoupError::IllegalCoup) }

        Ok(coup)
    }
}



#[derive(Debug)]
pub enum StrCoupError { UnrecognizedInput, CoupError(CoupError) }

impl fmt::Display for StrCoupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnrecognizedInput => write!(f, "Unrecognized input. Input should be formed with '<start square>-<target square>' (eg E2E4) or O-O (short castle) or O--O (long castle)"),
            Self::CoupError(err) => write!(f, "Can not make Coup from start and target square: {}",err)
        }
    }
}
impl Error for StrCoupError {}

impl TryFrom<(&str, &mut Board, Color)> for Coup {
    type Error = StrCoupError;

    fn try_from(str_board_player: (&str, &mut Board, Color)) -> Result<Self, StrCoupError> {
        let (move_str, board, player) = str_board_player;

        match move_str {

            "O-O" =>  match player {
                Color::Black => Ok(Coup::black_h_castle()),
                Color::White => Ok(Coup::white_h_castle())
            },

            "O--O" => match player {
                Color::Black => Ok(Coup::black_a_castle()),
                Color::White => Ok(Coup::white_a_castle())
            },

            _other => {

                let two_squares = Regex::new(r"[\wa-h][1-8]-[\wa-h][1-8]").unwrap();
                if !two_squares.is_match(&move_str) { return Err(StrCoupError::UnrecognizedInput) } 

                let start = Square::try_from(&move_str[0..2]).map_err(|_| StrCoupError::UnrecognizedInput)?;
                let end = Square::try_from(&move_str[3..5]).map_err(|_| StrCoupError::UnrecognizedInput)?;

                Coup::try_from((start, end, false, board)).map_err(|err| StrCoupError::CoupError(err))
            }
        }
    }
}