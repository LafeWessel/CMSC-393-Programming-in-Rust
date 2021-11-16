
use std::fs;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::time::Instant;
use std::io;


/// Read a file and split into lines, collecting into a Vec<String>
fn read_file(file_name : &str) -> Vec<String>{
    fs::read_to_string(file_name).unwrap().lines().map(|s| String::from(s)).collect()
}


fn main() {

    println!("How many numbers would you like to sort? (max 250,000)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read input, exiting...");
    let count : usize = input.trim().parse().expect("Unable to convert input to usize, exiting...");

    let mut results :Vec<Vec<RunData<String>>> = vec![vec![]];
    let mut index:usize = 0;
    let files = fs::read_dir(".\\text_files")
        .unwrap()
        .map(|p| String::from(p.unwrap().path().to_str().unwrap()))
        .filter(|p| p.contains(".txt"))
        .collect::<Vec<String>>();

    for f in files.iter(){
        println!("File {:?}", f);
        results.push(vec![]);

        println!("Running Selection sort");
        let mut v = read_file(f);
        results[index].push(SortingAlgorithm::sort(SortingAlgorithm::Selection, &mut v[..count]));

        println!("Running Insertion sort");
        let mut v = read_file(f);
        results[index].push(SortingAlgorithm::sort(SortingAlgorithm::Insertion, &mut v[..count]));

        println!("Running Bubble sort");
        let mut v = read_file(f);
        results[index].push(SortingAlgorithm::sort(SortingAlgorithm::BubbleSort, &mut v[..count]));

        println!("Running Merge sort");
        let mut v = read_file(f);
        results[index].push(SortingAlgorithm::sort(SortingAlgorithm::MergeSort, &mut v[..count]));

        println!("Running Quicksort");
        let mut v = read_file(f);
        results[index].push(SortingAlgorithm::sort(SortingAlgorithm::QuickSort, &mut v[..count]));

        index += 1;
    }

    println!("Algorithm:\t\tTime(ms):\tCompares:\t\tSwaps:\t\t\tFile:");
    for f in 0..files.len(){
        for d in results[f].iter(){
            println!("{}\t\t{}",d.output(),files[f]);
        }
    }



}

struct RunData<T>{
    algorithm: SortingAlgorithm,
    time : i32,
    compares: i32,
    swaps: i32,
    result : Option<Vec<T>>,
}

impl<T> RunData<T> {
    /// Print out data for an algorithm run
    fn output(&self) -> String{
        format!("{:?}\t\t{}\t\t{}\t\t{}",self.algorithm,self.time,self.compares,self.swaps)
    }

    /// Increment compares by one
    fn incr_comp(&mut self){
        self.compares += 1;
    }

    /// Increment swaps by one
    fn incr_swap(&mut self){
        self.swaps += 1;
    }

}

#[derive(Debug)]
enum SortingAlgorithm{
    Selection,Insertion,BubbleSort,MergeSort,QuickSort
}

impl SortingAlgorithm{


    fn sort<T: Ord + Clone + Debug>(self, list : &mut [T]) -> RunData<T> {
        match self{
            SortingAlgorithm::Selection => SortingAlgorithm::selection_sort(list),
            SortingAlgorithm::Insertion => SortingAlgorithm::insertion_sort(list),
            SortingAlgorithm::BubbleSort => SortingAlgorithm::bubble_sort(list),
            SortingAlgorithm::MergeSort => SortingAlgorithm::merge_sort(list),
            SortingAlgorithm::QuickSort => SortingAlgorithm::quick_sort(list),
        }
    }

    fn selection_sort<T : Ord + Clone + Debug>(list: &mut [T]) -> RunData<T>{
        // repeatedly find the smallest item and swap it with the next position of the list,
        // starting from the front
        let start = Instant::now();
        let mut swaps = 0;
        let mut compares = 0;

        for i in 0..list.len(){
            let mut smallest_t = &list[i];
            let mut smallest_i = i;
            for j in i+1..list.len(){
                smallest_t = match smallest_t.cmp(&list[j]){
                    Ordering::Greater => {
                        smallest_i = j;
                        &list[j]
                    },
                    Ordering::Equal => smallest_t,
                    Ordering::Less => smallest_t,
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
            swaps,
            result : Option::None,
        }
    }

    fn insertion_sort<T : Ord + Clone + Debug>(list: &mut [T]) -> RunData<T>{
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
            swaps,
            result : Option::None,
        }
    }

    fn bubble_sort<T : Ord + Clone + Debug>(list: &mut [T]) -> RunData<T> {
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
            algorithm: SortingAlgorithm::BubbleSort,
            time: start.elapsed().as_millis() as i32,
            compares,
            swaps,
            result : Option::None,
        }
    }

    fn merge_sort<T: Ord + Clone + Debug>(list: &mut [T]) -> RunData<T>{
        let start = Instant::now();
        let mut data = RunData{
            algorithm: SortingAlgorithm::MergeSort,
            time: 0,
            compares: 0,
            swaps: 0,
            result : Option::None,
        };
        let len = list.len();
        let result = Self::merge_sort_split(list, 0, len, &mut data);

        data.result = Some(result);
        data.time = start.elapsed().as_millis() as i32;
        data
    }

    /// Recursive function for splitting and recombining the array
    fn merge_sort_split<T : Ord + Clone + Debug>(list : &mut [T], begin : usize, end : usize, data: &mut RunData<T>) -> Vec<T>{
        if (end as i32 - begin as i32) <= 1{
            return vec![list[begin].clone()];
        }
        let mid = (end+begin)/2;
        // Split array into individual pieces
        let a = Self::merge_sort_split(list, begin, mid, data);
        let b = Self::merge_sort_split(list, mid, end,data);
        let mut r : Vec<T> = vec![];

        // merge arrays back together
        let mut i  = 0;
        let mut j = 0;

        // add from each until one is exhausted
        while i < a.len() && j < b.len(){
            match a[i].cmp(&b[j]){
                Ordering::Less | Ordering::Equal => {
                    r.push(a[i].clone());
                    i += 1;
                }
                Ordering::Greater => {
                    r.push(b[j].clone());
                    j += 1;
                }
            }
            data.incr_comp();
        }
        // exhaust left array
        while i < a.len(){
            r.push(a[i].clone());
            i += 1;
        }
        // exhaust right array
        while j < b.len(){
            r.push(b[j].clone());
            j += 1;
        }
        r
    }

    fn quick_sort<T : Ord + Clone + Debug>(list: &mut [T]) -> RunData<T>{
        let start = Instant::now();
        let mut data = RunData{
            algorithm: SortingAlgorithm::QuickSort,
            time: 0,
            compares: 0,
            swaps: 0,
            result : Option::None,
        };
        Self::quick_sort_recursive(list, 0, (list.len()-1) as i32,&mut data);
        data.time = start.elapsed().as_millis() as i32;
        data
    }

    /// Find partition and recursively quick sort on either half of the partition
    fn quick_sort_recursive<T : Ord + Clone + Debug>(list : &mut [T], begin : i32, end : i32, data : &mut RunData<T>){

        if begin < end as i32{
            let mid = Self::partition(list, begin as usize, end as usize, data);

            Self::quick_sort_recursive(list, begin, (mid as i32-1) as i32, data);
            Self::quick_sort_recursive(list, (mid + 1) as i32, end, data);
        }
    }

    /// Find the partition of an array slice and swap elements as necessary
    fn partition<T : Ord + Clone + Debug>(list : &mut [T], begin : usize, end : usize, data : &mut RunData<T>) -> usize{
        // pick a pivot -> last element in sublist
        let pivot = list[end].clone();
        let mut i: i32 = begin as i32 - 1;
        for j in begin..=end{
            if pivot.cmp(&list[j]) == Ordering::Greater{
                data.incr_comp();
                i += 1;
                list.swap(j, i as usize);
                data.incr_swap();
            }
        }
        list.swap((i + 1) as usize, end);
        data.incr_swap();
        (i + 1) as usize
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
        SortingAlgorithm::sort(SortingAlgorithm::BubbleSort, &mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::BubbleSort, &mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::BubbleSort, &mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }
    #[test]
    fn merge_sort_test(){
        // short
        let mut v = vec![1,5,2,3,4];
        let res = SortingAlgorithm::sort(SortingAlgorithm::MergeSort, &mut v);
        assert_eq!(vec![1,2,3,4,5],res.result.unwrap());

        // medium
        let mut v = vec![9,5,1,7,3,0,8,4,2,6];
        let res = SortingAlgorithm::sort(SortingAlgorithm::MergeSort, &mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],res.result.unwrap());

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        let res = SortingAlgorithm::sort(SortingAlgorithm::MergeSort, &mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],res.result.unwrap());

    }
    #[test]
    fn quick_sort_test(){
        // short
        let mut v = vec![1,5,3,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::QuickSort, &mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::QuickSort, &mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::QuickSort, &mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }


}