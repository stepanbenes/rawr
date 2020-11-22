extern crate pleco;
extern crate pleco_engine;

use pleco::{Board, PieceType, Player};
use pleco_engine::movepick::MovePicker;
use pleco_engine::search::eval::Evaluation;
use pleco_engine::search::Searcher;

fn main() {
    let mut board = Board::start_pos();
    assert_eq!(board.count_piece(Player::White, PieceType::P), 8);
    assert_eq!(
        &board.fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
    let moves = board.generate_moves();
    for m in moves.iter() {
        println!("{:?}", m);
    }
    board.pretty_print();
    board.apply_move(moves[0]);
    board.pretty_print();

    // let moves = board.generate_scoring_moves();
    // for m in moves.iter() {
    //     println!("{:?}", m);
    // }

    // let x = MovePicker.moves;
}
