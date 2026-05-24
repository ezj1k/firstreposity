use std::fs;

fn introduce_u32(mesaj: &str) -> u32 {
    println!("{}", mesaj);
    let mut numar = String::new();
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

fn caractere() -> String {
    let mut input = String::new();
    println!("Introdu sir de caractere: ");
    io::stdin().read_line(&mut input).expect("eroare de citire");
    
    return input.trim().to_lowercase().to_string();
}

//ex4
// fn int_to_string(nr: i32) -> String {
//     let mut numar = String::new();
//     numar = nr.to_string();
//     return numar;
// }
// fn string_to_int(mesaj: &str) -> i32 {
//     let nr:i32 = match mesaj.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("gresit");
//             std::process::exit(1);
//         }
//     };
//     return nr;
// }

// #[derive(Debug)]
// struct Om {
//     name: String,
//     age: u32,
// }

//ex5
// use std::fs::File;
// use std::io::Write;
// use std::fs::OpenOptions;

//ex6
// use std::fs::File;
// use std::io::{self, Write, Read};

// fn read_numbers_from_file(path: &str) -> io::Result<Vec<i32>> {
//     let mut file = File::open(path)?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;

//     let numbers: Vec<i32> = content.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
//     Ok(numbers)
// }

// fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
//     let mut i = 0;
//     let mut j = 0;
//     let mut merged = Vec::new();

//     while i<a.len() && j<b.len() {
//         if a[i]<=b[j] {
//             merged.push(a[i]);
//             i+=1;
//         } else {
//             merged.push(b[j]);
//             j+=1;
//         }
//     }
//     merged.extend_from_slice(&a[i..]);
//     merged.extend_from_slice(&b[j..]);
//     merged
// }

fn main() -> io::Result<()> {
    //ex1
    // let x: Vec<Vec<i32>> = vec![vec![5; 3]; 3];
    // let y: Vec<Vec<i32>> = vec![vec![2; 3]; 3];

    // let mut res: Vec<Vec<i32>> = vec![vec![0; 3]; 3];
    // for i in 0..3 {
    //     for j in 0..3 {
    //         res[i][j]=x[i][j]+y[i][j];
    //     }
    // }
    // for i in 0..3 {
    //     for j in 0..3 {
    //         print!("{},",res[i][j]);
    //     }
    //     print!("\n");
    // }

    //ex2
    // let n = introduce_u32("introdu cate:");
    // let mut x = 1;
    // let mut y = 1;
    // for i in 1..=n/2 {
    //     print!("{} {} ", x, y);
    //     x+=y;//2
    //     y+=x;//3
    // }
    // if n%2==1 {
    //     print!("{}", x);
    // }

    //ex3
    // let mut oameni = vec![
    //     Om {name: "Ion".to_string(), age:18},
    //     Om {name: "Ana".to_string(), age:24},
    //     Om {name: "Dan".to_string(), age:23},
    // ];
    // oameni.sort_by_key(|s| s.age);
    // for i in 0..3 {
    //     println!("{:?}", oameni[i]);
    // }

    //ex4
    // let mut text = String::new();
    // text = int_to_string(2025);
    // println!("{}",text);
    // let nr = string_to_int("342");
    // println!("{}",nr);

    //ex5
    // let mut file = File::create("name.txt")?;
    // writeln!(file, "Careva text")?;
    // writeln!(file, "Alt text")?;

    // let continut = fs::read_to_string("name.txt")?;
    // println!("{}", continut);

    // let mut nr = 0;
    // nr = continut.matches("text").count();
    // println!("de {} ori", nr);
    //let mut file = OpenOptions::new().append(true).open("name.txt")?;
    //Ok(())

    //ex6
    // let numbers_a = read_numbers_from_file("a.txt")?;
    // let numbers_b = read_numbers_from_file("b.txt")?;
    // let merged = merge_sorted(&numbers_a, &numbers_b);
    // let mut output = File::create("c.txt")?;
    // for num in merged {
    //     write!(output, "{} ", num)?;
    // }
    // Ok(())
}