use std::fmt;

const SIZE: usize = 64; //Number of squares on a board
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
        write!(
            f,
            "
            {} {} {} {} {} {} {} {}\n
            {} {} {} {} {} {} {} {}\n
            {} {} {} {} {} {} {} {}\n
            {} {} {} {} {} {} {} {}\n
            {} {} {} {} {} {} {} {}\n
            {} {} {} {} {} {} {} {}\n
            {} {} {} {} {} {} {} {}\n
            {} {} {} {} {} {} {} {}
        ",
            Piece::get_symbol(&self.fields[56]),
            Piece::get_symbol(&self.fields[57]),
            Piece::get_symbol(&self.fields[58]),
            Piece::get_symbol(&self.fields[59]),
            Piece::get_symbol(&self.fields[60]),
            Piece::get_symbol(&self.fields[61]),
            Piece::get_symbol(&self.fields[62]),
            Piece::get_symbol(&self.fields[63]),
            Piece::get_symbol(&self.fields[48]),
            Piece::get_symbol(&self.fields[49]),
            Piece::get_symbol(&self.fields[50]),
            Piece::get_symbol(&self.fields[51]),
            Piece::get_symbol(&self.fields[52]),
            Piece::get_symbol(&self.fields[53]),
            Piece::get_symbol(&self.fields[54]),
            Piece::get_symbol(&self.fields[55]),
            Piece::get_symbol(&self.fields[40]),
            Piece::get_symbol(&self.fields[41]),
            Piece::get_symbol(&self.fields[42]),
            Piece::get_symbol(&self.fields[43]),
            Piece::get_symbol(&self.fields[44]),
            Piece::get_symbol(&self.fields[45]),
            Piece::get_symbol(&self.fields[46]),
            Piece::get_symbol(&self.fields[47]),
            Piece::get_symbol(&self.fields[32]),
            Piece::get_symbol(&self.fields[33]),
            Piece::get_symbol(&self.fields[34]),
            Piece::get_symbol(&self.fields[35]),
            Piece::get_symbol(&self.fields[36]),
            Piece::get_symbol(&self.fields[37]),
            Piece::get_symbol(&self.fields[38]),
            Piece::get_symbol(&self.fields[39]),
            Piece::get_symbol(&self.fields[24]),
            Piece::get_symbol(&self.fields[25]),
            Piece::get_symbol(&self.fields[26]),
            Piece::get_symbol(&self.fields[27]),
            Piece::get_symbol(&self.fields[28]),
            Piece::get_symbol(&self.fields[29]),
            Piece::get_symbol(&self.fields[30]),
            Piece::get_symbol(&self.fields[31]),
            Piece::get_symbol(&self.fields[16]),
            Piece::get_symbol(&self.fields[17]),
            Piece::get_symbol(&self.fields[18]),
            Piece::get_symbol(&self.fields[19]),
            Piece::get_symbol(&self.fields[20]),
            Piece::get_symbol(&self.fields[21]),
            Piece::get_symbol(&self.fields[22]),
            Piece::get_symbol(&self.fields[23]),
            Piece::get_symbol(&self.fields[8]),
            Piece::get_symbol(&self.fields[9]),
            Piece::get_symbol(&self.fields[10]),
            Piece::get_symbol(&self.fields[11]),
            Piece::get_symbol(&self.fields[12]),
            Piece::get_symbol(&self.fields[13]),
            Piece::get_symbol(&self.fields[14]),
            Piece::get_symbol(&self.fields[15]),
            Piece::get_symbol(&self.fields[0]),
            Piece::get_symbol(&self.fields[1]),
            Piece::get_symbol(&self.fields[2]),
            Piece::get_symbol(&self.fields[3]),
            Piece::get_symbol(&self.fields[4]),
            Piece::get_symbol(&self.fields[5]),
            Piece::get_symbol(&self.fields[6]),
            Piece::get_symbol(&self.fields[7])
        )
    }
}
