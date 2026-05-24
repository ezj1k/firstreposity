// Am eliminat 'use std::collections::HashMap;' deoarece nu era folosit

// --- 1. Tabele DES (CONSTANTE NECESARE) ---

// Permutarea Cheii 1 (PC-1) - 64 biți la 56 biți
const PC_1: [usize; 56] = [
    57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60,
    52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29,
    21, 13, 5, 28, 20, 12, 4,
];

// Permutarea Cheii 2 (PC-2) - 56 biți la 48 biți
const PC_2: [usize; 48] = [
    14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41, 52,
    31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32,
];

// Expansiunea (E) - 32 biți la 48 biți
const E: [usize; 48] = [
    32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17, 16, 17, 18,
    19, 20, 21, 20, 21, 22, 23, 24, 25, 24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1,
];

// Permutarea P (P) - 32 biți la 32 biți
const P: [usize; 32] = [
    16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10, 2, 8, 24, 14, 32, 27, 3, 9, 19,
    13, 30, 6, 22, 11, 4, 25,
];

// Rotațiile pentru generarea subcheilor
const ROTATIONS: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

// S-Box-urile (8 cutii, fiecare 6 biți -> 4 biți)
const S_BOXES: [[[u8; 16]; 4]; 8] = [
    // S1
    [
        [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
        [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
        [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
        [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
    ],
    // S2
    [
        [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
        [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
        [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
        [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
    ],
    // S3
    [
        [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
        [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
        [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
        [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
    ],
    // S4
    [
        [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
        [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
        [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
        [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
    ],
    // S5
    [
        [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
        [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
        [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
        [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
    ],
    // S6
    [
        [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
        [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
        [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
        [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
    ],
    // S7
    [
        [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
        [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
        [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
        [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
    ],
    // S8
    [
        [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
        [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
        [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
        [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
    ],
];

// --- 2. Funcții Utilitare (COMPLETE) ---

/// Permută biții unui bloc de 64 de biți (u64) conform unei tabele.
fn permute(data: u64, table: &[usize]) -> u64 {
    let mut result: u64 = 0;
    for (i, &bit_index) in table.iter().enumerate() {
        let src_pos = 64 - bit_index;
        let dest_pos = table.len() - 1 - i;

        if (data >> src_pos) & 1 == 1 {
            result |= 1 << dest_pos;
        }
    }
    result
}

/// Permută biții unui bloc de 56 de biți conform PC-2.
fn permute_key(data: u64, table: &[usize]) -> u64 {
    let mut result: u64 = 0;
    let total_bits = table.len();
    for (i, &bit_index) in table.iter().enumerate() {
        let src_pos = 56 - bit_index;
        let dest_pos = total_bits - 1 - i;

        if (data >> src_pos) & 1 == 1 {
            result |= 1 << dest_pos;
        }
    }
    result
}

/// Citește o cheie sau un text din format hexazecimal.
fn hex_to_u64(hex: &str) -> Result<u64, &'static str> {
    u64::from_str_radix(hex, 16).map_err(|_| "Format hexazecimal invalid.")
}

/// Convertește un u64 la format hexazecimal.
fn u64_to_hex(data: u64) -> String {
    format!("{:016X}", data)
}

/// Rotește circular biții unui u32 la stânga (pentru 28 de biți).
fn rotate_left(val: u32, shift: u8) -> u32 {
    (val << shift) | (val >> (28 - shift))
}

// --- 3. Generarea Subcheilor (COMPLETĂ) ---

/// Generează cele 16 subchei (48 de biți fiecare).
fn generate_subkeys(key: u64) -> [u64; 16] {
    let key_56 = permute(key, &PC_1);

    let mask_28: u64 = (1 << 28) - 1;
    let mut c = ((key_56 >> 28) & mask_28) as u32;
    let mut d = (key_56 & mask_28) as u32;

    let mut subkeys = [0u64; 16];

    for i in 0..16 {
        c = rotate_left(c, ROTATIONS[i]);
        d = rotate_left(d, ROTATIONS[i]);

        let key_cd_56: u64 = ((c as u64) << 28) | (d as u64);

        subkeys[i] = permute_key(key_cd_56, &PC_2);
    }
    subkeys
}

// --- 4. Funcția Feistel (F) (COMPLETĂ) ---

/// Implementează funcția F (Expansiune, XOR, S-Box, Permutare P).
fn feistel_function(r: u32, subkey: u64) -> u32 {
    // 1. Expansiune R (32 biți -> 48 biți).
    let mut expanded_r: u64 = 0;
    for (i, &bit_index) in E.iter().enumerate() {
        let src_pos = 32 - bit_index;
        let dest_pos = 48 - 1 - i;

        if ((r as u64) >> src_pos) & 1 == 1 {
            expanded_r |= 1 << dest_pos;
        }
    }

    // 2. XOR cu subcheia (48 biți)
    let xored = expanded_r ^ subkey;

    // 3. S-Box Substitution (48 biți -> 32 biți)
    let mut s_box_output: u32 = 0;
    for i in 0..8 {
        let block_6_bits = (xored >> (48 - 6 * (i + 1))) & 0b111111;

        let row = ((block_6_bits & 0b100000) >> 4) | (block_6_bits & 0b000001);
        let col = (block_6_bits >> 1) & 0b1111;

        let s_val = S_BOXES[i][row as usize][col as usize];

        s_box_output |= (s_val as u32) << (32 - 4 * (i + 1));
    }

    // 4. Permutarea P (32 biți -> 32 biți)
    let mut p_output: u32 = 0;
    for (i, &bit_index) in P.iter().enumerate() {
        let src_pos = 32 - bit_index;
        let dest_pos = 32 - 1 - i;

        if (s_box_output >> src_pos) & 1 == 1 {
            p_output |= 1 << dest_pos;
        }
    }

    p_output
}

// --- 5. Criptare/Decriptare DES (Bloc) (PLACEHOLDER, fără avertismente) ---

// Am redenumit variabilele cu '_' pentru a ignora avertismentele de "unused variable"
#[allow(dead_code)]
fn des_block(data: u64, _subkeys: &[u64; 16], _is_encrypt: bool) -> u64 {
    data // placeholder
}

// --- 6. Logica de Testare a S-Box-urilor ---

/// Testează proprietatea de "Avalanșă Strică" (Strict Avalanche Criterion - SAC) a unui S-Box.
fn test_s_box_sac(s_box_index: usize) -> f64 {
    let mut flip_counts = [0; 4]; // 4 biți de ieșire
    let s_box = S_BOXES[s_box_index];
    let num_inputs = 64; // 2^6 posibilități de intrare de 6 biți

    for input in 0..num_inputs {
        for bit_pos in 0..6 {
            let flipped_input = input ^ (1 << bit_pos);

            let row_orig = ((input & 0b100000) >> 4) | (input & 0b000001);
            let col_orig = (input >> 1) & 0b1111;
            let output_orig = s_box[row_orig as usize][col_orig as usize];

            let row_flip = ((flipped_input & 0b100000) >> 4) | (flipped_input & 0b000001);
            let col_flip = (flipped_input >> 1) & 0b1111;
            let output_flip = s_box[row_flip as usize][col_flip as usize];

            let xor_result = output_orig ^ output_flip;

            for i in 0..4 {
                if (xor_result >> i) & 1 == 1 {
                    flip_counts[i] += 1;
                }
            }
        }
    }

    let total_tests_per_output_bit = (num_inputs * 6) as f64;

    println!("  S-Box S{}:", s_box_index + 1);
    let mut total_deviation = 0.0;

    for i in 0..4 {
        let flip_ratio = flip_counts[i] as f64 / total_tests_per_output_bit;
        let deviation = (flip_ratio - 0.5).abs();
        total_deviation += deviation;

        println!(
            "    Bit de ieșire {}: Raport schimbare = {:.4} (Deviație de la 0.5: {:.4})",
            i + 1,
            flip_ratio,
            deviation
        );
    }

    total_deviation / 4.0
}

/// Măsoară proprietatea de Non-Linearitate (non-linearity) a unui S-Box.
fn calculate_s_box_non_linearity(s_box_index: usize) -> u8 {
    let s_box = S_BOXES[s_box_index];
    let mut min_non_linearity = 32;

    for output_bit in 0..4 {
        let mut min_deviation = 32;

        let mut truth_table = [0u8; 64];
        for input in 0..64 {
            let row = ((input & 0b100000) >> 4) | (input & 0b000001);
            let col = (input >> 1) & 0b1111;
            let output = s_box[row as usize][col as usize];

            truth_table[input as usize] = (output >> output_bit) & 1;
        }

        for mask in 0..64 {
            for constant in 0..2 {
                let mut hamming_distance = 0;

                for input in 0..64 {
                    // FIX: Am adăugat 'as u32' pentru a rezolva eroarea E0689
                    let linear_part = ((input & mask) as u32).count_ones() % 2;
                    let affine_output = (linear_part as u8) ^ constant;

                    if truth_table[input as usize] != affine_output {
                        hamming_distance += 1;
                    }
                }

                let deviation = hamming_distance.min(64 - hamming_distance) as u8;
                min_deviation = min_deviation.min(deviation);
            }
        }

        min_non_linearity = min_non_linearity.min(min_deviation);
    }

    min_non_linearity
}

fn main() {
    println!("--- Programul 5: Testarea Proprietăților S-Box-urilor DES ---");

    // --- 1. Testarea Strict Avalanche Criterion (SAC) ---
    println!("\n[1] Testul Criteriului de Avalanșă Strică (SAC)");
    println!(
        "SAC ideal: Raport de schimbare de 0.5 pentru fiecare bit de ieșire la schimbarea unui bit de intrare."
    );

    let mut total_sac_deviation = 0.0;
    for i in 0..8 {
        total_sac_deviation += test_s_box_sac(i);
    }

    let avg_sac_deviation = total_sac_deviation / 8.0;
    println!(
        "\nMedia deviației SAC pe toate S-Box-urile: {:.4}",
        avg_sac_deviation
    );
    if avg_sac_deviation < 0.05 {
        println!("Rezultat: S-Box-urile DES demonstrează o proprietate SAC puternică.");
    }

    // --- 2. Testarea Non-Linearității ---
    println!("\n[2] Testul Non-Linearității");
    println!(
        "Non-linearitatea ideală: 2^(n-1) - 2^(n/2 - 1) = 32 - 4 = 28. (Valoare ideală pentru 6 biți: 28)"
    );

    let mut total_non_linearity = 0;
    for i in 0..8 {
        let non_linearity = calculate_s_box_non_linearity(i);
        total_non_linearity += non_linearity as u32;

        println!("  S-Box S{}: Non-Linearitate = {}", i + 1, non_linearity);
    }

    let avg_non_linearity = total_non_linearity as f64 / 8.0;
    println!(
        "\nNon-Linearitate medie pe toate S-Box-urile: {:.2}",
        avg_non_linearity
    );

    if avg_non_linearity >= 24.0 {
        println!(
            "Rezultat: S-Box-urile DES au o non-linearitate ridicată, oferind o bună rezistență la atacurile liniare."
        );
    }
}
