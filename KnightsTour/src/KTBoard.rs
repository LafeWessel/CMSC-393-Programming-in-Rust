struct KTBoard{
    board : Vec<Vec<u32>>,
}

impl KTBoard{
    fn new(size :usize) -> KTBoard{
        let board = vec![vec![-1;size];size];
        KTBoard{board};
    }
}