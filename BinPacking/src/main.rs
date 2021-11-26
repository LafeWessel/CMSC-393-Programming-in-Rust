use crate::packing_algorithm::{first_fit, worst_fit, worst_fit_decreasing, best_fit, packing_results};
use std::fs::File;
use std::io::BufReader;
use std::fs;


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
        pub fn add_item(&mut self, space: u32){
            self.space -= space;
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

    /// Place item in Bin with most remaining room
    pub fn worst_fit(list : &[u32], bin_size : u32) -> Vec<Bin>{
        let mut bins = vec![Bin::new(bin_size)];
        for i in list{
            let most_room = bins.iter().map(|b| b.space).max().unwrap();
            if *i <= most_room{
                let mut most = bins.iter_mut().find(|b| b.space == most_room).unwrap();
                most.add_item(*i);
            }else{
                let mut b = Bin::new(bin_size);
                b.add_item(*i);
                bins.push(b);
            }
        }
        bins
    }

    /// Sort the array into biggest first, then use worst_fit
    pub fn worst_fit_decreasing(list : &[u32], bin_size : u32) -> Vec<Bin>{
        let mut list_sort = list.to_vec();
        list_sort.sort_unstable();
        list_sort.reverse();

        worst_fit(&list_sort, bin_size)
    }

    /// Put next item into the Bin with the minimum necessary space
    pub fn best_fit(list : &[u32], bin_size : u32) -> Vec<Bin>{
        let mut bins = vec![Bin::new(bin_size)];

        for i in list{
            // find minimum bin with space >= current item
            match bins.iter().filter(|b| b.space >= *i).map(|b| b.space).min(){
                None => {
                    // need to add a new bin
                    let mut bin = Bin::new(bin_size);
                    bin.add_item(*i);
                    bins.push(bin);
                }
                Some(min) => {
                    // have a bin to add to
                    let mut bin = bins.iter_mut().find(|b| b.space == min).unwrap();
                    bin.add_item(*i);
                }
            }

        }

        bins
    }

    /// Return results of a packing algorithm as a String
    pub fn packing_results(list : &[Bin]) -> String{
        let util : f32 = list.iter().map(|b| ((b.starting_space -  b.space) as f32 / (b.starting_space as f32)) * 100.0 as f32).sum::<f32>() / list.len() as f32;
        format!("Bins required: {}\nAverage utilization: {}\n", list.len(), util,)
    }
}


fn main() {
    let items = read_items("./order_sizes.txt");

    let item_count = [10,100,1000,10000];
    let bin_size = [1000,5000,25000];
    for i in item_count{
        for b in bin_size{
            // first
            let v = first_fit(&items[..i],b);
            println!("First Fit (ct:{},sz:{})\n{}",i,b,packing_results(&v));
            // worst
            let v = worst_fit(&items[..i],b);
            println!("Worst Fit (ct:{},sz:{})\n{}",i,b,packing_results(&v));
            // worst decr
            let v = worst_fit_decreasing(&items[..i],b);
            println!("Worst Fit Decr (ct:{},sz:{})\n{}",i,b,packing_results(&v));
            // best
            let v = best_fit(&items[..i],b);
            println!("Best Fit (ct:{},sz:{})\n{}",i,b,packing_results(&v));
        }
    }
}

fn read_items(file_path : &str) -> Vec<u32>{
    let mut content = fs::read_to_string(file_path).unwrap();
    let mut items = content.lines().map(|s| s.parse().unwrap()).collect::<Vec<u32>>();

    items
}

#[cfg(test)]
mod tests{
    use crate::packing_algorithm::{Bin, first_fit, worst_fit, worst_fit_decreasing, best_fit};

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

    #[test]
    fn test_worst_fit(){
        let list: [u32;6] = [9,2,3,5,1,5];
        let mut b1 = Bin::new(10);
        b1.add_item(10);
        let mut b2 = Bin::new(10);
        b2.add_item(10);
        let mut b3 = Bin::new(10);
        b3.add_item(5);
        let v = vec![b1,b2,b3];

        let res = worst_fit(&list,10);
        assert_eq!(v,res);
    }

    #[test]
    fn test_worst_fit_decreasing(){
        let list: [u32;6] = [9,2,3,5,1,5];
        let mut b1 = Bin::new(10);
        b1.add_item(9);
        let mut b2 = Bin::new(10);
        b2.add_item(10);
        let mut b3 = Bin::new(10);
        b3.add_item(6);
        let v = vec![b1,b2,b3];

        let res = worst_fit_decreasing(&list,10);
        assert_eq!(v,res);
    }

    #[test]
    fn test_best_fit_decreasing(){
        let list: [u32;6] = [9,2,3,5,1,5];
        let mut b1 = Bin::new(10);
        b1.add_item(10);
        let mut b2 = Bin::new(10);
        b2.add_item(10);
        let mut b3 = Bin::new(10);
        b3.add_item(5);
        let v = vec![b1,b2,b3];

        let res = best_fit(&list,10);
        assert_eq!(v,res);
    }

}