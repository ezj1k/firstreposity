fn bubble_sort(nums: &mut [i32]) {
    let n = nums.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn insertion_sort(nums: &mut [i32]) {
    let n = nums.len();
    for i in 1..n {
        // 1 3 4 5 7
        let key = nums[i];
        let mut j = i;
        while j > 0 && nums[j - 1] > key {
            nums[j] = nums[j - 1];
            j -= 1;
        }
        nums[j] = key;
    }
}

fn selection_sort(nums: &mut [i32]) {
    let n = nums.len();
    for i in 0..n {
        let mut min_index = i;
        for j in (i+1)..n {
            if nums[j] < nums[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            nums.swap(i, min_index);
        }
    }
}// i4 7 j2 min=j

//let mut x = 0;
//let y: i64 = x;
//let sum = y+1

fn quick_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }
    quick_sort_recursive(nums, 0, nums.len()-1);
}
fn quick_sort_recursive(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let p = partition(nums, low, high);
        if p > 0 {
            quick_sort_recursive(nums, low, p-1);
        }
        quick_sort_recursive(nums, p+1, high);
    }
}
fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = nums[high];
    let mut i = low;
    for j in low..high {
        if nums[j] < pivot {
            nums.swap(i, j);
            i += 1;
        }
    }// 0 i2 3 1
    nums.swap(i, high);
    i
}

fn merge_sort(nums: &mut [i32]) {
    let n = nums.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    let mut left = nums[..mid].to_vec();
    let mut right = nums[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(nums, &left, &right);
}
fn merge(nums: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            nums[k] = left[i];
            i += 1;
        } else {
            nums[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        nums[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        nums[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn heap_sort(nums: &mut [i32]) {
    let n = nums.len();
    for i in (0..n/2).rev() {
        heapify(nums, n, i);
    }
    for i in (1..n).rev() {
        nums.swap(0, i);
        heapify(nums, i, 0);
    }
}
fn heapify(nums: &mut [i32], heap_size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < heap_size && nums[left] > nums[largest] {
        largest = left;
    }
    if right < heap_size && nums[right] > nums[largest] {
        largest = right;
    }
    if largest != root {
        nums.swap(root, largest);
        heapify(nums, heap_size, largest);
    }
}

fn main() {
    let mut arr = [64, 25, 12, 22, 11];
    println!("Initial: {:?}", arr);

    let mut test = arr.clone();
    bubble_sort(&mut test);
    println!("Bubble sort: {:?}", test);

    let mut test = arr.clone();
    insertion_sort(&mut test);
    println!("Insertion sort: {:?}", test);

    let mut test = arr.clone();
    selection_sort(&mut test);
    println!("Selection sort: {:?}", test);

    let mut test = arr.clone();
    quick_sort(&mut test);
    println!("Quick sort: {:?}", test);

    let mut test = arr.clone();
    merge_sort(&mut test);
    println!("Merge sort: {:?}", test);

    let mut test = arr.clone();
    heap_sort(&mut test);
    println!("Heap sort: {:?}", test);
}
