use std::{thread, time, io};
use rand::Rng;

fn introduce(mesaj:  &str) -> i32 {
    println!("{}",mesaj);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("eroare");
    let input:i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("eroare");
            std::process::exit(1);
        },
    };
    input
}

fn liftul_lucreaza(x: &mut i32, y: &mut i32, mesaj: &str, jos: &str, sus: &str) {
    let mut rng = rand::thread_rng();
    let mut stricat;
    if *x == *y {
        println!("{} {}", mesaj, *x);
    } else if *x > *y {
        println!("{}. de la {} la {}", jos, *x, *y);
        for i in (*y..=*x).rev() {
            stricat = rng.gen_range(1..=100);
            if i == *y {
                println!("{}<<<", i);
            } else {
                println!("{}<", i);
            }
            if stricat == 10 {
                println!("liftul sa stricat. la etajul {}", i);
                println!("asteptam sal remonteze!");
                thread::sleep(time::Duration::from_secs(10));
                *x = i;
                continue;
            }
            thread::sleep(time::Duration::from_secs(1));
        }
        *x = *y;
        println!("a ajuns la etajul {}!", *y);
    } else {
        println!("{}. de la {} la {}", sus, *x, *y);
        for i in *x..=*y {
            stricat = rng.gen_range(1..=100);
            if i == *y {
                println!("{}<<<", i);
            } else {
                println!("{}<", i);
            }
            if stricat == 10 {
                println!("liftul sa stricat. la etajul {}", i);
                println!("asteptam sal remonteze!");
                thread::sleep(time::Duration::from_secs(10));
                *x = i;
                continue;
            }
            thread::sleep(time::Duration::from_secs(1));
        }
        *x = *y;
        println!("a ajuns la etajul {}!", *y);
    }
}

fn main() {
    //let mut rng = rand::thread_rng();
    
    let mut etaje = introduce("cate etaje in bloc?");
    while etaje > 100 || etaje < 1 {
        etaje = introduce("introdu un numar intre [1 si 100]!");
    }
    let mut floors = Vec::new();
    let mut firstlvl = -2;
    for _ in -2..=etaje {
        floors.push(firstlvl);
        firstlvl += 1;
    }

    let mut elevatoron = 1;
    let mut uareon = introduce("la ce etaj esti:");

    liftul_lucreaza(&mut elevatoron, &mut uareon, "liftul si asa e la etajul", "acus liftul va veni la tine in jos", "acus liftul va merge la tine in sus");

    while uareon!=-2 {
        let mut wantfloor = introduce("la ce etaj vrei?");
        while wantfloor > etaje || wantfloor < -2 {
            wantfloor = introduce("introdu un numar existent!");
        }

        liftul_lucreaza(&mut elevatoron, &mut wantfloor, "deja esti la etajul", "acus vei merge in jos", "acus vei merge in sus");
        uareon = wantfloor;
    }
}