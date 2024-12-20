use chess::{Board, ChessMove, Color, MoveGen};
use std::time::{Duration, Instant};

/// Piece values for evaluation
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 350;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

/// Evaluate the position statically
fn evaluate_board(board: &Board) -> i32 {
    let mut score: i32 = 0;

    for square in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(square) {
            let value = match piece {
                chess::Piece::Pawn => PAWN_VALUE,
                chess::Piece::Knight => KNIGHT_VALUE,
                chess::Piece::Bishop => BISHOP_VALUE,
                chess::Piece::Rook => ROOK_VALUE,
                chess::Piece::Queen => QUEEN_VALUE,
                chess::Piece::King => 0,
            };
            score += if board.color_on(square) == Some(Color::White) {
                value
            } else {
                -value
            };
        }
    }

    score
}

/// Minimax algorithm with depth-limited search
fn minimax(board: &Board, depth: u32, maximizing: bool) -> i32 {
    if depth == 0 || board.status() != chess::BoardStatus::Ongoing {
        return evaluate_board(board);
    }

    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

    let moves = MoveGen::new_legal(&board);
    for chess_move in moves {
        let new_board = board.make_move_new(chess_move);

        let score = minimax(&new_board, depth - 1, !maximizing);
        if maximizing {
            best_score = best_score.max(score);
        } else {
            best_score = best_score.min(score);
        }
    }

    best_score
}

/// Iterative deepening with time control
fn find_best_move_iteratively(board: &Board, max_depth: u32, time_limit: Duration) -> Option<ChessMove> {
    let mut best_move: Option<ChessMove> = None;
    let mut best_score = i32::MIN;
    let start_time = Instant::now();

    for depth in 1..=max_depth {
        // Check if time limit is exceeded
        if start_time.elapsed() >= time_limit {
            println!("Time limit reached. Stopping at depth {}.", depth - 1);
            break;
        }

        let mut current_best_move: Option<ChessMove> = None;
        let mut current_best_score = i32::MIN;

        let moves = MoveGen::new_legal(&board);
        for chess_move in moves {
            let new_board = board.make_move_new(chess_move);

            let score = minimax(&new_board, depth - 1, false);
            if score > current_best_score {
                current_best_score = score;
                current_best_move = Some(chess_move);
            }
        }

        // Update the best move found so far
        if let Some(current_move) = current_best_move {
            best_move = Some(current_move);
            best_score = current_best_score;
        }

        // Print the best move and its evaluation at the current depth
        if let Some(current_move) = best_move {
            println!(
                "Depth {}: Best move: {}, Evaluation: {}",
                depth,
                current_move,
                best_score
            );
        }
    }

    best_move
}

fn main() {
    // Starting position
    let board = Board::default();

    // Set maximum depth and time limit
    let max_depth = 5;
    let time_limit = Duration::from_secs(10);

    // Find the best move iteratively
    if let Some(best_move) = find_best_move_iteratively(&board, max_depth, time_limit) {
        println!("Final Best Move: {}", best_move);
    } else {
        println!("No legal moves available.");
    }
}