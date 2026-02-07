use std::{fs, io};

fn main() {
    //let option = find_element(vec![1,2,34,6],3).unwrap();

    let file_name = "output.txt"
    // match write_to_file(file_name, "Hello, Mir") {
    //     Ok(()) => println!("its ok"),
    //     Err(e) => println!("its not ok: {}", e),
    // }

    match read_to_file(file_name) {
        Ok(val) => println!("{}", val),
        Err(e) => println!("its not ok: {}", e),
    }
}

// fn divide(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         Err(String::from("Error"))
//     } else {
//         Ok(a / b)
//     }
// }

// fn find_element(vec: Vec<i32>, value: i32) -> Option<usize> {
//     vec.iter().position(|&x| x == value)
// }

// fn write_to_file(file_path: &str, content: &str) -> Result<(), io::Error> {
//     fs::write(file_path, content)
// }

fn read_to_file(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}