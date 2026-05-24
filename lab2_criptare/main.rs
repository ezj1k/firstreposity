mod am5cript;
mod am5decript;
mod rc4cript;
mod rc4decript;

use std::io::{self, Write};

fn main() {
    println!("=== Sistem de criptare ===");
    println!("1. RC4 - Criptare");
    println!("2. RC4 - Decriptare");
    println!("3. AM5 - Criptare");
    println!("4. AM5 - Decriptare");

    print!("Alege opțiunea (1-4): ");
    io::stdout().flush().unwrap();

    let mut opt = String::new();
    io::stdin().read_line(&mut opt).unwrap();

    match opt.trim() {
        "1" => rc4cript::run_rc4cript(),
        "2" => rc4decript::run_rc4decript(),
        "3" => am5cript::run_am5cript(),
        "4" => am5decript::run_am5decript(),
        _ => println!("Opțiune invalidă!"),
    }
}
