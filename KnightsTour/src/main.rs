use std::io;
use crate::kt_board::knights_tour_board;
use crate::kt_solver::knights_tour_solver;

mod kt_board;
mod kt_solver;

fn main() {


    // read in board size
    println!("What size board do you want to solve?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let size : usize = input.trim().parse().expect("Unable to parse size");
    let mut board = knights_tour_board::new(size);

    // read in starting row
    println!("What row would you like to start at?");
    input.clear();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let start_row : u32 = input.trim().parse().expect("Unable to parse starting row");

    // read in starting column
    println!("What column would you like to start at?");
    input.clear();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let start_column : u32 = input.trim().parse().expect("Unable to parse starting column");

    // call solver
    println!("Solving a {}x{} board, starting from {},{}",size,size,start_row,start_column);
    let mut solver = knights_tour_solver::new(&mut board);
    solver.solve_board(start_row as usize, start_column as usize);


    // print board
    println!("------------------");
    println!("Took {} attempts", solver.get_attempt_count());
    board.print_board();

}
