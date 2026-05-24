pub fn bs_recursiv(arr: &[i32], target: i32, left: i32, right: i32) -> i32 {
    if left > right {
        return -1;
    }

    let mid = left + (right - left) / 2;

    if arr[mid as usize] == target {
        return mid;
    } else if arr[mid as usize] < target {
        return bs_recursiv(arr, target, mid + 1, right);
    } else {
        return bs_recursiv(arr, target, left, mid - 1);
    }
}

pub fn bs_iterativ(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid as usize] == target {
            return mid;
        } else if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return -1;
}

pub fn first_appear(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;
    let mut result = -1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if target == arr[mid as usize] {
            result = mid;
            right = mid - 1;
        } else if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return result;
}

pub fn last_appear(arr: &[i32], target: i32) -> i32 { 
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;
    let mut result = -1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if target == arr[mid as usize] {
            result = mid;
            left = mid + 1;
        } else if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return result;
}

pub fn insert_appear(arr: &[i32], target: i32) -> i32 { 
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;
    let mut result = arr.len() as i32;

    if target > arr[right as usize] {
        return result;
    }
    if target < arr[0] {
        return 0;
    }
    while left <= right {
        let mid = left + (right - left) / 2;

        if target == arr[mid as usize] {
            let mut i = mid as usize;
            while arr[i]==target {
                result = i as i32;
                i -= 1;
            }
            return result;
        } else if arr[mid as usize] < target {
            left = mid + 1;
            if arr[left as usize] >= target {
                return left;
            }
        } else {
            right = mid - 1;
            if arr[right as usize] < target {
                return mid;
            } else if arr[right as usize] == target {
                let mut i = right as usize;
                while arr[i]==target {
                result = i as i32;
                i -= 1;
            }
            return result;
            }
        }
    }
    return result;
}

// weigh int, days int, capacitate ca sa fie livrat totul in days  respectand ordinea
pub fn transporting(weighs: &[u32], days: u32) -> u32 {
    let mut right = weighs.iter().sum();
    let mut left = *weighs.iter().max().unwrap();
    let mut result = right;

    while left <= right {
        let mid = left + (right - left) / 2;
        if needed_days(weighs, mid) <= days {
            result = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return result;
}

fn needed_days(weighs: &[u32], capacity: u32) -> u32 {
    let mut step = 0;
    let mut i = 0;
    let mut tries:u32 = 1;
    while i < weighs.len() {
        if step + weighs[i as usize] > capacity {
            step = 0;
            tries += 1;
        }
        step += weighs[i as usize];
        i += 1;
    }
    return tries;
}