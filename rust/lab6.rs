use std::io;

// fn introduce(mesaj: &str) -> i32 {
//     let mut numar = String::new();

//     println!("{}", mesaj);
//     io::stdin().read_line(&mut numar).expect("eroare de citire");

//     let numar: i32 = match numar.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("eroare");
//             std::process::exit(1);
//         }
//     };
//     return numar;
// }

fn introduce_u(mesaj: &str) -> usize {
    let mut numar = String::new();

    println!("{}", mesaj);
    io::stdin().read_line(&mut numar).expect("eroare de citire");

    let numar: usize = match numar.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("eroare");
            std::process::exit(1);
        }
    };
    return numar;
}

// fn introduce_float(mesaj: &str) -> f32 {
//     let mut numar = String::new();

//     println!("{}", mesaj);
//     io::stdin().read_line(&mut numar).expect("eroare de citire");

//     let numar: f32 = match numar.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("eroare");
//             std::process::exit(1);
//         }
//     };
//     return numar;
// }

struct Sportiv {
    id: u32,
    nume: String,
    sport: String,
    varsta: u32,
}

impl Sportiv {
    fn afisare(&self) {
        println!("{} | {} | {} | {}", self.id, self.nume, self.sport, self.varsta);
    }

    fn adaugare(lista: &mut Vec<Sportiv>) {
//id
        let id = introduce_u("introdu id-ul:") as u32;
//nume
        let mut nume = String::new();
        println!("introdu numele:");
        io::stdin().read_line(&mut nume).expect("eroare de citire");
        let nume = nume.trim().to_lowercase();
//sport
        let mut sport = String::new();
        println!("introdu sportul:");
        io::stdin().read_line(&mut sport).expect("eroare de citire");
        let sport = sport.trim().to_lowercase();
//varsta
        let varsta = introduce_u("introdu varsta:") as u32;
//--------------------
        let nou = Sportiv {
            id,
            nume,
            sport,
            varsta,
        };
        lista.push(nou);
    }

    fn stergere(lista: &mut Vec<Sportiv>) {
        let mut criteriu = String::new();
        println!("introdu: id / nume / sport / varsta;");
        io::stdin().read_line(&mut criteriu).expect("eroare de citire");
        let criteriu = criteriu.trim().to_lowercase();

        let mut valoare = String::new();
        println!("Introdu valoarea:");
        io::stdin().read_line(&mut valoare).expect("eroare de citire");
        let valoare = valoare.trim().to_lowercase();

        match criteriu.as_str() {
            "id" => {
                if let Ok(id_cautat) = valoare.parse::<u32>() {
                    lista.retain(|s| s.id != id_cautat);
                }
            }
            "nume" => {
                lista.retain(|s| s.nume != valoare);
            }
            "sport" => {
                lista.retain(|s| s.sport != valoare);
            }
            "varsta" => {
                if let Ok(varsta_cautata) = valoare.parse::<u32>() {
                    lista.retain(|s| s.varsta != varsta_cautata);
                }
            }
            _ => {
                println!("Criteriu necunoscut!");
            }
        }
    }
}

fn afisare(lista: &Vec<Sportiv>) {
    for s in lista {
        s.afisare();
    }
}

fn main() {
    let mut sportivi: Vec<Sportiv> = Vec::new();

    loop {
        let action = introduce_u("introdu: (1) afisare / (2) adaugare / (3) stergere / (4) stop;");

        match action {
            1 => {
                afisare(&sportivi);
            }
            2 => {
                Sportiv::adaugare(&mut sportivi);
            }
            3 => {
                Sportiv::stergere(&mut sportivi);
            }
            4 => {
                break;
            }
            _ => {
                println!("Criteriu necunoscut!");
            }
        }
    }
}