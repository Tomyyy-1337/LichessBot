use std::collections::HashMap;

use chess::{Board, Color, MoveGen};
use rayon::iter::{ParallelBridge, ParallelIterator};

pub struct Engine {

}

impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }

    pub fn best_move(&self, board: &chess::Board) -> Option<chess::ChessMove> {
        let moves = self.evaluate_moves(board);
        moves.into_iter().min_by_key(|(_, eval)| *eval).map(|(mv, _)| mv)
    }

    pub fn evaluate_moves(&self, board: &chess::Board) -> Vec<(chess::ChessMove, i32)> {
        let start = std::time::Instant::now();
        let moves = MoveGen::new_legal(&board);
        let result = moves.into_iter().par_bridge().map(|mv| {
            let new_board = board.make_move_new(mv);
            let eval = self.iterative_deepening(&new_board);
            (mv, eval)
        }).collect();
        let duration = start.elapsed();
        println!("Evaluation took: {:?}", duration);
        result 
    }

    pub fn iterative_deepening(&self, board: &chess::Board) -> i32 {
        let mut lookup = std::collections::HashMap::new();
        let start = std::time::Instant::now();

        let mut val = 0;
        for depth in 2.. {
            val = self.mini_max_alpha_beta(board.clone(), 0, depth, std::i32::MIN + 1, std::i32::MAX, &mut lookup);
            let time = start.elapsed().as_secs_f64();
            if time > 0.2 {
                break;
            }
        }
        return val;
    }

    pub fn mini_max_alpha_beta(&self, board: chess::Board, depth: i32, max_depth: i32, alpha: i32, beta: i32, lookup: &mut HashMap<Board, i32>) -> i32 {
        if depth >= max_depth {          
            return self.eval_position(&board, depth);
        }
        if board.status() != chess::BoardStatus::Ongoing {
            return self.eval_position(&board, depth);
        }
        
        let mut best = alpha;

        let moves = MoveGen::new_legal(&board);
        let mut boards: Vec<(bool, Board)> = moves.map(|mv| (board.piece_on(mv.get_dest()).is_some() || mv.get_promotion().is_some(), board.make_move_new(mv))).collect();
        let sign = if board.side_to_move() == Color::White { -1 } else { 1 };
        boards.sort_by_key(|(_, b)| sign * lookup.get(b).unwrap_or(&0));
        
        for (capture_or_prom, board) in boards {
            let max_depth = if capture_or_prom && depth + 1 == max_depth { max_depth + 1 } else { max_depth };
            let eval = -self.mini_max_alpha_beta(board, depth + 1, max_depth, -beta, -best, lookup);
            if eval > best {
                best = eval;
                if best >= beta {
                    break;
                }
            }
        }    
        lookup.insert(board,best);
        best
    }

    pub fn eval_position(&self, board: &chess::Board, depth: i32) -> i32 {
        let result = match board.status() {
            chess::BoardStatus::Checkmate if board.side_to_move() == Color::White => -10100 + depth,
            chess::BoardStatus::Checkmate if board.side_to_move() == Color::Black =>  10100 - depth,
            chess::BoardStatus::Stalemate => 0,
            _ => chess::ALL_SQUARES.iter().map(|&sq| match board.piece_on(sq) {
                Some(piece) => {
                    let value = match piece {
                        chess::Piece::Pawn => 100,
                        chess::Piece::Knight => 320,
                        chess::Piece::Bishop => 330,
                        chess::Piece::Rook => 500,
                        chess::Piece::Queen => 900,
                        chess::Piece::King => 0,
                    };
                    if board.color_on(sq).unwrap() == Color::White {
                        value
                    } else {
                        -value
                    }
                }
                None => 0
            }).sum()
        };
        if board.side_to_move() == Color::Black {
            -result
        } else {
            result
        }
    }
}