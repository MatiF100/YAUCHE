use crate::board::Board;

mod board;
mod moves;

fn main() {
    let mut board = board::Board::default();
    let a = moves::Move {
        captured: None,
        promotion: None,
        source: 85,
        target: 65,
        ep: None,
        castle: None,
    };
    let test = moves::Move {
        captured: None,
        promotion: None,
        source: 34,
        target: 64,
        ep: None,
        castle: None,
    };
    let tes2 = moves::Move {
        captured: None,
        promotion: None,
        source: 33,
        target: 63,
        ep: None,
        castle: None,
    };
    let mut move_log: Vec<moves::Move> = Vec::new();

    //board.make_move(&test, &mut move_log);
    board.make_move(&a, &mut move_log);
    //board.make_move(&tes2, &mut move_log);
    println!("Current board state is: {}", board);
    println!(
        "Found {} valid moves for white!",
        board
            .get_v_moves(&board::PieceColor::White, &move_log)
            .len()
    );
    println!(
        "Found {:?} valid moves for black!",
        board
            .get_v_moves(&board::PieceColor::Black, &move_log)
            .len()
    );

    let perft_board = board::Board::default();
    let mut move_log: Vec<moves::Move> = Vec::new();
    println!("Perft result: {}", perft_board.perft(3, &mut move_log, &board::PieceColor::White));
}
