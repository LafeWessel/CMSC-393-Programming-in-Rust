




fn read_file(file_name : &str) -> Vec<&str>{
    vec![]
}







fn main() {
    println!("Hello, world!");
}



impl SortingAlgorithm{

    fn sort<T>(self, list : &mut Vec<T>)
    {
        match self{
            SortingAlgorithm::Selection => SortingAlgorithm::selection_sort(list),
            SortingAlgorithm::Insertion => SortingAlgorithm::insertion_sort(list),
            SortingAlgorithm::Bubble => SortingAlgorithm::bubble_sort(list),
            SortingAlgorithm::Merge => SortingAlgorithm::merge_sort(list),
            SortingAlgorithm::Quicksort => SortingAlgorithm::quick_sort(list),
        }
    }

    fn selection_sort<T>(list: &mut [T]){

    }
    fn insertion_sort<T>(list: &mut [T]){

    }
    fn bubble_sort<T>(list: &mut [T]){

    }
    fn merge_sort<T>(list: &mut [T]){

    }
    fn quick_sort<T>(list: &mut [T]){

    }
}

enum SortingAlgorithm{
    Selection,Insertion,Bubble,Merge,Quicksort
}


#[cfg(test)]
mod tests{
    use crate::SortingAlgorithm;

    #[test]
    fn insertion_sort_test(){
        // short
        let mut v = vec![1,5,3,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Insertion,&mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Insertion,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::Insertion,&mut v);
        assert_eq!(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

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
        assert_eq!(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

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
        assert_eq!(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }
    #[test]
    fn merge_sort_test(){
        // short
        let mut v = vec![1,5,3,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Merge,&mut v);
        assert_eq!(vec![1,2,3,4,5],v);

        // medium
        let mut v = vec![1,5,3,7,9,0,8,6,2,4];
        SortingAlgorithm::sort(SortingAlgorithm::Merge,&mut v);
        assert_eq!(vec![0,1,2,3,4,5,6,7,8,9],v);

        // long
        let mut v = vec![1,5,3,7,9,0,8,6,2,4,10,16,14,15,19,17,18,12,11,13];
        SortingAlgorithm::sort(SortingAlgorithm::Merge,&mut v);
        assert_eq!(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

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
        assert_eq!(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],v);

    }


}