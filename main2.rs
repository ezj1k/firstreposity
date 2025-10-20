// //ex4
// //cwel mai mare din stanga, sau =.
// fn binarySearch4(nums: &[i32], target: i32) -> i32 {
//     if nums.is_empty() {
//         return nums.len() as i32;
//     }
    
//     let mut x = 0;
//     let mut left = 0;
//     let mut right = nums.len()-1;
//     let mut mid = (right+left)/2;
//     while left <= right {
//         if target > nums[mid] {
//             left = mid+1;
//             x = mid;
//         } else if target < nums[mid] {
//             //if mid == 0 { return nums.len() as i32; }
//             right = mid-1;
//             if target<nums[0] {
//                 return 0;
//             }
//         } else {
//             while mid > 0 && nums[mid] == nums[mid-1] {
//                 mid -= 1;
//             }
//             return mid as i32;
//         }
//         mid = (right+left)/2;
//     }
//     return nums.len() as i32;
// }

// fn binarySearch22(nums: &[i32], target: i32) -> i32 {
//     if nums.is_empty() {
//         return -1;
//     }
    
//     let mut left = 0;
//     let mut right = nums.len()-1;
//     let mut mid = (right+left)/2;
//     while left <= right {
//         if target > nums[mid] {
//             left = mid+1;
//         } else if target < nums[mid] {
//             if mid == 0 { return -1; }
//             right = mid-1;
//         } else {
//             while mid > 0 && nums[mid] == nums[mid-1] {
//                 mid -= 1;
//             }
//             return mid as i32;
//         }
//         mid = (right+left)/2;
//     }
//     return -1;
// }

// // n = 2 ^ k; k *C + 2^k-1; ...; k*C + 1; k = log 2 de n

// fn binarySearch3(nums: &[i32], target: i32) -> i32 {
//     if nums.is_empty() {
//         return -1;
//     }

//     let mut left = 0;
//     let mut right = nums.len()-1;
//     let mut mid = (right+left)/2;
//     while left <= right {
//         if target > nums[mid] {
//             left = mid+1;
//         } else if target < nums[mid] {
//             if mid == 0 { return -1; }
//             right = mid-1;
//         } else {
//             while mid + 1 < nums.len() && nums[mid] == nums[mid+1] {
//                 mid += 1;
//             }
//             return mid as i32;
//         }
//         mid = (right+left)/2;
//     }
//     return -1;
// }

// fn binarySearchIterativ(nums: &[i32], target: i32) -> i32 {
//     if nums.is_empty() {
//         return -1;
//     }
    
//     let mut left = 0;
//     let mut right = nums.len()-1;
//     let mut mid = (right+left)/2;
//     while left <= right {
//         if target > nums[mid] {
//             left = mid+1;
//         } else if target < nums[mid] {
//             if mid == 0 { return -1; }
//             right = mid-1;
//         } else {
            
//             return mid as i32;
//         }
//         mid = (right+left)/2;
//     }
//     return -1;
// }

// fn binarySearchRecursiv(nums: &[i32], target: i32, left: i32, right: i32) -> i32 {
//     if left>right {
//         return -1;
//     }
//     //let mut left = 0;
//     //let mut right = nums.len()-1;
//     let mut mid = (right+left)/2;

//     if target > nums[mid as usize] {
//         return binarySearchRecursiv(&nums, target, mid+1, right);
//     } else if target < nums[mid as usize] {
//         return binarySearchRecursiv(&nums, target, left,  mid-1);
//     } else {
//         return mid;
//     }
// }

fn main() {
    // //ex2
    // //a, 3 => 1;
    // let nums = [-10,3,3,3,8,8,10];
    // let target = 3;
    // println!("raspuns: {}", binarySearch22(&nums, target));

    // //b 8 => 4
    // let nums = [-10,3,3,3,8,8,10];
    // let target = 8;
    // println!("raspuns: {}", binarySearch22(&nums, target));

    // //c -1000 => -1
    // let nums = [-10,3,3,3,8,8,10];
    // let target = -1000;
    // println!("raspuns: {}", binarySearch22(&nums, target));

    // //ex3
    // //a 3 => 3
    // let nums = [-10,3,3,3,8,8,10];
    // let target = 3;
    // println!("raspuns: {}", binarySearch3(&nums, target));

    // //b 8 => 5
    // let nums = [-10,3,3,3,8,8,10];
    // let target = 8;
    // println!("raspuns: {}", binarySearch3(&nums, target));

    // //c 2025 => -1
    // let nums = [-10,3,3,3,8,8,10];
    // let target = 2025;
    // println!("raspuns: {}", binarySearch3(&nums, target));

    // //ex1
    // let nums = [-10,3,3,3,8,8,10];
    // let target = 8;
    // let left=0;
    // let right = (nums.len() -1) as i32;
    // println!("raspuns: {:?}", binarySearchIterativ(&nums, target));
    // println!("raspuns: {:?}", binarySearchRecursiv(&nums, target, left, right));

    //ex4
    // let nums = [-10,3,3,3,8,8,10];
    // let target = -1000;
    // //let left=0;
    // //let right = (nums.len() -1) as i32;
    // println!("raspuns: {:?}", binarySearch4(&nums, target));
}

