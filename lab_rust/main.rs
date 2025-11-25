use std::io;

mod main1;
mod main2;
mod main3;
mod main4;

fn introdu() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

enum Choice {
    Main1,
    Main2,
    Main3,
    Main4,
}

fn introdu_infinit() -> Choice {
    println!("Introdu numarul al laboratorului care vreai sa se execute: ");

    loop {
        let variant = introdu();
        match variant {
            1 => break Choice::Main1,
            2 => break Choice::Main2,
            3 => break Choice::Main3,
            4 => break Choice::Main4,
            _ => continue,
        }
    }
}

fn main() {
    let variant = introdu_infinit();
    match variant {
        Choice::Main1 => main1::main(),
        Choice::Main2 => main2::main(),
        Choice::Main3 => main3::main(),
        Choice::Main4 => main4::main(),
    }
}
