use crate::board::{Board, Piece, PieceColor, PieceType};

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub source: usize,
    pub target: usize,
    pub captured: Option<Piece>,
    pub promotion: Option<Piece>,
    pub ep: Option<usize>,             //Position of captured pawn
    pub castle: Option<(bool, Piece)>, //True for king side
}

impl Move {
    fn is_on_board(&self) -> bool {
        !(self.target % 10 == 0
            || (self.target + 1) % 10 == 0
            || self.target < 20
            || self.target > 100)
    }

    fn create_move(source: usize, target: usize, captured: Option<Piece>) -> Self {
        Self {
            source,
            target,
            captured,
            promotion: None,
            ep: None,
            castle: None,
        }
    }

    fn get_promoted_pawn(&self, color: PieceColor) -> [Move; 4] {
        let mut p1 = self.clone();
        p1.promotion = Some(Piece::new("Queen", color));
        let mut p2 = self.clone();
        p2.promotion = Some(Piece::new("Knight", color));
        let mut p3 = self.clone();
        p3.promotion = Some(Piece::new("Bishop", color));
        let mut p4 = self.clone();
        p4.promotion = Some(Piece::new("Rook", color));
        [p1, p2, p3, p4]
    }
}

impl Board {
    pub fn perft(&self, depth: usize, log: &Vec<Move>, color: &PieceColor) -> u128{
        if depth == 0{
            return 1;
        }
        let mut nodes = 0;

        let mut board = self.clone();
        let mut log_copy = log.clone();

        let moves = self.get_pv_moves(color, log);
        for pv_move in moves{
            board.make_move(&pv_move, &mut log_copy);
            if board.validate(){
                nodes += board.perft(depth - 1, &log_copy, if *color == PieceColor::Black {&PieceColor::White} else {&PieceColor::Black});
            }
            board.undo_move(&mut log_copy);
        }
        nodes

    }

    pub fn validate(&self) -> bool {
        for (position, king) in self.fields.iter().enumerate().filter(|p| match p {
            (_, Some(q)) => {
                q.piece_type == PieceType::King(true) || q.piece_type == PieceType::King(false)
            }
            _ => false,
        }) {
            let directions = [10, 1];
            let king = king.unwrap();

            // Rook check
            for step in directions {
                let mut d = step;
                while !((position + d) % 10 == 0
                    || (position + d + 1) % 10 == 0
                    || position + d < 20
                    || position + d > 100)
                {
                    let tmp = Move::create_move(position, position + d, self.fields[position + d]);
                    if let Some(piece) = tmp.captured {
                        if piece.piece_color != king.piece_color && {
                            piece.piece_type == PieceType::Queen
                                || piece.piece_type == PieceType::Rook(false)
                                || piece.piece_type == PieceType::Rook(true)
                        } {
                            //moves.push(tmp);
                            return false;
                        }
                        break;
                    }
                    d += step;
                }

                d = step;
                while !((position - d) % 10 == 0
                    || (position - d + 1) % 10 == 0
                    || position - d < 20
                    || position - d > 100)
                {
                    let tmp = Move::create_move(position, position - d, self.fields[position - d]);
                    if let Some(piece) = tmp.captured {
                        if piece.piece_color != king.piece_color && {
                            piece.piece_type == PieceType::Queen
                                || piece.piece_type == PieceType::Rook(false)
                                || piece.piece_type == PieceType::Rook(true)
                        } {
                            //moves.push(tmp);
                            return false;
                        }
                        break;
                    }
                    d += step;
                }
            }

            // Bishop check

            let directions = [11, 9];

            for step in directions {
                let mut d = step;
                while !((position + d) % 10 == 0
                    || (position + d + 1) % 10 == 0
                    || position + d < 20
                    || position + d > 100)
                {
                    let tmp = Move::create_move(position, position + d, self.fields[position + d]);
                    if let Some(piece) = tmp.captured {
                        if piece.piece_color != king.piece_color && {
                            piece.piece_type == PieceType::Queen
                                || piece.piece_type == PieceType::Bishop
                        } {
                            return false;
                        }
                        break;
                    }
                    d += step;
                }

                d = step;
                while !((position - d) % 10 == 0
                    || (position - d + 1) % 10 == 0
                    || position - d < 20
                    || position - d > 100)
                {
                    let tmp = Move::create_move(position, position - d, self.fields[position - d]);
                    if let Some(piece) = tmp.captured {
                        if piece.piece_color != king.piece_color && {
                            piece.piece_type == PieceType::Queen
                                || piece.piece_type == PieceType::Bishop
                        } {
                            return false;
                        }
                        break;
                    }
                    d += step;
                }
            }
            // Knight check
            let targets = [
                position + 19, //FL
                position + 21, //FR
                position + 12, //RF
                position - 8,  //RB
                position - 19, //BR
                position - 21, //BL
                position - 12, //LB
                position + 8,  //LF
            ];
            for target in targets {
                if let Some(on_target) = self.fields[target] {
                    //Attack
                    if on_target.piece_color != king.piece_color
                        && on_target.piece_type == PieceType::Knight
                    {
                        return false;
                    }
                }
            }

            // Oh god... Pawn check
                let step = if king.piece_color == PieceColor::White {
                    10
                } else {
                    -10
                };
                let target = (position as isize + 2 * step) as usize;

                //Attacks
                if let Some(piece) = self.fields[target + 1] {
                    if piece.piece_color != king.piece_color && (piece.piece_type == PieceType::Pawn(false) || piece.piece_type == PieceType::Pawn(true)) {
                        return false;
                    }
                }

                if let Some(piece) = self.fields[target - 1] {
                    if piece.piece_color != king.piece_color && (piece.piece_type == PieceType::Pawn(false) || piece.piece_type == PieceType::Pawn(true)) {
                        return false;
                    }
                }

        }
        true
    }

    pub fn get_v_moves(&self, color: &PieceColor, log: &Vec<Move>) -> Vec<Move> {
        let mut log_clone = log.clone();
        return self.get_pv_moves(color, log).into_iter().filter(|m| {
            let mut board = self.clone();
            board.make_move(&m, &mut log_clone);
            board.validate()

        } ).collect();
    }

    pub fn get_pv_moves(&self, color: &PieceColor, log: &Vec<Move>) -> Vec<Move> {
        let mut vec = Vec::new();
        vec.append(&mut self.get_pawn_moves(color, log));
        vec.append(&mut self.get_knight_moves(color));
        vec.append(&mut self.get_bishop_moves(color));
        vec.append(&mut self.get_rook_moves(color));
        vec.append(&mut self.get_king_moves(color));

        vec
    }
    fn get_king_moves(&self, color: &PieceColor) -> Vec<Move> {
        let kings = self
            .fields
            .iter()
            .enumerate()
            .filter(|p| match p {
                (_, Some(q)) => {
                    q.piece_color == *color
                        && (q.piece_type == PieceType::King(true)
                            || q.piece_type == PieceType::King(false))
                }
                _ => false,
            })
            .map(|(position, king)| {
                let mut moves: Vec<Move> = Vec::new();

                let king = king.unwrap();

                let targets = [
                    position + 10,
                    position + 11,
                    position + 1,
                    position - 9,
                    position - 10,
                    position - 11,
                    position - 1,
                    position + 9,
                ];
                for target in targets {
                    if let Some(on_target) = self.fields[target] {
                        //Attack
                        if on_target.piece_color != king.piece_color {
                            let tmp = Move::create_move(position, target, Some(on_target));
                            moves.push(tmp);
                        }
                    } else {
                        //Normal move
                        let tmp = Move::create_move(position, target, None);
                        if tmp.is_on_board() {
                            moves.push(tmp);
                        }
                    }
                }

                //Castling
                if king.piece_type == PieceType::King(false) {
                    if let Some(rook) = self.fields[position - 4] {
                        if rook.piece_type == PieceType::Rook(false)
                            && self.fields[position - 1].is_none()
                            && self.fields[position - 2].is_none()
                            && self.fields[position - 3].is_none()
                        {
                            let mut tmp = Move::create_move(position, position - 4, None);
                            tmp.castle = Some((true, rook));
                            moves.push(tmp);
                        }
                    }

                    if let Some(rook) = self.fields[position + 3] {
                        if rook.piece_type == PieceType::Rook(false)
                            && self.fields[position + 1].is_none()
                            && self.fields[position + 2].is_none()
                        {
                            let mut tmp = Move::create_move(position, position + 3, None);
                            tmp.castle = Some((true, rook));
                            moves.push(tmp);
                        }
                    }
                }

                moves
            })
            .fold(Vec::new(), |mut acc, mut v| {
                acc.append(&mut v);
                acc
            });
        kings
    }
    fn get_rook_moves(&self, color: &PieceColor) -> Vec<Move> {
        let knights = self
            .fields
            .iter()
            .enumerate()
            .filter(|p| match p {
                (_, Some(q)) => {
                    q.piece_color == *color
                        && (q.piece_type == PieceType::Rook(true)
                            || q.piece_type == PieceType::Rook(false)
                            || q.piece_type == PieceType::Queen)
                }
                _ => false,
            })
            .map(|(position, rook)| {
                let mut moves: Vec<Move> = Vec::new();
                let rook = rook.unwrap();

                let directions = [10, 1];

                for step in directions {
                    let mut d = step;
                    while !((position + d) % 10 == 0
                        || (position + d + 1) % 10 == 0
                        || position + d < 20
                        || position + d > 100)
                    {
                        let tmp =
                            Move::create_move(position, position + d, self.fields[position + d]);
                        if let Some(piece) = tmp.captured {
                            if piece.piece_color != rook.piece_color {
                                moves.push(tmp);
                            }
                            break;
                        } else {
                            moves.push(tmp);
                        }
                        d += step;
                    }

                    d = step;
                    while !((position - d) % 10 == 0
                        || (position - d + 1) % 10 == 0
                        || position - d < 20
                        || position - d > 100)
                    {
                        let tmp =
                            Move::create_move(position, position - d, self.fields[position - d]);
                        if let Some(piece) = tmp.captured {
                            if piece.piece_color != rook.piece_color {
                                moves.push(tmp);
                            }
                            break;
                        } else {
                            moves.push(tmp);
                        }
                        d += step;
                    }
                }

                moves
            })
            .fold(Vec::new(), |mut acc, mut v| {
                acc.append(&mut v);
                acc
            });
        knights
    }
    fn get_bishop_moves(&self, color: &PieceColor) -> Vec<Move> {
        let knights = self
            .fields
            .iter()
            .enumerate()
            .filter(|p| match p {
                (_, Some(q)) => {
                    q.piece_color == *color
                        && (q.piece_type == PieceType::Bishop || q.piece_type == PieceType::Queen)
                }
                _ => false,
            })
            .map(|(position, bishop)| {
                let mut moves: Vec<Move> = Vec::new();
                let bishop = bishop.unwrap();

                let directions = [11, 9];

                for step in directions {
                    let mut d = step;
                    while !((position + d) % 10 == 0
                        || (position + d + 1) % 10 == 0
                        || position + d < 20
                        || position + d > 100)
                    {
                        let tmp =
                            Move::create_move(position, position + d, self.fields[position + d]);
                        if let Some(piece) = tmp.captured {
                            if piece.piece_color != bishop.piece_color {
                                moves.push(tmp);
                            }
                            break;
                        } else {
                            moves.push(tmp);
                        }
                        d += step;
                    }

                    d = step;
                    while !((position - d) % 10 == 0
                        || (position - d + 1) % 10 == 0
                        || position - d < 20
                        || position - d > 100)
                    {
                        let tmp =
                            Move::create_move(position, position - d, self.fields[position - d]);
                        if let Some(piece) = tmp.captured {
                            if piece.piece_color != bishop.piece_color {
                                moves.push(tmp);
                            }
                            break;
                        } else {
                            moves.push(tmp);
                        }
                        d += step;
                    }
                }

                moves
            })
            .fold(Vec::new(), |mut acc, mut v| {
                acc.append(&mut v);
                acc
            });
        knights
    }

    fn get_knight_moves(&self, color: &PieceColor) -> Vec<Move> {
        let knights = self
            .fields
            .iter()
            .enumerate()
            .filter(|p| match p {
                (_, Some(q)) => q.piece_color == *color && q.piece_type == PieceType::Knight,
                _ => false,
            })
            .map(|(position, knight)| {
                let mut moves: Vec<Move> = Vec::new();
                let knight = knight.unwrap();

                let targets = [
                    position + 19, //FL
                    position + 21, //FR
                    position + 12, //RF
                    position - 8,  //RB
                    position - 19, //BR
                    position - 21, //BL
                    position - 12, //LB
                    position + 8,  //LF
                ];
                for target in targets {
                    if let Some(on_target) = self.fields[target] {
                        //Attack
                        if on_target.piece_color != knight.piece_color {
                            let tmp = Move::create_move(position, target, Some(on_target));
                            moves.push(tmp);
                        }
                    } else {
                        //Normal move
                        let tmp = Move::create_move(position, target, None);
                        if tmp.is_on_board() {
                            moves.push(tmp);
                        }
                    }
                }

                moves
            })
            .fold(Vec::new(), |mut acc, mut v| {
                acc.append(&mut v);
                acc
            });
        knights
    }
    fn get_pawn_moves(&self, color: &PieceColor, log: &Vec<Move>) -> Vec<Move> {
        let pawns = self
            .fields
            .iter()
            .enumerate()
            .filter(|p| match p {
                (_, Some(q)) => {
                    q.piece_color == *color
                       // && (q.piece_type == PieceType::Pawn(true)
                       //     || q.piece_type == PieceType::Pawn(false))
                       && if let PieceType::Pawn(_) = q.piece_type {true} else {false}
                }
                _ => false,
            })
            .map(|(position, pawn)| {
                let mut moves: Vec<Move> = Vec::new();
                let step = if pawn.unwrap().piece_color == PieceColor::White {
                    10
                } else {
                    -10
                };
                //Forward moves

                //target for first move
                let target = (position as isize + 2 * step) as usize;
                if self.fields[target].is_none()
                    && pawn.unwrap().piece_type == PieceType::Pawn(false)
                {
                    let tmp = Move::create_move(position, target, None);
                    moves.push(tmp);
                }

                //target for most of the moves
                let target = (position as isize + step) as usize;

                if let None = self.fields[target] {
                    let tmp = Move::create_move(position, target, None);
                    if tmp.is_on_board()
                        && ((*color == PieceColor::White && tmp.target > 90)
                            || (*color == PieceColor::Black && tmp.target < 30))
                    {
                        moves.extend_from_slice(&tmp.get_promoted_pawn(pawn.unwrap().piece_color));
                    } else if tmp.is_on_board() {
                        moves.push(tmp);
                    }
                }

                //Attacks
                if let Some(piece) = self.fields[target + 1] {
                    if piece.piece_color != *color {
                        let tmp = Move::create_move(position, target + 1, Some(piece));
                        if tmp.is_on_board()
                            && ((*color == PieceColor::White && tmp.target > 90)
                                || (*color == PieceColor::Black && tmp.target < 30))
                        {
                            moves.extend_from_slice(
                                &tmp.get_promoted_pawn(pawn.unwrap().piece_color),
                            );
                        } else if tmp.is_on_board() {
                            moves.push(tmp);
                        }
                    }
                }

                if let Some(piece) = self.fields[target - 1] {
                    if piece.piece_color != *color {
                        let tmp = Move::create_move(position, target - 1, Some(piece));
                        if tmp.is_on_board()
                            && ((*color == PieceColor::White && tmp.target > 90)
                                || (*color == PieceColor::Black && tmp.target < 30))
                        {
                            moves.extend_from_slice(
                                &tmp.get_promoted_pawn(pawn.unwrap().piece_color),
                            );
                        } else if tmp.is_on_board() {
                            moves.push(tmp);
                        }
                    }
                }

                //En passant
                if let Some(last_move) = log.iter().last() {
                    if (*color == PieceColor::Black
                        && last_move.source < 40
                        && last_move.target > 50)
                        || (*color == PieceColor::White
                            && last_move.source > 80
                            && last_move.target < 70)
                    {
                        if let Some(pawn) = self.fields[last_move.target] {
                            if let PieceType::Pawn(_) = pawn.piece_type {
                                if position + 1 == last_move.target {
                                    let mut tmp =
                                        Move::create_move(position, target + 1, Some(pawn));
                                    tmp.ep = Some(position + 1);
                                    moves.push(tmp);
                                } else if position - 1 == last_move.target {
                                    let mut tmp =
                                        Move::create_move(position, target - 1, Some(pawn));
                                    tmp.ep = Some(position - 1);
                                    moves.push(tmp);
                                }
                            }
                        }
                    }
                }

                moves
            })
            .fold(Vec::new(), |mut acc, mut v| {
                acc.append(&mut v);
                acc
            });
        pawns
    }

    pub fn make_move(&mut self, mv: &Move, log: &mut Vec<Move>) {
        if let Some(_) = mv.promotion {
            self.fields[mv.source] = None;
            self.fields[mv.target] = mv.promotion;
        } else if let Some(pos) = mv.ep {
            self.fields[pos] = None;
            self.fields[mv.target] = self.fields[mv.source];
            self.fields[mv.source] = None;
        } else if let Some((king_side, rook)) = mv.castle {
            self.fields[mv.target] = self.fields[mv.source];
            self.fields[mv.source] = None;
            if king_side {
                self.fields[mv.source + 1] = Some(rook);
                self.fields[mv.source + 3] = None;
            } else {
                self.fields[mv.source - 1] = Some(rook);
                self.fields[mv.source - 4] = None;
            }
        } else {
            if let Some(mut p) = self.fields[mv.source] {
                match p.piece_type {
                    PieceType::Pawn(false) => p.piece_type = PieceType::Pawn(true),
                    _ => (),
                }
                self.fields[mv.source] = Some(p);
            }
            self.fields[mv.target] = self.fields[mv.source];
            self.fields[mv.source] = None;
        };
        log.push(*mv);
    }

    pub fn undo_move(&mut self, log: &mut Vec<Move>) {
        let mv = log.pop().unwrap();
        if let Some(p) = mv.promotion {
            self.fields[mv.source] = Some(Piece {
                piece_type: PieceType::Pawn(true),
                piece_color: p.piece_color,
            });
            self.fields[mv.target] = mv.captured;
        } else if let Some(pos) = mv.ep {
            self.fields[pos] = mv.captured;
            self.fields[mv.source] = self.fields[mv.target];
            self.fields[mv.target] = None;
        } else if let Some((king_side, rook)) = mv.castle {
            self.fields[mv.source] = self.fields[mv.target];
            self.fields[mv.target] = None;
            if king_side {
                self.fields[mv.source + 3] = Some(rook);
                self.fields[mv.source + 1] = None;
            } else {
                self.fields[mv.source - 4] = Some(rook);
                self.fields[mv.source - 1] = None;
            }
        } else {
            if let Some(mut p) = self.fields[mv.target] {
                if p.piece_type == PieceType::Pawn(true)
                    && ((mv.source < 40 && p.piece_color == PieceColor::White)
                        || (mv.source > 80 && p.piece_color == PieceColor::Black))
                {
                    p.piece_type = PieceType::Pawn(false);
                    self.fields[mv.source] = Some(p);
                }
            }
            self.fields[mv.source] = self.fields[mv.target];
            self.fields[mv.target] = mv.captured;
        };
    }
}
