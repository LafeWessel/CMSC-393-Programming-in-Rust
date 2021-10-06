use crate::kt_board::knights_tour_board;

pub struct knights_tour_solver<'a>{
    current_num : u32,
    board : &'a knights_tour_board,
    moves : [[i32;2];8],
}


impl knights_tour_solver<'_>{
    pub fn new(board : &knights_tour_board) -> knights_tour_solver{
        let moves : [[i32;2];8] = [[2,-1], [2,1], [1,-2], [1,2], [-1,-2], [-1,2], [-2,-1], [-2,1]];
        knights_tour_solver{board, current_num : 1, moves}
    }


    pub fn solve_board(&self, row: u32, col : u32){
        // check each possible move
    }
}
