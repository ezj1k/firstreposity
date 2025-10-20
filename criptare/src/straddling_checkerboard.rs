use std::collections::{HashMap, HashSet};

/// Construiește tabela Straddling Checkerboard cu cuvânt-cheie
fn build_checkerboard_with_key(key: &str) -> HashMap<char, String> {
    let mut table = HashMap::new();
    let mut seen = HashSet::new();

    // Primul rând: literele/cifrele din cuvântul-cheie, fără duplicate
    let mut idx = 0;
    for ch in key.to_uppercase().chars() {
        if ch.is_ascii_alphanumeric() && !seen.contains(&ch) && idx < 10 {
            seen.insert(ch);
            table.insert(ch, format!("{:02}", idx)); // cod fix 2 cifre
            idx += 1;
        }
    }

    // Restul literelor și cifrelor, fără duplicate
    let rest = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars();
    let mut rest_iter = rest.filter(|c| !seen.contains(c));

    // Liniile speciale (1, 4, 6)
    let special_rows = [1, 4, 6];
    for &row in &special_rows {
        for col in 0..10 {
            if let Some(ch) = rest_iter.next() {
                table.insert(ch, format!("{}{}", row, col)); // tot cod de 2 cifre
            }
        }
    }

    table
}

/// Construiește tabela inversă pentru decriptare
fn build_inverse_checkerboard(table: &HashMap<char, String>) -> HashMap<String, char> {
    let mut inv = HashMap::new();
    for (ch, code) in table.iter() {
        inv.insert(code.clone(), *ch);
    }
    inv
}

/// Codifică textul în funcție de tabela checkerboard
fn encode_checkerboard(msg: &str, table: &HashMap<char, String>) -> String {
    let mut result = String::new();
    for ch in msg.chars() {
        if ch.is_whitespace() { continue; }
        let up = ch.to_ascii_uppercase();
        if let Some(code) = table.get(&up) {
            result.push_str(code);
        }
    }
    result
}

/// Decodează textul în funcție de tabela inversă
fn decode_checkerboard(msg: &str, inv_table: &HashMap<String, char>) -> String {
    let mut result = String::new();
    let chars: Vec<char> = msg.chars().collect();
    let mut i = 0;

    while i + 1 < chars.len() {
        let code = format!("{}{}", chars[i], chars[i + 1]);
        if let Some(&ch) = inv_table.get(&code) {
            result.push(ch);
        }
        i += 2;
    }

    result
}

/// Transpoziție coloane
fn columnar_transposition(text: &str, key: &str) -> String {
    let mut pairs: Vec<(usize, char)> = key.chars().enumerate().collect();
    pairs.sort_by_key(|&(_i, ch)| ch);

    let key_len = key.len();
    let rows = (text.len() + key_len - 1) / key_len;
    let mut grid = vec![vec![' '; key_len]; rows];

    for (i, ch) in text.chars().enumerate() {
        grid[i / key_len][i % key_len] = ch;
    }

    let mut result = String::new();
    for &(orig_idx, _ch) in &pairs {
        for r in 0..rows {
            result.push(grid[r][orig_idx]);
        }
    }
    result
}

/// Inversează transpoziția pe coloane
fn inverse_columnar_transposition(text: &str, key: &str) -> String {
    let key_len = key.len();
    let rows = (text.len() + key_len - 1) / key_len;
    let mut grid = vec![vec![' '; key_len]; rows];

    let mut pairs: Vec<(usize, char)> = key.chars().enumerate().collect();
    pairs.sort_by_key(|&(_i, ch)| ch);

    let mut text_chars = text.chars();
    for &(orig_idx, _ch) in &pairs {
        for r in 0..rows {
            if let Some(c) = text_chars.next() {
                grid[r][orig_idx] = c;
            }
        }
    }

    let mut result = String::new();
    for r in 0..rows {
        for c in 0..key_len {
            result.push(grid[r][c]);
        }
    }
    result
}

/// Funcție publică pentru criptare
pub fn encrypt(plaintext: &str, checker_key: &str, trans_key: &str) -> String {
    let table = build_checkerboard_with_key(checker_key);
    let encoded = encode_checkerboard(plaintext, &table);
    columnar_transposition(&encoded, trans_key)
}

/// Funcție publică pentru decriptare
pub fn decrypt(ciphertext: &str, checker_key: &str, trans_key: &str) -> String {
    let table = build_checkerboard_with_key(checker_key);
    let inv_table = build_inverse_checkerboard(&table);
    let inv_trans = inverse_columnar_transposition(ciphertext, trans_key);
    decode_checkerboard(&inv_trans, &inv_table)
}
