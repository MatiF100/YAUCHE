use std::fmt;

const COLUMNS: usize = 8;
const ROWS: usize = 8;
const SIZE: usize = COLUMNS * ROWS; //Number of squares on a board
const INIT: Option<Piece> = None; //Default piece for any square

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Queen,
    Rook,
    Bishop,
    Knight,
    King,
}

#[derive(Debug)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Debug)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
}
impl Piece {
    pub fn new(name: &str, black: bool) -> Self {
        Self {
            piece_color: if black == true {
                PieceColor::Black
            } else {
                PieceColor::White
            },
            piece_type: match name {
                "king" => PieceType::King,
                "knight" => PieceType::Knight,
                "rook" => PieceType::Rook,
                "bishop" => PieceType::Bishop,
                "queen" => PieceType::Queen,
                _ => PieceType::Pawn,
            },
        }
    }

    pub fn get_symbol(piece: &Option<Piece>) -> char {
        match piece {
            Some(piece) => match piece.piece_color {
                PieceColor::White => match piece.piece_type {
                    PieceType::King => '\u{2654}',
                    PieceType::Queen => '\u{2655}',
                    PieceType::Rook => '\u{2656}',
                    PieceType::Bishop => '\u{2657}',
                    PieceType::Knight => '\u{2658}',
                    PieceType::Pawn => '\u{2659}',
                    _ => 'X',
                },
                PieceColor::Black => match piece.piece_type {
                    PieceType::King => '\u{265A}',
                    PieceType::Queen => '\u{265B}',
                    PieceType::Rook => '\u{265C}',
                    PieceType::Bishop => '\u{265D}',
                    PieceType::Knight => '\u{265E}',
                    PieceType::Pawn => '\u{265F}',
                    _ => 'X',
                },
            },
            None => ' ',
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            piece_color: PieceColor::White,
            piece_type: PieceType::Pawn,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    fields: [Option<Piece>; SIZE],
}

impl Default for Board {
    fn default() -> Self {
        let mut default_board: [Option<Piece>; SIZE] = [INIT; SIZE];
        for i in 0..64 {
            default_board[i] = match i {
                0 | 7 => Some(Piece::new("rook", false)),
                1 | 6 => Some(Piece::new("knight", false)),
                2 | 5 => Some(Piece::new("bishop", false)),
                3 => Some(Piece::new("queen", false)),
                4 => Some(Piece::new("king", false)),
                (8..=15) => Some(Piece::new("pawn", false)),
                //56 63
                //55 48
                56 | 63 => Some(Piece::new("rook", true)),
                57 | 62 => Some(Piece::new("knight", true)),
                58 | 61 => Some(Piece::new("bishop", true)),
                59 => Some(Piece::new("queen", true)),
                60 => Some(Piece::new("king", true)),
                (48..=55) => Some(Piece::new("pawn", true)),
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
        for row in (0..ROWS).rev(){
            for column in 0..COLUMNS{
                let piece_idx = (row * 8) + column;
                write!(f, "{} ", Piece::get_symbol(&self.fields[piece_idx]))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
