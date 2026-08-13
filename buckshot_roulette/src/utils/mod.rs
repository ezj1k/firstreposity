use std::io::{self, Write};

pub fn input() -> String {
    let mut s = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut s) 
        .expect("Error while reading");
    s.trim().to_string()
}