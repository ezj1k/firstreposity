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

pub fn run_rc4cript() {
    println!("\n--- RC4 Criptare ---");

    let mut key = String::new();
    let mut plaintext = String::new();

    print!("Introdu cheia (text): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).unwrap();

    print!("Introdu textul de criptat: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut plaintext).unwrap();

    let key = key.trim();
    let plaintext = plaintext.trim().as_bytes();

    let encrypted = rc4(key.as_bytes(), plaintext);

    // Afișăm textul criptat (poate conține caractere neafișabile)
    println!(
        "\nText criptat (text brut): {}",
        String::from_utf8_lossy(&encrypted)
    );

    // Afișăm și în hex pentru siguranță (opțional)
    print!("Text criptat (hex): ");
    for b in &encrypted {
        print!("{:02x} ", b);
    }
    println!();
}
