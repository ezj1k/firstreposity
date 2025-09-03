use std::io;

fn introduce_u32(mesaj: &str) -> u32 {
    let mut numar = String::new();
    println!("{}", mesaj);
    io::stdin().read_line(&mut numar).expect("eroare de citire");
    let numar: u32 = match numar.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("gresit");
            std::process::exit(1);
        }
    };
    return numar;
}

//1, ..., 12,13,15
fn caractere() -> String {
    let mut input = String::new();
    println!("Introdu sir de caractere: ");
    io::stdin().read_line(&mut input).expect("eroare de citire");
    
    return input.trim().to_lowercase().to_string();
}

//14
// fn caractere() -> String {
//     loop {
//         let mut input = String::new();
//         println!("Introdu sir de caractere: ");
//         io::stdin().read_line(&mut input).expect("eroare de citire");
        
//         let input = input.trim().to_string();

//         if input.len() >= 3 {
//             return input;
//         } else {
//             println!("mai multe elemente! ");
//         }
//     }
// }

fn main() {
    //1
    // let sirul = caractere();
    // let numar_de_majuscule_2 = sirul.chars().filter(|c| c.is_ascii_uppercase()).count();

    // let mut numar_de_majuscule = 0;
    // for i in sirul.chars() {
    //     if i>= 'A' && i<='Z' {
    //         numar_de_majuscule+=1;
    //     }
    // }
    // println!("{}", numar_de_majuscule);
    // println!("{}", numar_de_majuscule_2);

    //2
    // let sirul = caractere();
    // let mut numar_de_numere = 0;
    // for i in sirul.chars() {
    //     if i>= '0' && i<='9' {
    //         numar_de_numere+=1;
    //     }
    // }
    // println!("{}", numar_de_numere);

    //3
    // let sirul = caractere();
    // let mut numar_de_caractere = 0;
    // for i in sirul.chars() {
    //     if i== '-' && i=='+' && i=='*' && i=='/' && i=='%' && i=='(' && i==')' {
    //         numar_de_caractere+=1;
    //     }
    // }
    // println!("{}", numar_de_caractere);

    //4
    // let sirul = caractere();
    // let mut numar_de_numere_pare = 0;
    // for i in sirul.chars() {
    //     if i== '2' && i=='4' && i=='6' && i=='8' {
    //         numar_de_numere_pare+=1;
    //     }
    // }
    // println!("{}", numar_de_numere_pare);

    //5
    // let sirul = caractere();
    // let mut rezultat = String::new();
    // for i in sirul.chars() {
    //     if i >= '0' && i <= '9' {
    //         rezultat.push('+');
    //     } else {
    //         rezultat.push(i);
    //     }
    // }
    // println!("{}", rezultat);

    //6
    // let sirul = caractere();
    // let mut rezultat = String::new();
    // for i in sirul.chars() {
    //     if i== 'a' && i=='e' && i=='y' && i=='u' && i=='i' && i=='o' {
    //         rezultat.push('+');
    //     } else {
    //         rezultat.push(' ');
    //     }
    // }
    // println!("{}", rezultat);

    //7
    // let sirul = caractere();
    // let mut rezultat = String::new();
    // for i in sirul.chars() {
    //     if i >= 'A' && i <= 'Z' {
    //         rezultat.push('8');
    //     } else {
    //         rezultat.push(i);
    //     }
    // }
    // println!("{}", rezultat);

    //8
    // let sirul = caractere();
    // let mut rezultat = String::new();
    // for i in sirul.chars() {
    //     if i >= '0' && i <= '9' {
    //         rezultat.push(i);
    //     }
    // }
    // println!("{}", rezultat);

    //9
    // let sirul = caractere();
    // let mut rezultat = String::new();
    // for i in sirul.chars() {
    //     if i == ' ' {
    //         rezultat.push('\n');
    //     } else {
    //         rezultat.push(i);
    //     }
    // }
    // println!("{}", rezultat);

    //10
    // let sirul = caractere();
    // let mut rezultat = String::new();
    // for i in sirul.chars() {
    //     if i!= 'a' && i!='e' && i!='y' && i!='u' && i!='i' && i!='o' {
    //         rezultat.push(i);
    //     }
    // }
    // println!("{}", rezultat);

    //11
    // let sirul = caractere();
    // let mut rezultat = String::new();
    // for i in sirul.chars() {
    //     if i>='0' && i<='9' {
    //         let x = (i as u8) - b'0';
    //         if x%3==0 {
    //             rezultat.push(i);
    //         }
    //     }
    // }
    // println!("{}", rezultat);

    //12
    // let sirul = caractere();
    // let mut rezultat = false;
    // for i in sirul.chars() {
    //     if i>='0' && i<='9' {
    //         let x = (i as u8) - b'0';
    //         if x%2==0 {
    //             rezultat=true;
    //             break;
    //         }
    //     }
    // }
    // println!("{}", rezultat);

    //13
    // let word1 = caractere();
    // let word2 = caractere();
    // let word3 = caractere();
    // let word4 = caractere();
    // let mut rezultat1:[String; 4] = [word1, word2, word3, word4];
    // let mut rezultat = String::new();
    // for i in 0..4 {
    //     rezultat.push_str(rezultat1[i].as_str());
    //     rezultat.push(' ');
    // }
    // rezultat.pop();
    // rezultat.push('.');
    // println!("{}\n{:?}", rezultat,rezultat1);

    //14
    // let word1 = caractere();
    // let word2 = caractere();
    // let word3 = caractere();
    // let word4 = caractere();
    // let mut word5 = String::new();

    // for i in word1.chars().take(2) {
    //     word5.push(i);
    // }
    // for i in word2.chars().take(1) {
    //     word5.push(i);
    // }
    // for i in word3.chars().take(3) {
    //     word5.push(i);
    // }
    // for i in word4.chars().take(word4.len()/2) {
    //     word5.push(i);
    // }
    // println!("{}", word5);

    //15
    let word1 = caractere();
    let word2 = caractere();
    let word3 = caractere();
    let word4 = caractere();
    let mut rezultat1:[String; 4] = [word1, word2, word3, word4];

    rezultat1.sort();
    
    println!("{:?}", rezultat1);
}