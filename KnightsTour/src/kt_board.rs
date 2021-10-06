use std::fmt::Debug;

pub struct knights_tour_board{
    board : Vec<Vec<i32>>,
}

impl knights_tour_board{
    pub fn new(size :usize) -> knights_tour_board{
        let board = vec![vec![-1;size];size];
        return knights_tour_board{board};
    }

    // Get value of position on board, return false if unable to
    pub fn get_position_value(&self, row : usize, col: usize) -> (i32,bool){
        if self.valid_position(row,col){
            return (self.board[row][col],true);
        }
        (0,false)
    }

    // Set value of position on board, return false if unable to
    pub fn set_position_value(&mut self, row: usize, col : usize, val : i32) -> bool{
        if self.valid_position(row,col){
            self.board[row][col] = val;
            return true;
        }
        return false;
    }

    // determine if position is on board (i.e. not off sides)
    pub fn valid_position(&self, row : usize, col :usize) -> bool{
        row >= 0 && row < self.board.len() && col >= 0 && col < self.board.len()
    }

    pub fn print_board(&self){
        for i in self.board.iter(){
            for j in i.iter(){
                if *j == -1{ print!(" . ") }
                else { print!(" {} ", *j) }
            }
            println!();
        }
    }
}