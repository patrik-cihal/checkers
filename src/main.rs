use std::{io::{stdin, Stdin}, ops::{IndexMut, Index, Neg}, fmt::Display, error::Error};
use checkers::*;

mod ai;
use ai::AI;


fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let mut ai = AI::new();

    let mut color_str = String::new();
    stdin.read_line(&mut color_str)?;
    color_str = color_str.trim().into();

    let mut color = Color::from_str(&color_str);

    loop {
        let mut inp = String::new();
        stdin.read_line(&mut inp).unwrap();
        inp = inp.trim().into();
        if inp == "white" || inp == "black" {
            color = Color::from_str(&inp);
            continue;
        }
        if inp == "exit" {
            break Ok(());
        }
        let must_jump = inp.split_whitespace().map(|pstr| {
            CellPos::from_str(pstr)
        }).collect::<Vec<_>>();

        // eprintln!("Reading board from stdin...");
        let mut board = Board::from_stdin(&stdin);
        board.turn = color;
        board.must_jump = must_jump;

        // eprintln!("Parsed board: \n{}", board);

        // eprintln!("Computing best move...");
        let mv = ai.compute_move(&board);

        println!("{}", mv);
        // eprintln!("Printed move to stdout.");
    }
}