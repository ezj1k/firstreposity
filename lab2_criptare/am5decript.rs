use std::io::{self, Write};

pub fn run_am5decript() {
    println!("\n--- AM5 Decriptare ---");

    let mut hex_input = String::new();
    print!("Introdu textul criptat (hex): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hex_input).unwrap();

    let ciphertext: Vec<u8> = hex_input
        .split_whitespace()
        .filter_map(|h| u8::from_str_radix(h, 16).ok())
        .collect();

    let mut r18 = [0u8; 18];
    let mut r21 = [0u8; 21];
    let mut r22 = [0u8; 22];

    let mut output = Vec::new();

    for &byte in &ciphertext {
        let mut plain_byte = 0;
        for bit_index in 0..8 {
            let bit = (byte >> (7 - bit_index)) & 1;
            let out_bit = r18[17] ^ r21[20] ^ r22[21];
            let dec_bit = bit ^ out_bit;

            let new_r18 = r18[17] ^ r18[16] ^ r18[15] ^ r18[12];
            let new_r21 = r21[20] ^ r21[19];
            let new_r22 = r22[21] ^ r22[20] ^ r22[19] ^ r22[6];

            let clock_bit = ((r18[7] + r21[9] + r22[9]) >= 2) as u8;
            if r18[7] == clock_bit {
                shift_left(&mut r18, new_r18);
            }
            if r21[9] == clock_bit {
                shift_left(&mut r21, new_r21);
            }
            if r22[9] == clock_bit {
                shift_left(&mut r22, new_r22);
            }

            plain_byte = (plain_byte << 1) | dec_bit;
        }
        output.push(plain_byte);
    }

    println!("Text decriptat: {}", String::from_utf8_lossy(&output));
}

fn shift_left(arr: &mut [u8], new_bit: u8) {
    for i in (1..arr.len()).rev() {
        arr[i] = arr[i - 1];
    }
    arr[0] = new_bit;
}
