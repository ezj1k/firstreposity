use std::io;

fn main() {
    //memory
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter num1:");
    io::stdin()
        .read_line(&mut num1)
        .expect("error: to read info");

    println!("Enter num2:");
    io::stdin()
        .read_line(&mut num2)
        .expect("error: to read info");

    let data1: i16 = num1.trim().parse().expect("pls enter valid number");
    // trim - exclude spaces; parse - change type which u wrote(i16);
    let data2: u8 = num2.trim().parse().expect("pls enter valid number");

    let result = data1 + data2 as i16;
    println!("Result: {}", result);
}
