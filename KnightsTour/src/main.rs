use std::io;

mod KTBoard;
mod KTSolver;

fn main() {
    println!("Hello, world!");

    // read in board size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let size = input.trim().parse().expect("Unable to parse size");

    let mut board = KTBoard::new(size as usize);
    // read in starting column
    input.clear();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let start_column = input.trim().parse().expect("Unable to parse starting column");
    // read in starting row
    input.clear();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let start_row = input.trim().parse().expect("Unable to parse starting row");
    // call solver

    // print board

}
