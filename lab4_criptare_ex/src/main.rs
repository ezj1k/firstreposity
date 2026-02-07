use num_bigint::{BigInt, BigUint};
// ACUM IMPORTĂM TOATE CELE NECESARE INCLUSIV ToPrimitive PENTRU to_u64()
use num_traits::{One, ToPrimitive, Zero};
use std::io::{self, Write};

// --- CONSTANTE PENTRU CODIFICARE ---
// Folosim 27 de simboluri: ' ' (spațiu) și A-Z.
// Alfabetul: ' ' (Spațiu) = 0, 'A' = 1, 'B' = 2, ..., 'Z' = 26.
const BASE: u32 = 27;

// =========================================================================
// FUNCTII AJUTATOARE PENTRU NUMERE MARI
// =========================================================================

/// Citește un număr BigInt de la utilizator.
fn read_bigint(prompt: &str) -> BigInt {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<BigInt>() {
            Ok(n) => return n,
            Err(_) => println!("Eroare: Vă rugăm introduceți un număr întreg valid."),
        }
    }
}

/// Implementarea Exponentierii Modulare Rapide: (base^exponent) mod modulus.
/// Aceasta este funcția centrală de criptare/decriptare.
fn modular_exponentiation(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    if modulus == &BigUint::zero() {
        panic!("Modulul nu poate fi zero.");
    }
    if exponent == &BigUint::zero() {
        return BigUint::one();
    }

    let mut res = BigUint::one();
    let mut b = base % modulus;
    let mut e = exponent.clone();

    while e > BigUint::zero() {
        // Dacă bitul curent este 1, înmulțim rezultatul
        if &e & BigUint::one() == BigUint::one() {
            res = (res * &b) % modulus;
        }
        // Ridicăm baza la pătrat
        b = (&b * &b) % modulus;
        // Trecem la următorul bit
        e = e >> 1;
    }
    res
}

/// Algoritmul lui Euclid Extins (pentru a găsi d = e^-1 mod phi_n).
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        return (b.clone(), BigInt::zero(), BigInt::one());
    }
    let (g, x1, y1) = extended_gcd(&(b % a), a);
    let x = &y1 - (b / a) * &x1;
    let y = x1;
    (g, x, y)
}

/// Găsește inversul modular 'd' al lui 'e' modulo 'phi_n'.
fn find_d(e: &BigInt, phi_n: &BigInt) -> Option<BigInt> {
    let (g, mut x, _) = extended_gcd(e, phi_n);
    if g != BigInt::one() {
        return None; // Nu există invers modular (e și phi_n nu sunt relativ prime)
    }
    // Asigurăm că d este pozitiv: d = x mod phi_n
    while x < BigInt::zero() {
        x += phi_n;
    }
    Some(x % phi_n)
}

// =========================================================================
// FUNCTII DE CODIFICARE/DECODIFICARE TEXT
// =========================================================================

/// Convertește un caracter la valoarea sa numerică (0-26).
fn char_to_value(c: char) -> Option<u8> {
    let upper = c.to_ascii_uppercase();
    if upper == ' ' {
        Some(0)
    } else if upper.is_ascii_alphabetic() {
        // 'A' -> 1, 'B' -> 2, ...
        Some(upper as u8 - 'A' as u8 + 1)
    } else {
        None // Ignoră alte caractere
    }
}

/// Convertește o valoare numerică (0-26) la caracterul său.
fn value_to_char(val: u8) -> char {
    match val {
        0 => ' ',
        1..=26 => (('A' as u8) + val - 1) as char,
        _ => '?', // Caracter invalid
    }
}

/// Convertește textul clar în blocuri numerice (M).
/// De asemenea, determină automat dimensiunea blocului k, astfel încât BASE^k < N.
fn text_to_blocks(text: &str, n_modulus: &BigUint) -> (Vec<BigUint>, usize) {
    let base = BigUint::from(BASE);

    // 1. Determină lungimea blocului (k)
    // Căutăm k maxim astfel încât BASE^k < N
    let mut block_len = 0;
    let mut max_val = BigUint::one();

    // Cât timp baza ridicată la o putere mai mare este mai mică decât N, mărim blocul.
    while max_val.clone() * &base < *n_modulus {
        max_val *= &base;
        block_len += 1;
    }

    if block_len == 0 {
        println!(
            "\n! Eroare: Modulul N ({}) este prea mic. Nu se poate cripta nici un caracter (B={}).",
            n_modulus, BASE
        );
        return (Vec::new(), 0);
    }

    println!(
        "\n! Dimensiunea blocului de caractere determinată: {} caractere ({}).",
        block_len,
        base.pow(block_len as u32)
    );

    let mut blocks: Vec<BigUint> = Vec::new();
    let filtered_text: Vec<u8> = text.chars().filter_map(char_to_value).collect();

    // 2. Împarte textul filtrat în blocuri și le convertește în numere
    for chunk in filtered_text.chunks(block_len) {
        let mut block_value = BigUint::zero();
        let mut power_of_base = BigUint::one();

        // Conversia din baza B (27) la număr întreg M:
        // M = c[0]*B^(k-1) + c[1]*B^(k-2) + ... + c[k-1]*B^0
        // Parcurgem de la sfârșit pentru o implementare mai eficientă
        for &val in chunk.iter().rev() {
            let val_bigint = BigUint::from(val as u32);
            block_value += val_bigint * &power_of_base;
            power_of_base *= &base;
        }

        blocks.push(block_value);
    }

    (blocks, block_len)
}

/// Convertește blocurile numerice (M) înapoi în text clar.
fn blocks_to_text(blocks: Vec<BigUint>, block_len: usize) -> String {
    let base = BigUint::from(BASE);
    let mut result = String::new();

    for block_val in blocks {
        let mut temp_val = block_val;
        let mut block_chars = Vec::new();

        // 1. Conversia din număr întreg M înapoi în baza B (27)
        for _ in 0..block_len {
            // Verificăm dacă nu mai avem nimic de adăugat în cazul în care M era 0.
            if temp_val.is_zero() && block_chars.len() >= block_len {
                break;
            }

            // M = q * B + r. r este valoarea caracterului
            let remainder = &temp_val % &base;
            // to_u64() este acum vizibil datorită importului ToPrimitive.
            let val_u8 = remainder.to_u64().unwrap_or(0) as u8;
            block_chars.push(value_to_char(val_u8));
            temp_val /= &base;
        }

        // 2. Caracterele sunt obținute în ordine inversă, deci le inversăm
        block_chars.reverse();

        // 3. Adăugăm la rezultat
        let trimmed_chars: String = block_chars.into_iter().collect();
        result.push_str(&trimmed_chars);
    }

    // Eliminăm spațiile de padding (00-uri) de la sfârșitul textului
    result.trim_end().to_string()
}

// =========================================================================
// LOGICA APLICATIEI
// =========================================================================

/// Funcția principală de Criptare.
fn run_encryption() {
    println!("\n--- CRIPTARE RSA ---");
    let n_bigint = read_bigint("Introduceți modulul public N: ")
        .to_biguint()
        .unwrap();
    let e_bigint = read_bigint("Introduceți exponentul public E: ")
        .to_biguint()
        .unwrap();

    print!("Introduceți textul T (caractere permise A-Z și spațiu): ");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();

    // 1. Convertire Text la Blocuri Numerice (M)
    let (message_blocks, block_len) = text_to_blocks(text.trim(), &n_bigint);

    if message_blocks.is_empty() {
        return;
    }

    // 2. Criptare: C = M^e mod N
    let mut ciphertext_blocks: Vec<BigUint> = Vec::new();
    println!("\nBlocuri Cifrate (C = M^e mod N):");
    for m in message_blocks.iter() {
        let c = modular_exponentiation(m, &e_bigint, &n_bigint);
        println!("M: {} -> C: {}", m, c);
        ciphertext_blocks.push(c);
    }

    // 3. Afișare rezultate
    let ciphertext_string = ciphertext_blocks
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("\n--- REZULTAT CRIPTARE ---");
    println!("Cheie Publică (N, E): ({}, {})", n_bigint, e_bigint);
    println!("Lungime bloc: {} caractere", block_len);
    println!("Text Cifrat (C): {}", ciphertext_string);
    println!("\n(Salvați N, D (privat) și textul cifrat pentru decriptare.)");
}

/// Funcția principală de Decriptare.
fn run_decryption() {
    println!("\n--- DECRIPTARE RSA ---");
    let n_bigint = read_bigint("Introduceți modulul N: ");
    let n_biguint = n_bigint.to_biguint().unwrap();
    let d_bigint: BigInt;

    println!("Aveți cheia privată D sau doriți să o calculați?");
    println!("1. Voi introduce cheia privată D direct.");
    println!("2. Voi calcula D pe baza lui P, Q și E.");
    print!("Alegeți opțiunea (1/2): ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            d_bigint = read_bigint("Introduceți exponentul privat D: ");
        }
        "2" => {
            let p = read_bigint("Introduceți numărul prim P: ");
            let q = read_bigint("Introduceți numărul prim Q: ");
            let e = read_bigint("Introduceți exponentul public E: ");

            if p.to_biguint().unwrap() * q.to_biguint().unwrap() != n_biguint {
                println!("! AVERTISMENT: P * Q nu este egal cu N. Calculele pot fi incorecte.");
            }

            // Calculăm phi(n) = (P-1)(Q-1)
            let phi_n = (&p - BigInt::one()) * (&q - BigInt::one());

            match find_d(&e, &phi_n) {
                Some(d) => {
                    d_bigint = d;
                    println!("! Cheia privată D a fost calculată: {}", d_bigint);
                }
                None => {
                    println!("! Eroare: Nu s-a putut calcula D (e și phi(n) nu sunt relativ prime). Oprirea.");
                    return;
                }
            }
        }
        _ => {
            println!("Opțiune invalidă. Oprirea.");
            return;
        }
    }

    let d_biguint = d_bigint.to_biguint().unwrap();

    print!("Introduceți blocurile cifrate C separate prin spațiu (ex: 123 456 789): ");
    io::stdout().flush().unwrap();
    let mut ciphertext_input = String::new();
    io::stdin().read_line(&mut ciphertext_input).unwrap();

    // 1. Conversia șirului de text cifrat în blocuri numerice (C)
    let ciphertext_blocks: Vec<BigUint> = ciphertext_input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<BigUint>().ok())
        .collect();

    if ciphertext_blocks.is_empty() {
        println!("Eroare: Nu s-au putut citi blocurile cifrate.");
        return;
    }

    // 2. Re-determinarea lungimii blocului original
    let base = BigUint::from(BASE);
    let mut block_len = 0;
    let mut max_val = BigUint::one();
    while max_val.clone() * &base < n_biguint {
        max_val *= &base;
        block_len += 1;
    }

    if block_len == 0 {
        println!("! Eroare: Modulul N este prea mic pentru a decripta. Oprirea.");
        return;
    }
    println!(
        "\n! Dimensiunea blocului de caractere re-determinată: {} caractere.",
        block_len
    );

    // 3. Decriptare: M = C^d mod N
    let mut decrypted_blocks: Vec<BigUint> = Vec::new();
    println!("\nBlocuri Decriptate (M = C^d mod N):");
    for c in ciphertext_blocks.iter() {
        let m = modular_exponentiation(c, &d_biguint, &n_biguint);
        println!("C: {} -> M: {}", c, m);
        decrypted_blocks.push(m);
    }

    // 4. Conversia blocurilor numerice (M) înapoi în Text
    let decrypted_text = blocks_to_text(decrypted_blocks, block_len);

    println!("\n--- REZULTAT DECRIPTARE ---");
    println!("Cheie Privată (N, D): ({}, {})", n_biguint, d_bigint);
    println!("Text Decriptat (T): {}", decrypted_text);
}

/// Funcția principală care afișează meniul.
fn main() {
    println!("Algoritmul RSA: Criptare și Decriptare (Implementare Simplă pe Blocuri)");
    println!("======================================================================");
    println!("Sistem de codificare: Spațiu=0, A=1, ..., Z=26 (Baza B=27)");

    loop {
        println!("\nAlegeți operațiunea:");
        println!("1. Criptare (T -> C)");
        println!("2. Decriptare (C -> T)");
        println!("3. Ieșire");
        print!("Opțiunea: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => run_encryption(),
            "2" => run_decryption(),
            "3" => {
                println!("Ieșire din program.");
                break;
            }
            _ => println!("Opțiune invalidă. Vă rugăm introduceți 1, 2 sau 3."),
        }
    }
}
