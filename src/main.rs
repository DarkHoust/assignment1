use dark_sort_bundle::sorting;

// Tests for the library
fn main() {
    {
        let mut arr = vec![3, 2, 1, 5, 4];
        println!("Original array (Quick Sort): {:?}", arr);
        sorting::quick_sort(&mut arr);
        println!("Sorted array (Quick Sort): {:?}", arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    {
        let mut arr = vec![3, 2, 1, 5, 4];
        println!("Original array (Selection Sort): {:?}", arr);
        sorting::selection_sort(&mut arr);
        println!("Sorted array (Selection Sort): {:?}", arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    {
        let mut arr = vec![3, 2, 1, 5, 4];
        println!("Original array (Insertion Sort): {:?}", arr);
        sorting::insertion_sort(&mut arr);
        println!("Sorted array (Insertion Sort): {:?}", arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    {
        let mut arr = vec![3, 2, 1, 5, 4];
        println!("Original array (Merge Sort): {:?}", arr);
        sorting::merge_sort(&mut arr);
        println!("Sorted array (Merge Sort): {:?}", arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    println!("All tests passed successfully!");
}
