use crate::kt_board::knights_tour_board;


pub struct knights_tour_solver<'a>{
    current_num : u32,
    attempt_count : u32,
    board : &'a mut knights_tour_board,
    moves : [[i32;2];8],
}


impl knights_tour_solver<'_>{
    pub fn new(board : &mut knights_tour_board) -> knights_tour_solver{
        let moves : [[i32;2];8] = [[2,-1], [2,1], [1,-2], [1,2], [-1,-2], [-1,2], [-2,-1], [-2,1]];
        return knights_tour_solver{board, current_num : 1, attempt_count : 0, moves};
    }

    // recursively solve Knights Tour
    pub fn solve_board(&mut self, row: usize, col : usize) -> bool{
        // set current spot to current_num
        self.board.set_position_value(row ,col,self.current_num as i32);
        self.current_num += 1;
        self.attempt_count += 1;

        // print board once every 1 million attempts
        if self.attempt_count % 1000000 == 0{
            println!("Number of attempts: {}", self.attempt_count);
            self.board.print_board();
        }

        // return true if current num > number of spots on board
        if self.current_num > u32::pow(self.board.get_board_size() as u32, 2){
            return true;
        }

        // try each possible move
        for i in self.moves{
            // see if next spot is valid

            let r : i32 = row as i32+ i[0];
            let c : i32 = col as i32 + i[1];

            if self.board.get_position_value(r as usize, c as usize).1 &&
                self.board.get_position_value(r as usize, c as usize).0 == -1{
                // attempt to solve from next spot
                if self.solve_board(r as usize, c as usize){
                    return true;
                }
            }
        }
        self.board.set_position_value(row,col,-1);
        self.current_num -= 1;

        return false;
    }

    pub fn get_attempt_count(&self) -> u32{
        return self.attempt_count;
    }

}
