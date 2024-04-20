pub mod sorting {
    // Quick Sort
    pub fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot_index = partition(arr);
        quick_sort(&mut arr[..pivot_index]);
        quick_sort(&mut arr[pivot_index + 1..]);
    }

    fn partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
        let pivot_index = arr.len() - 1;
        let pivot_value = arr[pivot_index].clone();
        let mut i = 0;
        for j in 0..pivot_index {
            if &arr[j] <= &pivot_value {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pivot_index);
        i
    }

    // Selection sort
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        for i in 0..arr.len() {
            let mut min_index = i;
            for j in (i + 1)..arr.len() {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            if i != min_index {
                arr.swap(i, min_index);
            }
        }
    }

    // Insertion sort
    pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    // Merge sort
    pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        let mut merged = Vec::with_capacity(arr.len());
        merge(&arr[..mid], &arr[mid..], &mut merged);
        arr.clone_from_slice(&merged);
    }

    fn merge<T: Ord + Clone>(left: &[T], right: &[T], merged: &mut Vec<T>) {
        let mut i = 0;
        let mut j = 0;
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                merged.push(left[i].clone());
                i += 1;
            } else {
                merged.push(right[j].clone());
                j += 1;
            }
        }
        merged.extend_from_slice(&left[i..]);
        merged.extend_from_slice(&right[j..]);
    }
}
