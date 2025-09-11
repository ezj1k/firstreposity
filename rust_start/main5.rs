fn main() {
    // && - and; || - or;
    let number = 10;
    let is_has_car = true;
    if number > 5 && is_has_car{
        println!("bigger than 5");
    } else if number == 4 && !is_has_car{
        println!("number is 4");
    } else {
        println!("else");
    }

    let condition: bool = true;
    let number = if condition {5} else {10};
    println!("{}", number);

    let number = 3;
    match number {
        1 => println!("res 1"),
        2 => println!("res 2"),
        3 => println!("res 3"),
        _ => println!("else"),
    }
}
