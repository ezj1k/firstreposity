/// Transformă o literă A..Z într-un număr 0..25
fn char_to_num(c: char) -> u8 {
    (c as u8) - b'A'
}

/// Transformă un număr 0..25 într-o literă A..Z
fn num_to_char(n: u8) -> char {
    (b'A' + n) as char
}

/// Pregătește textul: majuscule + elimină caractere care nu sunt litere
fn clean_text(s: &str) -> String {
    s.to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}

/// Criptează cu Vigenère
pub fn encrypt(plaintext: &str, key: &str) -> String {
    let mut ciphertext = String::new();
    let key = clean_text(key);
    let key_bytes: Vec<u8> = key.chars().map(char_to_num).collect();

    for (i, ch) in clean_text(plaintext).chars().enumerate() {
        let p = char_to_num(ch);
        let k = key_bytes[i % key_bytes.len()];
        let c = (p + k) % 26;
        ciphertext.push(num_to_char(c));
    }
    ciphertext
}

/// Decriptează cu Vigenère
pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let mut plaintext = String::new();
    let key = clean_text(key);
    let key_bytes: Vec<u8> = key.chars().map(char_to_num).collect();

    for (i, ch) in ciphertext.chars().enumerate() {
        let c = char_to_num(ch);
        let k = key_bytes[i % key_bytes.len()];
        let p = (26 + c - k) % 26;
        plaintext.push(num_to_char(p));
    }
    plaintext
}
