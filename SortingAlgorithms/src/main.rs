
use std::fs;
use std::cmp::Ordering;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::time::Instant;

/// Read a file and split into lines, collecting into a Vec<String>
fn read_file(file_name : &str) -> Vec<String>{
    fs::read_to_string(file_name).unwrap().lines().map(|s| String::from(s)).collect()
}







fn main() {

}

struct RunData{
    algorithm: SortingAlgorithm,
    time : i32,
    compares: i32,
    swaps: i32,
}

impl RunData{
    /// Print out data for an algorithm run
    fn output(&self){
        println!("Algorithm:\t\tTime:\t\tCompares:\t\tSwaps\t\t");
        println!("{:?}\t\t{}\t\t{}\t\t{}",self.algorithm,self.time,self.compares,self.swaps);
    }

}

#[derive(Debug)]
enum SortingAlgorithm{
    Selection,Insertion,Bubble,Merge,Quicksort
}

impl SortingAlgorithm{


    fn sort<T: Ord + Clone + Debug>(self, list : &mut Vec<T>) -> RunData
    {
        match self{
            SortingAlgorithm::Selection => SortingAlgorithm::selection_sort(list),
            SortingAlgorithm::Insertion => SortingAlgorithm::insertion_sort(list),
            SortingAlgorithm::Bubble => SortingAlgorithm::bubble_sort(list),
            SortingAlgorithm::Merge => SortingAlgorithm::merge_sort(list),
            SortingAlgorithm::Quicksort => SortingAlgorithm::quick_sort(list),
        }
    }

    fn selection_sort<T : Ord + Clone + Debug>(mut list: &mut [T]) -> RunData{
        // repeatedly find the smallest item and swap it with the next position of the list,
        // starting from the front
        let start = Instant::now();
        let mut swaps = 0;
        let mut compares = 0;

        for i in 0..list.len(){
            let mut smallest_T  = &list[i];
            let mut smallest_i = i;
            for j in i+1..list.len(){
                smallest_T = match smallest_T.cmp(&list[j]){
                    Ordering::Greater => {
                        smallest_i = j;
                        &list[j]
                    },
                    Ordering::Equal => smallest_T,
                    Ordering::Less => smallest_T,
                };
                compares += 1;
            }
            list.swap(i,smallest_i);
            swaps += 1;
        }
        RunData{
            algorithm: SortingAlgorithm::Selection,
            time: start.elapsed().as_millis() as i32,
            compares,
            swaps
        }
    }
    fn insertion_sort<T : Ord + Clone + Debug>(list: &mut [T]) -> RunData{
        // iterate through elements, for each element, swap it backwards until in proper spot
        let start = Instant::now();
        let mut compares = 0;
        let mut swaps = 0;

        for i in 1..list.len(){
            let mut curr = list[i].clone();
            let mut j : usize = i - 1;

            while curr.cmp(&list[j]) == Ordering::Less{
                compares += 1;
                list.swap(j,j+1);
                curr = list[j].clone();
                swaps += 1;
                j = if j == 0 {0} else {j-1};
            }
        }

        RunData{
            algorithm: SortingAlgorithm::Insertion,
            time: start.elapsed().as_millis() as i32,
            compares,
            swaps
        }
    }
    fn bubble_sort<T : Ord + Clone + Debug>(list: &mut [T]) -> RunData {
        // for a moving window of 2 elements, move along array and swap if necessary
        // continue until array is sorted

        let start = Instant::now();
        let mut compares = 0;
        let mut swaps = 0;

        let mut sorted : bool = false;

        while !sorted{
            let mut j = 0;
            sorted = true;
            for i in 0..list.len()-1{
                j = i+1;
                if list[i].cmp(&list[j]) == Ordering::Greater{
                    compares += 1;
                    list.swap(i,j);
                    swaps += 1;
                    sorted = false;
                }
            }
        }

        RunData{
            algorithm: SortingAlgorithm::Bubble,
            time: start.elapsed().as_millis() as i32,
            compares,
            swaps,
        }
    }

    fn merge_sort<T : Ord + Clone + Debug>(list: &mut Vec<T>) -> RunData{

        println!("Running merge sort");

        Self::merge_sort_split(list, 0, list.len()-1);



        RunData{
            algorithm: SortingAlgorithm::Merge,
            time: 0,
            compares: 0,
            swaps: 0
        }
    }

    /// Recursive function for splitting and recombining the array
    fn merge_sort_split<T : Ord + Clone + Debug>(list : &mut Vec<T>, begin : usize, end : usize){
        if (end as i32 - begin as i32) <= 1{
            return;
        }
        let mid = (end+begin)/2;
        println!("Bounds: begin: {}, end: {}",begin, end);
        // Split array into individual pieces
        Self::merge_sort_split(list, begin, mid);
        Self::merge_sort_split(list, mid, end);




        // merge arrays back together
        let mut i  = begin;
        let mut j = mid;
        while i < mid  && ( (list[i].cmp(&list[j]) == Ordering::Greater) || j <= end){
            println!("i:{}={:?}, j:{}={:?}, mid:{}",i,&list[i],j,&list[j],mid);
            if list[i].cmp(&list[j]) == Ordering::Greater{
                println!("Swapping {}:{:?} <-> {}:{:?} in list:{:?}",i,&list[i],j,&list[j],list);
                list.swap(i,j);
                j = if j == end {j} else {j+1}
            }
            else {
                i = if i == mid {i} else {i+1}
            }
        }
        println!("{:?}",list);

    }


    fn quick_sort<T : Ord + Clone + Debug>(list: &mut [T]) -> RunData{
        RunData{
            algorithm: SortingAlgorithm::Quicksort,
            time: 0,
            compares: 0,
            swaps: 0
        }
    }

}


#[cfg(test)]
mod tests{
    use crate::SortingAlgorithm;


    #[test]
    fn insertion_sort_test(){
        // short
        let mut v = vec![5,1,3,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Insertion,&mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Insertion,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::Insertion,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }
    #[test]
    fn selection_sort_test(){
        // short
        let mut v = vec![1,5,3,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Selection,&mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Selection,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::Selection,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }
    #[test]
    fn bubble_sort_test(){
        // short
        let mut v = vec![1,5,3,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Bubble,&mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Bubble,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::Bubble,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }
    #[test]
    fn merge_sort_test(){
        // short
        let mut v = vec![1,5,2,3,4];
        SortingAlgorithm::sort(SortingAlgorithm::Merge,&mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Merge,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::Merge,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }
    #[test]
    fn quick_sort_test(){
        // short
        let mut v = vec![1,5,3,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Quicksort,&mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Quicksort,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::Quicksort,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }


}