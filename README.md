# Dark Sort Bundle

The **[Dark Sort Bundle](https://crates.io/crates/dark_sort_bundle)** is a Rust library that provides efficient implementations of various sorting algorithms. It offers a collection of commonly used sorting algorithms, each designed to handle different scenarios and data sizes.

***

# Usage

1) Add the library as a dependency in your Cargo.toml file.
```
[dependencies]
dark_sort_bundle = "0.1.0"
// other dependencies
```
2) Import the desired sorting functions from the ```dark_sort_bundle::sorting``` module into your Rust code.
3) Use the imported functions to sort arrays or collections of data efficiently.


<img src="./demo.gif">

***
# Examples
```
// Quick Sort
use dark_sort_bundle::sorting;

fn main() {
    let mut arr = vec![3, 2, 1, 5, 4];
    sorting::quick_sort(&mut arr);
    println!("Sorted array (Quick Sort): {:?}", arr);
}
```

```
// Selection Sort
use dark_sort_bundle::sorting;

fn main() {
    let mut arr = vec![3, 2, 1, 5, 4];
    sorting::selection_sort(&mut arr);
    println!("Sorted array (Selection Sort): {:?}", arr);
}
```

```
// Insertion Sort
use dark_sort_bundle::sorting;

fn main() {
    let mut arr = vec![3, 2, 1, 5, 4];
    sorting::insertion_sort(&mut arr);
    println!("Sorted array (Insertion Sort): {:?}", arr);
}
```

```
// Merge Sort
use dark_sort_bundle::sorting;

fn main() {
    let mut arr = vec![3, 2, 1, 5, 4];
    sorting::merge_sort(&mut arr);
    println!("Sorted array (Merge Sort): {:?}", arr);
}
```


