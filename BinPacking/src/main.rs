use crate::packing_algorithm::{first_fit, worst_fit, worst_fit_decreasing, best_fit};


mod packing_algorithm{

    #[derive(Debug, Eq, PartialEq)]
    pub struct Bin{
        space : u32,
        starting_space : u32
    }

    impl Bin{
        pub fn new(size : u32) -> Self {
            Bin{
                space : size,
                starting_space: size
            }
        }

        pub fn get_starting_space(&self) -> u32{
            self.starting_space
        }

        pub fn add_item(&mut self, space: u32){
            self.space -= space;
        }

        pub fn get_remaining_space(&self) -> u32{
            self.space
        }
    }

    /// Place item into first Bin it will fit into
    pub fn first_fit(list : &[u32], bin_size : u32) -> Vec<Bin>{
        let mut bins : Vec<Bin> = vec![Bin::new(bin_size)];
        let mut added = false;
        'outer: for i in list{
            added = false;
            'inner: for b in bins.iter_mut(){
                if b.space >= *i {
                    b.add_item(*i);
                    added = true;
                    break 'inner;
                }
            }
            if !added{
                let mut bi = Bin::new(bin_size);
                bi.add_item(*i);
                bins.push(bi);
            }
        }
        bins
    }

    pub fn worst_fit(list : &[u32], bin_size : u32) -> Vec<Bin>{
     vec![]
    }

    pub fn worst_fit_decreasing(list : &[u32], bin_size : u32) -> Vec<Bin>{
     vec![]
    }

    pub fn best_fit(list : &[u32], bin_size : u32) -> Vec<Bin>{
     vec![]
    }

    /// Return results of a packing algorithm as a String
    pub fn packing_results(list : &[Bin]) -> String{
        let util : f32 = list.iter().map(|b| ((b.space as f32) / (b.starting_space as f32)) as f32).sum::<f32>() / list.len() as f32;
        format!("Bins required: {}\nAverage utilization: {}\n", list.len(), util,)
    }
}


fn main() {

}

#[cfg(test)]
mod tests{
    use crate::packing_algorithm::{Bin, first_fit};

    #[test]
    fn test_first_fit(){
        let list: [u32;6] = [9,2,3,5,1,5];
        let mut b1 = Bin::new(10);
        b1.add_item(10);
        let mut b2 = Bin::new(10);
        b2.add_item(10);
        let mut b3 = Bin::new(10);
        b3.add_item(5);
        let v = vec![b1,b2,b3];

        let res = first_fit(&list,10);
        assert_eq!(v,res);

    }

}