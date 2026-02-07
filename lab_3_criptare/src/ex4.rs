use std::time::Instant;

// --- 1. Tabele DES (CONSTANTE) ---

// Permutarea Inițială (IP) - 64 biți la 64 biți
const IP: [usize; 64] = [
    58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61,
    53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
];

// Permutarea Finală (IP⁻¹) - Inversa lui IP
const IP_INV: [usize; 64] = [
    40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22, 62, 30,
    37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19, 59, 27,
    34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57, 25,
];

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

// --- 3. Generarea Subcheilor (COMPLETE) ---

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

// --- 5. Criptare/Decriptare DES (Bloc) (COMPLETĂ) ---

/// Funcția principală de criptare/decriptare DES pentru un bloc de 64 de biți.
fn des_block(data: u64, subkeys: &[u64; 16], is_encrypt: bool) -> u64 {
    // 1. Permutarea Inițială (IP)
    let initial_permuted = permute(data, &IP);

    // 2. Divizarea în L0 și R0 (32 de biți fiecare)
    let mask_32: u64 = (1 << 32) - 1;
    let mut l = ((initial_permuted >> 32) & mask_32) as u32;
    let mut r = (initial_permuted & mask_32) as u32;

    // 3. Cele 16 Runde Feistel
    for i in 0..16 {
        let key_index = if is_encrypt { i } else { 15 - i };
        let k_i = subkeys[key_index];

        let temp = r;
        r = l ^ feistel_function(r, k_i);
        l = temp;
    }

    // 4. Recombinarea și Permutarea Finală (IP⁻¹)
    let pre_final: u64 = ((r as u64) << 32) | (l as u64);

    permute(pre_final, &IP_INV)
}

// --- 6. Triple DES (3DES) EDE ---

/// Criptare DES de bază.
fn des_encrypt_block(data: u64, subkeys: &[u64; 16]) -> u64 {
    des_block(data, subkeys, true)
}

/// Decriptare DES de bază.
fn des_decrypt_block(data: u64, subkeys: &[u64; 16]) -> u64 {
    des_block(data, subkeys, false)
}

/// Implementează 3DES EDE (Encrypt-Decrypt-Encrypt)
fn triple_des_ede(
    data: u64,
    subkeys_k1: &[u64; 16],
    subkeys_k2: &[u64; 16],
    subkeys_k3: &[u64; 16],
) -> u64 {
    // E(K1)
    let temp1 = des_encrypt_block(data, subkeys_k1);

    // D(K2)
    let temp2 = des_decrypt_block(temp1, subkeys_k2);

    // E(K3)
    des_encrypt_block(temp2, subkeys_k3)
}

fn main() {
    println!("--- Triple DES (3DES) EDE și Măsurarea Timpului ---");

    // Chei pentru 3DES EDE (K1, K2, K3)
    let k1_hex = "133457799BBCDF01";
    let k2_hex = "2A468ACEE0F80214";
    let k3_hex = "3B579BBDDF113355";

    let k1 = hex_to_u64(k1_hex).expect("K1 Invalid");
    let k2 = hex_to_u64(k2_hex).expect("K2 Invalid");
    let k3 = hex_to_u64(k3_hex).expect("K3 Invalid");

    let subkeys_k1 = generate_subkeys(k1);
    let subkeys_k2 = generate_subkeys(k2);
    let subkeys_k3 = generate_subkeys(k3);

    // --- Test de Corectitudine pe un Bloc ---
    let plaintext_hex = "0123456789ABCDEF";
    let plaintext = hex_to_u64(plaintext_hex).expect("Plaintext Invalid");

    let tdes_ciphertext = triple_des_ede(plaintext, &subkeys_k1, &subkeys_k2, &subkeys_k3);

    // Decriptarea (Se inversează cheile K1 și K3 pentru EDE)
    let tdes_decrypted = triple_des_ede(tdes_ciphertext, &subkeys_k3, &subkeys_k2, &subkeys_k1);

    println!("\nTest de Corectitudine (EDE):");
    println!("Cheia 1 (K1): {}", k1_hex);
    println!("Cheia 2 (K2): {}", k2_hex);
    println!("Cheia 3 (K3): {}", k3_hex);
    println!("Text Clar: {}", plaintext_hex);
    println!("Text Criptat: {}", u64_to_hex(tdes_ciphertext));
    println!("Text Decriptat: {}", u64_to_hex(tdes_decrypted));

    // --- Măsurarea Performanței (Exemplu) ---

    const BLOCK_SIZE: usize = 8;
    const NUM_BLOCKS: usize = 1024 * 1024 / BLOCK_SIZE; // 1MB de date
    const NUM_RUNS: u32 = 5;

    // Simulează datele
    let data_blocks: Vec<u64> = vec![plaintext; NUM_BLOCKS];

    println!("\n--- Măsurarea Timpului de Execuție (1 MB) ⏱️ ---");

    let mut total_duration = 0.0;

    for run in 0..NUM_RUNS {
        let start = Instant::now();

        for &block in data_blocks.iter() {
            // Criptarea unui bloc
            let _ = triple_des_ede(block, &subkeys_k1, &subkeys_k2, &subkeys_k3);
        }

        let duration = start.elapsed();
        let duration_ms = duration.as_secs_f64() * 1000.0;
        total_duration += duration_ms;
        println!("Rularea {}: {:.2} ms", run + 1, duration_ms);
    }

    let avg_time = total_duration / (NUM_RUNS as f64);
    println!(
        "\nTimp mediu pe {} rulări (Implementare proprie): {:.2} ms/MB",
        NUM_RUNS, avg_time
    );
}
