// Merkle–Hellman Knapsack Cryptosystem implementation in Rust
// Includes: key generation, encryption, decryption, solving given task

use std::time::Instant;

// Extended Euclid algorithm to compute modular inverse
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (g, x, _) = egcd(a, m);
    if g != 1 { panic!("Nu exista invers modular"); }
    ((x % m) + m) % m
}

// Solve superincreasing knapsack
fn solve_super_knapsack(a: &Vec<i64>, mut s: i64) -> Vec<i64> {
    let n = a.len();
    let mut x = vec![0; n];
    for i in (0..n).rev() {
        if a[i] <= s {
            x[i] = 1;
            s -= a[i];
        }
    }
    x
}

// Encode character to 7-bit ASCII
fn char_to_bits(c: char) -> Vec<i64> {
    let mut v = vec![0; 7];
    let code = c as u8;
    for i in 0..7 {
        v[6 - i] = ((code >> i) & 1) as i64;
    }
    v
}

// Turn 7-bit vector into character
fn bits_to_char(b: &Vec<i64>) -> char {
    let mut val = 0u8;
    for i in 0..7 {
        val = (val << 1) | (b[i] as u8);
    }
    val as char
}

// Generate public key from private key (A, m, t, π)
fn generate_public_key(a: &Vec<i64>, m: i64, t: i64, perm: &Vec<usize>) -> Vec<i64> {
    let n = a.len();
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = (t * a[perm[i]] % m) as i64;
    }
    b
}

fn encrypt(message: &str, b: &Vec<i64>) -> Vec<i64> {
    let n = b.len();
    let mut cipher = vec![];
    let mut bits_message = vec![];

    for c in message.chars() {
        bits_message.extend(char_to_bits(c));
    }

    while bits_message.len() % n != 0 {
        bits_message.push(0);
    }

    for chunk in bits_message.chunks(n) {
        let mut sum = 0;
        for i in 0..n {
            sum += chunk[i] * b[i];
        }
        cipher.push(sum);
    }
    cipher
}

fn decrypt(cipher: &Vec<i64>, a: &Vec<i64>, m: i64, t: i64, perm: &Vec<usize>) -> String {
    let n = a.len();
    let s = mod_inverse(t, m);

    let mut bits = vec![];

    let mut inv_perm = vec![0; n];
    for i in 0..n { inv_perm[perm[i]] = i; }

    for y in cipher {
        let z = (s * y) % m;
        let mut x = solve_super_knapsack(a, z);

        let mut reordered = vec![0; n];
        for i in 0..n {
            reordered[inv_perm[i]] = x[i];
        }

        bits.extend(reordered);
    }

    let mut text = String::new();
    for chunk in bits.chunks(7) {
        text.push(bits_to_char(&chunk.to_vec()));
    }
    text
}

fn main() {
    use std::io::{self, Write};

    //==== Rezolvarea cerintei 1 ====//
    let m = 523;
    let t = 28;
    let b = vec![56,84,196,476,46,400,286];

    let s = mod_inverse(t, m);
    let mut a = vec![];
    for bi in &b {
        a.push((s * bi) % m);
    }
    println!("Vectorul super-crescator A = {:?}", a);

    let cipher = vec![566,774,864,928,686,538,1174,872,1042,672,606,586,1218,242];

    let mut message_bits = vec![];
    for y in &cipher {
        let z = (s * y) % m;
        message_bits.extend(solve_super_knapsack(&a, z));
    }
    println!("Bits = {:?}", message_bits);

    //==== Test complet cu mesaj propriu ====//
    println!("\nIntrodu mesajul pe care vrei sa-l criptezi (minim 3 propoziții):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let message = input.trim();

    let a_test = vec![2,3,7,25,80,160,300];
    let m_test = 700;
    let t_test = 37;
    let perm: Vec<usize> = (0..7).collect();

    let b_test = generate_public_key(&a_test, m_test, t_test, &perm);

    let start_encrypt = Instant::now();
    let encrypted = encrypt(message, &b_test);
    let time_encrypt = start_encrypt.elapsed();

    let start_decrypt = Instant::now();
    let decrypted = decrypt(&encrypted, &a_test, m_test, t_test, &perm);
    let time_decrypt = start_decrypt.elapsed();

    println!("\nCriptat = {:?}", encrypted);
    println!("Decriptat = {}", decrypted);
    println!("Timp criptare: {:?}", time_encrypt);
    println!("Timp decriptare: {:?}", time_decrypt);
}