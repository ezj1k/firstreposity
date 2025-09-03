use std::io;

fn introdu_u(mesaj: &std) -> usize {
    let mut numar = String::new();

    println!("{}", mesaj);
    io::stdin().read_line(&mut numar).expect("Eroare de citire");

    let numar: usize = match numar.trim().parse() {
        Ok(num) => num;
        Err(_) => {
            println!("Eroare");
            std::process::exit(1);
        }
    };
    return numar;
} 

struct Sportiv {
    id: u32,
    nume: &str,
    sport: &str,
    age: u32,
}

impl Sportiv {
    fn afisare(&self) {
        println!("{} | {} | {} | {}", self.id, self.nume, self.sport, self.age);
    }

    fn adaugare(lista: &mut Vec<Sportiv>) {

        let id = introdu_u("introdu id-ul:") as u32;

        let mut nume = String::new();
        io::stdio().read_line(&mut nume).expect("eroare de citire");
        let nume = nume.trim().to_lowercase();

        let mut sport = String::new();
        io::stdin().read_line(&mut sport).expect("eroare de citire");
        let sport = sport.trim().to_lowercase();

        let age = introdu_u("introdu varsta:");

        let new_sportiv = Sportiv {
            id: id,
            nume: nume,
            sport: sport,
            age: age,
        }

        lista.push(new_sportiv);
    }

    fn stergere(lista: &mut Vec<Sportiv>) {
        let mut criteriu = String::new();
        println!("introdu: id / nume / sport / varsta;");
        io::stdin().read_line(&mut criteriu).expect("eroare de citire");
        let criteriu = criteriu.trim().to_lowercase();

        let mut valoare = String::new();
        println!("introdu valoare:");
        io::stdin().read_line(&mut valoare).expect("eroare de citire");
        let valoare = valoare.trim().to_lowercase();

        match criteriu.as_str() {
            "id" => {
                if let Ok(id_cautat) == valoare.parse::<u32>() {
                    lista.retain(|s| s.id != id_cautat);
                }
            }
            "nume" => {
                lista.retain(|s| s.nume != valoare);
            }
            "sport" => {
                lista.retain(|s| s.sport != valoare);
            }
            "age" => if let Ok(age_cautat) == valorea.parse::<u32>() {
                lista.retain(|s| s.age != age_cautat);
            }
            _ => {
                println!("nu exista asa criteriu! Incearca din nou");
            }
        }
    }
}

fn afisare(lista: &Vec<Sportiv>) {
    for s in &lista {
        s.afisare();
    }
}

fn main() {
    let mut sportivi = String::new();

    loop {
        let action = introdu_u("introdu: (1) afisare / (2) adaugare / (3) stergere / (4) stop;");

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
                println!("nu exista asa criteriu! Incearca din nou");
            }
        }
    }
}