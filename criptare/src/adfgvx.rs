// src/adfgvx.rs
use std::collections::HashSet;

const HEADERS: [char; 6] = ['A', 'D', 'F', 'G', 'V', 'X'];

fn clean_text(s: &str) -> String {
    s.to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn build_square(key: &str) -> Vec<Vec<char>> {
    let mut seen = HashSet::new();
    let mut symbols = Vec::new();

    for c in clean_text(key).chars() {
        if seen.insert(c) {
            symbols.push(c);
        }
    }
    for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars() {
        if seen.insert(c) {
            symbols.push(c);
        }
    }

    let mut square = Vec::new();
    for row in 0..6 {
        square.push(symbols[row * 6..(row + 1) * 6].to_vec()); 
    }
    square
}

fn coords(square: &Vec<Vec<char>>, ch: char) -> (char, char) {
    for (i, row) in square.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == ch {
                return (HEADERS[i], HEADERS[j]);
            }
        }
    }
    panic!("Simbol invalid: {}", ch);
}

fn substitute(square: &Vec<Vec<char>>, text: &str) -> String {
    let mut result = String::new();
    for ch in clean_text(text).chars() {
        let (r, c) = coords(square, ch);
        result.push(r);
        result.push(c);
    }
    result
}

fn reverse_substitute(square: &Vec<Vec<char>>, text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut result = String::new();

    for pair in chars.chunks(2) {
        if pair.len() == 2 {
            let row = HEADERS.iter().position(|&x| x == pair[0]).unwrap();
            let col = HEADERS.iter().position(|&x| x == pair[1]).unwrap();
            result.push(square[row][col]);
        }
    }
    result
}

fn transpose(cipher: &str, key: &str) -> String {
    let key_clean = clean_text(key);
    let cols = key_clean.len();
    if cols == 0 { return String::new(); }

    let mut table: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    for (i, ch) in cipher.chars().enumerate() {
        row.push(ch);
        if (i + 1) % cols == 0 {
            table.push(row.clone());
            row.clear();
        }
    }
    if !row.is_empty() {
        while row.len() < cols {
            row.push('X');
        }
        table.push(row);
    }

    let mut order: Vec<(usize, char)> = key_clean.chars().enumerate().collect();
    order.sort_by_key(|&(_, c)| c);

    let mut result = String::new();
    for (col_idx, _) in order {
        for r in &table {
            result.push(r[col_idx]);
        }
    }
    result
}

fn inverse_transpose(cipher: &str, key: &str) -> String {
    let key_clean = clean_text(key);
    let cols = key_clean.len();
    if cols == 0 { return String::new(); }

    let cipher_chars: Vec<char> = cipher.chars().collect();
    let total = cipher_chars.len();
    let rows = total / cols;

    let mut table: Vec<Vec<char>> = vec![vec![' '; cols]; rows];
    let mut order: Vec<(usize, char)> = key_clean.chars().enumerate().collect();
    order.sort_by_key(|&(_, c)| c);

    let mut idx = 0;
    for (col_idx, _) in &order {
        for r in 0..rows {
            table[r][*col_idx] = cipher_chars[idx];
            idx += 1;
        }
    }

    let mut result = String::new();
    for r in 0..rows {
        for c in 0..cols {
            result.push(table[r][c]);
        }
    }
    result
}

pub fn encrypt(square_key: &str, trans_key: &str, text: &str) -> String {
    let square = build_square(square_key);
    let substituted = substitute(&square, text);
    transpose(&substituted, trans_key)
}

pub fn decrypt(square_key: &str, trans_key: &str, cipher: &str) -> String {
    let square = build_square(square_key);
    let inv_trans = inverse_transpose(cipher, trans_key);
    reverse_substitute(&square, &inv_trans)
}
