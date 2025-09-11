fn main() {
    for i in (1..=40).step_by(3) { //1..4 - 1,2,3;  1..=4 - 1,2,3,4; (1..4).rev() - 3,2,1; .step_by(nr) - step;
        println!("Number {}", i);
    }

    let mut number = 3;
    while number > 0 {
        println!("Number: {}", number);
        number -= 1;
    }
    println!("Finish");

    for i in 1..=20 {
        if i % 2 == 0 {
            continue;
        }

        println!("Number {}", i);
        
        if i == 7 {
            break;
        }
    }

    let mut count = 0;

    loop { // infinite while
        count += 1;
        println!("Number {}", count);
        if count == 5 {
            break;
        }
    }

    let array = [10, 20, 30, 40];
    for el in array {
        println!("El: {}", el);
    }
}
