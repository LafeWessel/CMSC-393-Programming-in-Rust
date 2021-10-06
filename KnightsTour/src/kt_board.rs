pub struct knights_tour_board{
    board : Vec<Vec<i32>>,
}

impl knights_tour_board{
    pub fn new(size :usize) -> knights_tour_board{
        let board = vec![vec![-1;size];size];
        return knights_tour_board{board};
    }

    pub fn get_position_value(&self, row : u32, col: u32) -> i32{
        self.board[row as usize][col as usize]
    }

    pub fn set_position_value(&mut self, row: u32, col : u32, val : i32){
        self.board[row as usize][col as usize] = val;
    }

    pub fn print_board(&self){
        println!("Printing board")
    }
}