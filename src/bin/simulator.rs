use std::{io::{Stdin, BufReader, BufRead}, process::{Command, Stdio}};
use std::io::Write;

use checkers::*;

fn main() {
    let mut main_stdin = std::io::stdin();

    let args: Vec<String> = std::env::args().collect();

    // if args.len() != 3 {
    //     eprintln!("Usage: {} <path_to_ai1_binary> <path_to_ai2_binary>", args[0]);
    //     return;
    // }
    
    let ai1_path = "/home/patrik/Code/Games/checkers/target/release/interactive";
    let ai2_path = "/home/patrik/Code/Games/checkers/target/release/checkers";

    let mut board = Board::new();

    let mut ai1_child = Command::new(ai1_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start AI 1");

    let mut ai2_child = Command::new(ai2_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start AI 2");    

    let mut ai1_color = Color::Black;
    let mut ai2_color = -ai1_color;

    let mut ai1_stdin = ai1_child.stdin.as_mut().unwrap();
    let mut ai2_stdin = ai2_child.stdin.as_mut().unwrap();
    let mut ai1_stdout = ai1_child.stdout.as_mut().unwrap();
    let mut ai2_stdout = ai2_child.stdout.as_mut().unwrap();
    
    writeln!(ai1_stdin, "{}", ai1_color).unwrap();
    writeln!(ai2_stdin, "{}", ai2_color).unwrap();

    let mut wins1 = 0;
    let mut wins2 = 0;
    let mut ties = 0;

    let mut evals = vec![];

    const MAX_MOVES: u32 = 200;

    let mut cnt_moves = 0;
    let mut cgame = 0;
    let games_total = 30;

    let mut time1 = 0;
    let mut time2 = 0;


    while cgame != games_total {
        println!("{}", board);
        println!("{}", board.turn);
        println!("move: {}", cnt_moves);

        if !board.exists_valid_move() || cnt_moves > MAX_MOVES {

            if cnt_moves > MAX_MOVES {
                ties += 1;
            }
            else {
                if board.turn == ai1_color {
                    wins2 += 1;
                }
                else {
                    wins1 += 1;
                }
            }
            let heur = heuristic(&board);
            if board.turn == ai1_color {
                evals.push(heur);
            }
            else {
                evals.push(-heur);
            }
            cnt_moves = 0;
            cgame += 1;
            ai1_color = -ai1_color;
            ai2_color = -ai2_color;
            writeln!(ai1_stdin, "{}", ai1_color).unwrap();
            writeln!(ai2_stdin, "{}", ai2_color).unwrap();
            board = Board::new();
        }

        let (stdin, stdout, time) = if board.turn == ai1_color {
            (&mut ai1_stdin, &mut ai1_stdout, &mut time1)
        } else {
            (&mut ai2_stdin, &mut ai2_stdout, &mut time2)
        };

        cnt_moves += 1;        

        for cp in &board.must_jump {
            write!(stdin, "{} ", cp).expect("Failed to write to stdin of ai1.");
        }
        writeln!(stdin).unwrap();
        writeln!(stdin, "{}", board).unwrap();
        stdin.flush().unwrap();

        let time_measure = std::time::Instant::now();

        let mut output = BufReader::new(stdout);

        let mut ai_out = String::new();
        output.read_line(&mut ai_out).unwrap();

        *time += time_measure.elapsed().as_millis();


        ai_out = ai_out.trim().to_string();
        let mv = PieceMove::from_str(&ai_out);

        if !board.make_move(mv) {
            println!("AI {} made invalid move {} (loser).", if board.turn==ai1_color { 1 } else { 2 }, mv);
            break;
        }
    }
    writeln!(ai1_stdin, "exit").unwrap();
    writeln!(ai2_stdin, "exit").unwrap();

    evals.sort();
    println!("{:?}", evals);

    println!("AI1 wins: {}; AI2 wins: {}; ties: {}; AI1 time: {}; AI2 time: {}; median eval: {}", wins1, wins2, ties, time1/games_total, time2/games_total, evals[evals.len()/2]);
}
