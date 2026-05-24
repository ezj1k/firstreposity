use std::io::{self, Write};

fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut s: Vec<u8> = (0..=255).collect();
    let mut t: Vec<u8> = Vec::with_capacity(256);

    for i in 0..256 {
        t.push(key[i % key.len()]);
    }

    let mut j: usize = 0;
    for i in 0..256 {
        j = (j + s[i] as usize + t[i] as usize) % 256;
        s.swap(i, j);
    }

    let mut i: usize = 0;
    j = 0;
    let mut output = Vec::with_capacity(data.len());

    for &byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[(s[i] as usize + s[j] as usize) % 256];
        output.push(byte ^ k);
    }

    output
}

pub fn run_rc4decript() {
    println!("\n--- RC4 Decriptare ---");

    let mut key = String::new();
    let mut hex_input = String::new();

    print!("Introdu cheia: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).unwrap();

    print!("Introdu textul criptat (hex, ex: 1a 2b 3c ...): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hex_input).unwrap();

    let key = key.trim();
    let ciphertext: Vec<u8> = hex_input
        .split_whitespace()
        .filter_map(|h| u8::from_str_radix(h, 16).ok())
        .collect();

    let decrypted = rc4(key.as_bytes(), &ciphertext);
    println!("Text decriptat: {}", String::from_utf8_lossy(&decrypted));
}
