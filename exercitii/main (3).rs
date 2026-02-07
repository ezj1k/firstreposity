fn binary(x: Vec<i32>, target: i32, i: i32) {
    let mut left = 0;
    let mut right = x.len()-1;
    while left <= right {
        let mid = (left+right)/2;
        if target > x[mid as usize] {
            left = mid+1;
        } else if target < x[mid as usize] {
            right = mid-1;
        } else {
            println!{"target e pe indexul {} {}", i, mid};
            return;
        }
    }
    println!{"target e pe indexul -1 -1"};
}

fn main() {
    let tab = [[-1,2,3,4], [5,8,9,34], [43,56,87,555]];
    let n = tab.len()-1;
    let m = tab[tab.len()-1].len()-1;
    let target = 87;
    let mut vec1: Vec<i32> = Vec::new();
    let mut index = 0;

    for i in 0..n {
        if target > tab[i][0] && target < tab[i][m] {
            index = i;
            for j in 0..=m {
                vec1.push(tab[i][j]);
            }
            break;
        } else if target == tab[i][0] {
            println!{"target e pe indexul {} 0", i};
            break;
        } else if target == tab[i][m] {
            println!{"target e pe indexul {} {}", i,m};
            break;
        } else if target < tab[0][0] {
            println!{"target e pe indexul -1 -1"};
            break;
        } else if target > tab[n][m] {
            println!{"target e pe indexul -1 -1"};
            break;
        }
    }
    binary(vec1, target, index as i32);
}
