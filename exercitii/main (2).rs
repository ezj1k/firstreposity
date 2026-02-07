use std::collections::HashMap;

/// Construim tabla Straddling Checkerboard
fn build_checkerboard() -> HashMap<char, String> {
    let mut table = HashMap::new();

    // Linia 0 -> litere frecvente (un singur digit)
    let freq = ['8', '9', 'I', 'S', 'P', 'R', 'M', 'E', 'A', 'B'];
    for (i, &ch) in freq.iter().enumerate() {
        table.insert(ch, format!("{}", i)); // cod 0..9
    }

    // Liniile speciale (1, 4, 6)
    let mut rest = "CDFGHJKLNOQTUVWXYZ01234567".chars();
    let special_rows = [1, 4, 6];
    for &row in &special_rows {
        for col in 0..10 {
            if let Some(ch) = rest.next() {// daca in coloana data este un caracter
                table.insert(ch, format!("{}{}", row, col));
            }
        }
    }

    table
}

/// Codificăm textul cu checkerboard
fn encode_checkerboard(msg: &str, table: &HashMap<char, String>) -> String {
    let mut result = String::new();
    for ch in msg.chars() {
        if ch.is_whitespace() {
            continue;
        }
        let up = ch.to_ascii_uppercase(); // in majuscula
        if let Some(code) = table.get(&up) {
            result.push_str(code);
        } else {
            println!("Caracter necunoscut: {}", ch);
        }
    }
    result
}

/// Transpoziția coloanelor după cheia MURPHY
fn columnar_transposition(text: &str, key: &str) -> String {
    let mut pairs: Vec<(usize, char)> = key.chars().enumerate().collect();
    // sortăm literele cheii alfabetic, dar păstrăm indicele original
    pairs.sort_by_key(|&(_i, ch)| ch);

    let key_len = key.len();
    let rows = (text.len() + key_len - 1) / key_len;

    // umplem un tabel row x key_len
    let mut grid = vec![vec![' '; key_len]; rows];
    for (i, ch) in text.chars().enumerate() {
        grid[i / key_len][i % key_len] = ch;
    }

    // citim coloanele în ordinea cheii sortate
    let mut result = String::new();
    for &(orig_idx, _ch) in &pairs {
        for r in 0..rows {
            result.push(grid[r][orig_idx]);
        }
    }
    result
}

fn main() {
    let table = build_checkerboard();

    let plaintext = "THE FIRST FEW PRIMES ARE 2 3 5 AND 7";
    let encoded = encode_checkerboard(plaintext, &table);

    println!("Codificat checkerboard: {}", encoded);

    let key = "MURPHY";
    let transposed = columnar_transposition(&encoded, key);

    println!("Mesaj final criptat: {}", transposed);
}
