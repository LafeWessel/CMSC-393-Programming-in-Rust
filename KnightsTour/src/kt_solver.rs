use crate::kt_board::knights_tour_board;

pub struct knights_tour_solver<'a>{
    current_num : u32,
    board : &'a knights_tour_board,
}


impl knights_tour_solver<'_>{
    pub fn new(board : &knights_tour_board) -> knights_tour_solver{
        knights_tour_solver{board, current_num : 1}
    }


    pub fn solve_board(row: u32, col : u32){

    }
}