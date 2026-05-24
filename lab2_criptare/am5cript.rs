use std::io::{self, Write};

pub fn run_am5cript() {
    println!("\n--- AM5 Criptare ---");

    let mut text = String::new();
    print!("Introdu textul: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();
    let text = text.trim().as_bytes();

    let mut r18 = [0u8; 18];
    let mut r21 = [0u8; 21];
    let mut r22 = [0u8; 22];

    let mut output = Vec::new();

    for &byte in text {
        for bit_index in 0..8 {
            let bit = (byte >> (7 - bit_index)) & 1;

            // XOR pentru octetul de ieșire
            let out_bit = r18[17] ^ r21[20] ^ r22[21];

            // calculăm bitii noi conform regulilor tale
            let new_r18 = r18[17] ^ r18[16] ^ r18[15] ^ r18[12];
            let new_r21 = r21[20] ^ r21[19];
            let new_r22 = r22[21] ^ r22[20] ^ r22[19] ^ r22[6];

            // "majority clocking"
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

            // ieșirea criptată
            let enc_bit = bit ^ out_bit;

            // adăugăm în output
            if bit_index % 8 == 0 {
                output.push(0);
            }
            let last = output.len() - 1;
            output[last] = (output[last] << 1) | enc_bit;
        }
    }

    print!("Rezultat criptat (binar/hex): ");
    for b in &output {
        print!("{:02x} ", b);
    }
    println!();
}

fn shift_left(arr: &mut [u8], new_bit: u8) {
    for i in (1..arr.len()).rev() {
        arr[i] = arr[i - 1];
    }
    arr[0] = new_bit;
}
