use std::collections::HashSet;

/// Construiește matricea Playfair (6x6) dintr-o cheie
fn build_table(key: &str) -> Vec<Vec<char>> {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut seen = HashSet::new();
    let mut table: Vec<char> = Vec::new();

    for ch in key.to_uppercase().chars() {
        if alphabet.contains(&ch) && !seen.contains(&ch) {
            seen.insert(ch);
            table.push(ch);
        }
    }

    for ch in &alphabet {
        if !seen.contains(ch) {
            table.push(*ch);
        }
    }

    table.chunks(6).map(|chunk| chunk.to_vec()).collect()
}

/// Găsește poziția unui caracter în matrice
fn find_pos(table: &Vec<Vec<char>>, ch: char) -> (usize, usize) {
    for (i, row) in table.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == ch) {
            return (i, j);
        }
    }
    panic!("Caracterul {} nu se află în tabel!", ch);
}

/// Pregătește textul pentru criptare (fără spații, cu X între dubluri, X la final dacă e impar)
fn prepare_text(text: &str) -> Vec<(char, char)> {
    let mut clean: Vec<char> = text
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    let mut pairs = Vec::new();
    let mut i = 0;
    while i < clean.len() {
        let a = clean[i];
        let b = if i + 1 < clean.len() { clean[i + 1] } else { 'X' };
        pairs.push((a, b));
        i += 2;
    }
    pairs
}

/// Aplică regulile Playfair pentru o pereche
fn encrypt_pair(pair: (char, char), table: &Vec<Vec<char>>) -> (char, char) {
    let (a, b) = pair;
    let (row_a, col_a) = find_pos(table, a);
    let (row_b, col_b) = find_pos(table, b);

    if row_a == row_b {
        let new_a = table[row_a][(col_a + 1) % 6];
        let new_b = table[row_b][(col_b + 1) % 6];
        (new_a, new_b)
    } else if col_a == col_b {
        let new_a = table[(row_a + 1) % 6][col_a];
        let new_b = table[(row_b + 1) % 6][col_b];
        (new_a, new_b)
    } else {
        let new_a = table[row_a][col_b];
        let new_b = table[row_b][col_a];
        (new_a, new_b)
    }
}

/// Funcție publică pentru criptare
pub fn encrypt(plaintext: &str, key: &str) -> String {
    let table = build_table(key);
    let pairs = prepare_text(plaintext);
    let mut result = String::new();
    for pair in pairs {
        let (a, b) = encrypt_pair(pair, &table);
        result.push(a);
        result.push(b);
    }
    result
}

/// Decriptare Playfair
fn decrypt_pair(pair: (char, char), table: &Vec<Vec<char>>) -> (char, char) {
    let (a, b) = pair;
    let (row_a, col_a) = find_pos(table, a);
    let (row_b, col_b) = find_pos(table, b);

    if row_a == row_b {
        let new_a = table[row_a][(col_a + 5) % 6]; // inversare stânga
        let new_b = table[row_b][(col_b + 5) % 6];
        (new_a, new_b)
    } else if col_a == col_b {
        let new_a = table[(row_a + 5) % 6][col_a]; // inversare sus
        let new_b = table[(row_b + 5) % 6][col_b];
        (new_a, new_b)
    } else {
        let new_a = table[row_a][col_b];
        let new_b = table[row_b][col_a];
        (new_a, new_b)
    }
}

/// Funcție publică pentru decriptare
pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let table = build_table(key);

    // Formăm perechi
    let chars: Vec<char> = ciphertext.chars().collect();
    let mut pairs = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let a = chars[i];
        let b = if i + 1 < chars.len() { chars[i + 1] } else { 'X' };
        pairs.push((a, b));
        i += 2;
    }

    // Decriptăm perechile
    let mut result = String::new();
    for pair in pairs {
        let (a, b) = decrypt_pair(pair, &table);
        result.push(a);
        result.push(b);
    }

    // Curățăm X-urile adăugate artificial
    let mut cleaned = String::new();
    let result_chars: Vec<char> = result.chars().collect();
    let mut i = 0;
    while i < result_chars.len() {
        if i + 2 < result_chars.len()
            && result_chars[i + 1] == 'X'
            && result_chars[i] == result_chars[i + 2]
        {
            cleaned.push(result_chars[i]);
            i += 2; // sărim peste X
        } else {
            cleaned.push(result_chars[i]);
            i += 1;
        }
    }

    // Dacă ultimul caracter este X adăugat artificial, îl ștergem
    if cleaned.ends_with('X') && result_chars.len() % 2 != 0 {
        cleaned.pop();
    }

    cleaned
}
