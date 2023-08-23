use std::collections::HashMap;

use rand::{thread_rng, seq::SliceRandom};

use super::*;

pub struct AI {}

fn ori_score(val: i64, my_t: Color, n_t: Color) -> i64 {
    if my_t != n_t {
        return -val;
    }
    return val;
}

impl AI {
    pub fn new() -> Self {
        Self {}
    }
    pub fn compute_move(&mut self, board: &Board) -> PieceMove {

        let mut board = board.clone();

        let mut to_explore = if board.must_jump.len() != 0 {
            board.must_jump.clone()
        }
        else {
            board.piece_pos(board.turn)
        };

        let mut rng = thread_rng();
        to_explore.shuffle(&mut rng);

        let to_explore = sort_by_heuristic(board.clone(), to_explore, heuristic);

        let old_board = board.clone();
        let mut best_move: Option<(i64, PieceMove)> = None;
        let mut thr_handles = vec![];
        for cp in to_explore {
            for dir in DIRS {
                let pm = PieceMove {pos: cp, dir};
                if board.make_move(pm) {
                    thr_handles.push(std::thread::spawn(move || {
                        let mp = if board.turn == old_board.turn {1} else {-1};
                        (evaluate(board)*mp, pm)
                    }));
                    board = old_board.clone();
                }
            }
        }
        for handle in thr_handles {
            let (score, mv) = handle.join().unwrap();
            if let Some(best_move) = &mut best_move {
                if score > best_move.0 {
                    best_move.0 = score;
                    best_move.1 = mv;
                }
            }
            else {
                best_move = Some((score, mv));
            }
        }
        eprintln!("Eval: {:?}", best_move.unwrap().0);
        return best_move.unwrap().1;
    }
}


fn nnminimax(mut board: Board, depth: u16) -> i64 {
    if depth >= 5 && board.must_jump.len() == 0 {
        return heuristic(&board);
    } 
    let to_explore = if board.must_jump.len() != 0 {
        board.must_jump.clone()
    }
    else {
        board.piece_pos(board.turn)
    }; 
    
    let mut bst = LOST;
    let old_board = board.clone();

    for cp in to_explore {
        for dir in DIRS {
            let pm = PieceMove {pos: cp, dir};
            if board.make_move(pm) {
                let score = if board.turn == old_board.turn {
                    nnminimax(board, depth+1)
                }
                else {
                    -nnminimax(board, depth+1)
                };
                

                if score > bst {
                    bst = score;
                }

                board = old_board.clone();
            }
        }
    }
    return bst;
}
fn nminimax(mut board: Board, depth: u16) -> i64 {
    if depth >= 5 {
        return simple_heuristic(&board);
    } 
    let to_explore = if board.must_jump.len() != 0 {
        board.must_jump.clone()
    }
    else {
        board.piece_pos(board.turn)
    }; 
    
    let mut bst = LOST;
    let old_board = board.clone();

    for cp in to_explore {
        for dir in DIRS {
            let pm = PieceMove {pos: cp, dir};
            if board.make_move(pm) {
                let score = if board.turn == old_board.turn {
                    nminimax(board, depth+1)
                }
                else {
                    -nminimax(board, depth+1)
                };
                

                if score > bst {
                    bst = score;
                }

                board = old_board.clone();
            }
        }
    }
    return bst;
}

fn dhminimax(mut board: Board, depth: i16, mut alpha: i64, beta: i64, (transp_table, overwritten, utilized): (&mut HashMap<u64, (i64, i16, i64, i64)>, &mut u64, &mut u64)) -> i64 {
    if let Some(&(teval, tdepth, talpha, tbeta)) = transp_table.get(&board.hash) {
        if tdepth >= depth && talpha <= alpha && tbeta >= beta {
            *utilized += 1;
            return teval;
        }
    }
    if depth <= 0 && board.must_jump.len() == 0 {
        return heuristic(&board);
    } 
    let to_explore = if board.must_jump.len() != 0 {
        board.must_jump.clone()
    }
    else {
        board.piece_pos(board.turn)
    }; 
    
    let old_board = board.clone();

    // let to_explore = sort_by_heuristic(board.clone(), to_explore, heuristic);
    let old_alpha = alpha;
    let old_beta = beta;

    for cp in to_explore {
        for dir in DIRS {
            let pm = PieceMove {pos: cp, dir};
            let ndepth = if board.must_jump.len() != 1 {depth-1} else {depth};
            if board.make_move(pm) {
                let score = if board.turn == old_board.turn {
                    dhminimax(board, ndepth, alpha, beta, (transp_table, overwritten, utilized))
                }
                else {
                    -dhminimax(board, ndepth, -beta, -alpha, (transp_table, overwritten, utilized))
                };
                

                if score > alpha {
                    alpha = score;
                    if alpha >= beta {
                        return alpha;
                    }
                }

                board = old_board.clone();
            }
        }
    }

    // store with old_alpha and old_beta
    if board.must_jump.len() != 1 && transp_table.insert(board.hash, (alpha, depth.max(0), old_alpha, old_beta)).is_some() {
        *overwritten += 1;
    }

    return alpha;
}

fn dminimax(mut board: Board, depth: u16, mut alpha: i64, beta: i64) -> i64 {
    if depth >= 8 && board.must_jump.len() == 0 {
        return heuristic(&board);
    } 
    let to_explore = if board.must_jump.len() != 0 {
        board.must_jump.clone()
    }
    else {
        board.piece_pos(board.turn)
    }; 
    
    let old_board = board.clone();

    // let to_explore = sort_by_heuristic(board.clone(), to_explore, heuristic);

    for cp in to_explore {
        for dir in DIRS {
            let pm = PieceMove {pos: cp, dir};
            let ndepth = if board.must_jump.len() != 1 {depth+1} else {depth};
            if board.make_move(pm) {
                let score = if board.turn == old_board.turn {
                    dminimax(board, ndepth, alpha, beta)
                }
                else {
                    -dminimax(board, ndepth, -beta, -alpha)
                };
                

                if score > alpha {
                    alpha = score;
                    if alpha >= beta {
                        return alpha;
                    }
                }

                board = old_board.clone();
            }
        }
    }
    return alpha;
}

const MAX_COMPUTE: i64 = 1_000_000;

fn evaluate(board: Board) -> i64 {
    let mut overwritten = 0;
    let mut utilized = 0;
    // eprintln!("Board hash: {}", board.hash);
    let eval = dhminimax(board, 6, LOST, WIN, (&mut Default::default(), &mut overwritten, &mut utilized));
    // eprintln!("Overwritten: {}; utilized: {}", overwritten, utilized);
    return eval;
}

fn simple_heuristic(board: &Board) -> i64 {
    return board.piece_pos(board.turn).len() as i64 - board.piece_pos(-board.turn).len() as i64;
}

