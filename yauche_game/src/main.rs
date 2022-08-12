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
    let mut move_log: Vec<moves::Move> = Vec::new();

    board.make_move(&test, &mut move_log);
    board.make_move(&a, &mut move_log);
    println!("Current board state is: {}", board);
    println!(
        "Found {} pseudo-valid moves for white!",
        board
            .get_pv_moves(&board::PieceColor::White, &move_log)
            .len()
    );
    println!(
        "Found {:?} pseudo-valid moves for black!",
        board
            .get_pv_moves(&board::PieceColor::Black, &move_log)
            .len()
    );
}
