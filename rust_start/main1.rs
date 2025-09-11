use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("vector: {:?}", v);
    v[0] = 456;
    println!("vector: {}", v[0]);

    let mut v2 = vec![1,3,4,5,6];
    v2.push(2);
    println!("vector: {:?}", v2);

    match v2.get(2) {
        Some(value) => println!("Vector el: {}", value),
        None => println!("Error"),
    }

    let mut v3 = vec![1,3,4,5,6,7];
    v3.pop(); // sterge ultimul
    v3.push(1111);
    v3.remove(1); //stege index 1
    for value in &v3 {
        println!("El: {}", value);
    }

    let s1 = String::new();
    let s2 = String::from("Hellow");

    println!("{}", s2);

    let s3 = s1 + &s2;

    let mut word = String::new();
    word.push_str("Hello");
    word.push(' ');
    word.push_str("World!");

    println!("{}", s3);
    println!("{}", word);

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Red", 20);
    scores.insert("White", 30);

    scores.remove("Blue");

    println!("Hash: {:?}", scores);

    scores.insert("Red", 132);
    println!("Hash: {}", scores.get("Red").unwrap());
}
