mod adfgvx; 
mod straddling_checkerboard;
mod vigenere;
mod playfair;

use std::io;

fn main() {
    println!("Alege algoritmul:"); 
    println!("1. ADFGVX");
    println!("2. Straddling Checkerboard");
    println!("3. Vigenere");
    println!("4. Playfair");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    println!("Alege acțiunea:");
    println!("1. Criptare");
    println!("2. Decriptare");

    let mut action = String::new();
    io::stdin().read_line(&mut action).unwrap();
    let action = action.trim();

    println!("Introdu textul:");
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();
    let text = text.trim();

    match choice {
        "1" => { // ADFGVX
            println!("Introdu cheia pentru pătratul Polybius:");
            let mut square_key = String::new();
            io::stdin().read_line(&mut square_key).unwrap();
            let square_key = square_key.trim();

            println!("Introdu cheia pentru transpoziție:");
            let mut trans_key = String::new();
            io::stdin().read_line(&mut trans_key).unwrap();
            let trans_key = trans_key.trim();

            if action == "1" {
                let enc = adfgvx::encrypt(square_key, trans_key, text);
                println!("Text criptat: {}", enc);
            } else {
                let dec = adfgvx::decrypt(square_key, trans_key, text);
                println!("Text decriptat: {}", dec);
            }
        }

        "2" => { // Straddling Checkerboard
            println!("Introdu cuvantul-cheie:");
            let mut key = String::new();
            io::stdin().read_line(&mut key).unwrap();
            let key = key.trim();

            println!("Introdu cheia pentru transpoziție:");
            let mut trans_key = String::new();
            io::stdin().read_line(&mut trans_key).unwrap();
            let trans_key = trans_key.trim();

            if action == "1" {
                let enc = straddling_checkerboard::encrypt(text, key, trans_key);
                println!("Text criptat: {}", enc);
            } else {
                let dec = straddling_checkerboard::decrypt(text, key, trans_key);
                println!("Text decriptat: {}", dec);
            }
        }

        "3" => { // Vigenere
            println!("Introdu cheia Vigenère:");
            let mut key = String::new();
            io::stdin().read_line(&mut key).unwrap();
            let key = key.trim();

            if action == "1" {
                let enc = vigenere::encrypt(text, key);
                println!("Text criptat: {}", enc);
            } else {
                let dec = vigenere::decrypt(text, key);
                println!("Text decriptat: {}", dec);
            }
        }

        "4" => { // Playfair
            println!("Introdu cheia Playfair:");
            let mut key = String::new();
            io::stdin().read_line(&mut key).unwrap();
            let key = key.trim();

            if action == "1" {
                let enc = playfair::encrypt(text, key);
                println!("Text criptat: {}", enc);
            } else {
                let dec = playfair::decrypt(text, key);
                println!("Text decriptat: {}", dec);
            }
        }

        _ => println!("Algoritm necunoscut"),
    }
}
