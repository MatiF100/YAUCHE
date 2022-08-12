use std::fmt;

pub const COLUMNS: usize = 10;
pub const ROWS: usize = 12;
pub const SIZE: usize = COLUMNS * ROWS; //Number of squares on a board
const INIT: Option<Piece> = None; //Default piece for any square

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn(bool),
    Queen,
    Rook(bool),
    Bishop,
    Knight,
    King(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}
impl Piece {
    pub fn new(name: &str, color: PieceColor) -> Self {
        Self {
            piece_color: color,
            piece_type: match name {
                "king" => PieceType::King(false),
                "knight" => PieceType::Knight,
                "rook" => PieceType::Rook(false),
                "bishop" => PieceType::Bishop,
                "queen" => PieceType::Queen,
                _ => PieceType::Pawn(false),
            },
        }
    }

    //    pub fn get_moves(&self, board: &Board) -> Vec<

    pub fn get_symbol(&self) -> char {
        match self.piece_color {
            PieceColor::Black => match self.piece_type {
                PieceType::King(_) => '\u{2654}',
                PieceType::Queen => '\u{2655}',
                PieceType::Rook(_) => '\u{2656}',
                PieceType::Bishop => '\u{2657}',
                PieceType::Knight => '\u{2658}',
                PieceType::Pawn(_) => '\u{2659}',
            },
            PieceColor::White => match self.piece_type {
                PieceType::King(_) => '\u{265A}',
                PieceType::Queen => '\u{265B}',
                PieceType::Rook(_) => '\u{265C}',
                PieceType::Bishop => '\u{265D}',
                PieceType::Knight => '\u{265E}',
                PieceType::Pawn(_) => '\u{265F}',
            },
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            piece_color: PieceColor::White,
            piece_type: PieceType::Pawn(false),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub fields: [Option<Piece>; SIZE],
}

impl Default for Board {
    fn default() -> Self {
        let mut default_board: [Option<Piece>; SIZE] = [INIT; SIZE];
        for i in 0..SIZE {
            default_board[i] = match i {
                21 | 28 => Some(Piece::new("rook", PieceColor::White)),
                22 | 27 => Some(Piece::new("knight", PieceColor::White)),
                23 | 26 => Some(Piece::new("bishop", PieceColor::White)),
                24 => Some(Piece::new("queen", PieceColor::White)),
                25 => Some(Piece::new("king", PieceColor::White)),
                (31..=38) => Some(Piece::new("pawn", PieceColor::White)),
                //56 63
                //55 48
                91 | 98 => Some(Piece::new("rook", PieceColor::Black)),
                92 | 97 => Some(Piece::new("knight", PieceColor::Black)),
                93 | 96 => Some(Piece::new("bishop", PieceColor::Black)),
                94 => Some(Piece::new("queen", PieceColor::Black)),
                95 => Some(Piece::new("king", PieceColor::Black)),
                (81..=88) => Some(Piece::new("pawn", PieceColor::Black)),
                _ => None,
            };
        }
        Self {
            fields: default_board,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for row in (0..ROWS).rev() {
            for column in 0..COLUMNS {
                let piece_idx = (row * 10) + column;
                write!(
                    f,
                    "{} ",
                    match &self.fields[piece_idx] {
                        Some(piece) => piece.get_symbol(),
                        None => ' ',
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
