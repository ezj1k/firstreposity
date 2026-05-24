fn bubblesort(list: &mut [i32]) {
    let mut length = list.len();
    while length > 1 {
        for i in 1..length {
            if list[i-1] > list[i] {
                list.swap(i, i-1);
            }
        }
        length -= 1;
    }
}
//------
// fn insertionsort(list: &mut [i32]) {
    // let length = list.len();
    // for i in 1..length {
    //     let mut j = i;

    //     while j>0 && list[j] < list[j-1] {
    //         list.swap(j, j-1);
    //         j -= 1;
    //     }
    // }
// }

fn insertionsort(list: &mut [i32]) {
    let length = list.len();
    for i in 1..length {
        let key = list[i];
        let mut j = i;

        while j > 0 && key < list[j-1] {
            list[j] = list[j-1];
            j -= 1;
        }
        list[j] = key;
    }
}
//-----
fn selectionsort(list: &mut [i32]) {
    let mut length = list.len();
    
    while length > 0 {
        let mut index_max = 0;
        for i in 1..length {
            if list[i] > list[index_max] {
                index_max = i;
            }
        }
        length -= 1;
        list.swap(index_max, length);
    }
}
//----
fn quicksort(list: &mut [i32]) {
    if list.len() <= 1 {
        return;
    }

    let pivot_index = partition(list);
    let (left, right) = list.split_at_mut(pivot_index);

    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition(list: &mut [i32]) -> usize {
    let pivot = list[list.len() - 1];
    let mut i = 0;

    for j in 0..list.len() - 1 {
        if list[j] <= pivot {
            list.swap(i,j);
            i += 1;
        }
    }
    list.swap(i, list.len()-1);
    i
}
//-----
fn mergesort(list: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mid = (left + right) / 2;

    mergesort(list, left, mid); // 0 3 / 0 1 
    mergesort(list, mid + 1, right);

    merge(list, left, mid, right);
}

fn merge(list: &mut [i32], left: usize, mid: usize, right: usize ) {
    let mut temp_list = [0; 10];
    let mut i = left;
    let mut j = mid + 1;
    let mut k = left;

    while i <= mid && j <= right {
        if list[i] <= list[j] {
            temp_list[k] = list[i];
            i += 1;
        } else {
            temp_list[k] = list[j];
            j += 1;
        }
        k += 1;
    }

    while i <= mid {
        temp_list[k] = list[i];
        i += 1;
        k += 1;
    }

    while j <= right {
        temp_list[k] = list[j];
        j += 1;
        k += 1;
    }

    for index in left..=right {
        list[index] = temp_list[index];
    }
}

fn shellsort(list: &mut [i32]) {
    let lenght = list.len();
    let mut gap = lenght / 2;

    while gap >0 {
        for i in gap..lenght {
            let temp = list[i];
            let mut j = i;

            while j >= gap && list[j - gap] > temp {
                list[j] = list[j - gap];
                j -= gap;
            }

            list[j] = temp;
        }
        gap /= 2;
    }
}

//  8, 5, 3, 7, 6, 2, 1, 4 => 
//4 4..8 t=l4 j=4 