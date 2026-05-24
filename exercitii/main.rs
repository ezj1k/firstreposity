// use lab1::{bs_iterativ, bs_recursiv};
// use lab1::{first_appear, last_appear, insert_appear};
use lab1::transporting;

fn main() {
    // let mut arr1 = [5, 7, 3, 9, 1, -4, 6, 2, -6];
    // let sorted_arr1 = [-6, -4, 1, 2, 3, 5, 6, 7, 9];
    // let sorted_arr2 = [-10, 3, 3, 3, 8, 8, 10];
    let sorted_arr3 = [4, 2, 3, 1, 5, 6, 7, 8, 9, 10];
    
    // -------------------------
    // let rs1 = bs_iterativ(&sorted_arr1, 9);
    // let rs2  = bs_recursiv(&sorted_arr1, 9, 0, arr1.len() as i32);
    // println!("{rs1}, {rs2}");

    // --------------------------
    // let rs4  = first_appear(&sorted_arr2, 8);
    // let rs3 = last_appear(&sorted_arr2, 8);
    // println!("{rs4}, {rs3}");

    // --------------------------
    // let rs5 = insert_appear(&sorted_arr2, 8);
    // println!("{rs5}");

    // --------------------------
    // weigh int, days int, capacitate ca sa fie livrat totul in days  respectand ordinea
    let rs6 = transporting(&sorted_arr3, 5);
    println!("{rs6}");
}
