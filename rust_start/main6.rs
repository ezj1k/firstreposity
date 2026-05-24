use std::io;

fn introduce(mesaj: &str) -> i32 {
    let mut numar = String::new();

    println!("{}", mesaj);
    io::stdin().read_line(&mut numar).expect("eroare de citire");

    let numar: i32 = match numar.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("eroare");
            std::process::exit(1);
        }
    };
    return numar;
}

/*fn main() {
loop {
        let mut cel = String::new();
        println!("Screi in celsius un nr");
        io::stdin().read_line(&mut cel).expect("nu e corect");

        let cel:f32 = match cel.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let double_cel = cel * 2.0;
        println!("da {}", double_cel);
        break;
    }
}*/
//celsius in farenheit
/*fn main() {
    loop {
        let mut rp = String::new();
        println!("Introdu numarul(C): ");
        io::stdin().read_line(&mut rp).expect("nu e numar");

        let rp: f32 = match rp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let rp_f: f32 = rp * (9.0 / 5.0) + 32.0;

        println!("ati introdus {}C, dar in F va fi {}",rp, rp_f);
        break;
    }
}*/
//al n-lea nr fibonacci
/*fn main() {
    let mut nr = String::new();
    println!("introdu un numar");
    io::stdin().read_line(&mut nr).expect("nu e nr");
    let mut nr:i32 = match nr.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("eroare");
            return;
        }
    };
    let mut nr1:i32 = 0;
    let mut nr2:i32 = 1;
    while nr > 0 {
        println!("{},", nr1);
        let nr3=nr1+nr2;
        nr1=nr2;
        nr2=nr3;
        nr-=1;
        }
    println!();
}*/
//triunghi din simbol
/*fn main() {
    let mut x1 = String::new();
    io::stdin().read_line(&mut x1).expect("error");

    let x1 = x1.trim();
    let x2:char = match x1.chars().next() {
        Some(c) => c,
        None => {
            println!("sosi huy");
            return;
        }
    };

    /*for i in 1..5 {
        for _j in 0..i {
            print!("{}", x2);
        }
        println!();
    }*/

    /*for i in 1..5 {
        for _j in i..5 {
            print!("{}", x2);
        }
        println!();
    }*/

    /*for i in 1..5 {
        for _k in i..4 {
            print!(" ");
        }
        for _j in 0..i {
            print!("{}", x2);
        }
        println!();
    }*/

    /*for i in 1..5 {
        for _k in 0..i-1 {
            print!(" ");
        }
        for _j in i..5 {
            print!("{}", x2);
        }
        println!();
    }*/

    println!("daaaaa");
}*/
//poezie din carte cu cicluri
/*fn main() {
let x = ["", "2 turtle doves", "3 french hens", "4 calling birds", "5 golden rings", "6 geese a laying", "7 swans a swimming", "8 maids a milking", "9 ladies dancing", "10 lords a leaping", "11 pipers piping", "12 drummers drumming"];
    let words = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelveth"];

    for mut i in 0..12 {
        println!("On the {} day of Christmas", words[i]);
        println!("My true love sent to me:");
        if i != 0 {

            while i !=0 {
            println!("{}", x[i]); // 2 - 2  3 - 3 2  4 - 4 3 2
            i-=1;
            }
        }
        println!("And a partridge in a pear tree");
        println!();
        }
}*/
//elevi in tabara
/*fn main() {
    let mut fete = String::new();
    println!("Introdu:");
    io::stdin().read_line(&mut fete).expect("eroare");

    let fete: i32 = match fete.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("eroare");
            return;
        }
    };
    let baieti: i32 = &fete + 10;
    let suma1:i32 = baieti + fete;
    let suma2: i32 = 2 * fete + 10;
    println!("suma1 = {}, suma2 = {}", suma1, suma2);
}*/
//autobus cu elevi
/*fn main() {
    let mut firsts = String::new();
    let mut seconds = String::new();
    let initial: i32 = 7;
    
    println!("introdu prima scoala");
    io::stdin().read_line(&mut firsts).expect("error1");

    println!("introdu adoua scoala");
    io::stdin().read_line(&mut seconds).expect("error2");

    let firsts: i32 = match firsts.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("nu e nr 1");
            return;
        }
    };

    let seconds: i32 = match seconds.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("nu e nr 2");
            return;
        }
    };

    let suma1: i32 = firsts + seconds + initial;
    let suma2: i32 = firsts + seconds + 7;
    println!("suma1 = {}, suma2 = {}", suma1, suma2);
}*/
//globuri pe brad
/*fn main() {
    let albe: i32 = introduce("introdu albe: ");
    let rosii:i32 = &albe + 3;
    let blue:i32 = &albe + &rosii - 2;

    let suma1: i32 = albe + rosii + blue;
    let suma2: i32 = albe * 4 + 4;

    println!("suma1 = {}, suma2 = {}", suma1, suma2);
}*/
//un nr in mijloc cu consecutivele sale
/*fn main() {
    let x = introduce("introdu numarul lui Vasile");
    println!("{}, {}, {}, {}, {}", x-2, x-1, x, x+1, x+2);
    if x==0 {
       println!("0, 0, {}, {}, {}", x, x+1, x+2); 
    }
}*/
//mere
/*fn main() {
    let mut x = introduce("introdu numarul de mere");
    let mut y = x;
    if x<2 {
        println!("eroare");
    }
    else {
    println!("{}, {}", x-2, x+1);
    x-=2;
    y += 1;
    println!("{}, {}", x, y);
    }
}*/
//formula pentru varsta
/*fn main() {
    let v = introduce("introdu varsta");
    let h = v*5 +80;
    let g = v*2 + 8;

    println!("v: {}; g: {}; h: {}",v, g, h);
    println!("v: {}; g: {}; h: {}",v, v*2+8,v*5+80);
}*/
//3 cifre si le scriu impreuna
/*fn main() {
    let x = introduce("introdu x: ");
    let y = introduce("introdu y: ");
    let z = introduce("introdu z: ");

    if x>9 || y>9 || z>9 || x<0 || y<0 || z<0 {
        println!("careva numar e prea mare");
    } else if  x<0 || y<0 || z<0 {
        println!("careva numar e prea mic");
    } else {
        println!("{}{}{}, {}{}{}, {}{}{}, {}{}{}, {}{}{}", x,y,z, x,z,y, y,x,z, y,z,x, z,x,y);
    }
}*/
//
/*fn main() {
    let x = introduce("introdu x: ");
    let y = introduce("introdu y: ");
    let z = introduce("introdu z: ");

    println!("{}, {}, {}", x+y, x+z, z+y);
}*/
//tabla inmultirii
/*fn main() {
    let mut x = introduce("introdu x: ");
    while x<1 || x>10 {
            println!("incorect! introdu din nou");
            x = introduce("introdu x: ");
    }
    println!("{}*1={}", x, x);
    println!("{}*2={}", x, x*2);
    println!("{}*3={}", x, x*3);
    println!("{}*4={}", x, x*4);
    println!("{}*5={}", x, x*5);
    println!("{}*6={}", x, x*6);
    println!("{}*7={}", x, x*7);
    println!("{}*8={}", x, x*8);
    println!("{}*9={}", x, x*9);
    println!("{}*10={}", x, x*10);
}*/
//ani luni saptamani zile ore
/*fn main() {
    loop {
        println!("intronu numar de ani");
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("error");

        let x: u32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("numar negativ introdus! introdu din nou");
                continue;
            }
        };
        
        let y = x/7;
        let z = x%7;
        if z != 0 {
            println!("ani: {}; luni: {}; saptamani: {} si {} zile; zile: {}; ore: {};", x, x*12, x*52+y, z, x*365, x*365*24);
        } else {
            println!("ani: {}; luni: {}; saptamani: {}; zile: {}; ore: {};", x, x*12, x*52+y, x*365, x*365*24);
        }
        break;
    }
}*/
//sa schimb cu locurile valorile lui a si b
/*fn main() {
    use std::mem;
    let mut a = introduce("introdu a: ");
    let mut b = introduce("introdu b: ");

    let temp: i32 = a;
    a = b;
    b = temp;
    println!("a: {}; b: {}", a, b);

    mem::swap(&mut a, &mut b);
    println!("a: {}; b: {}", a, b);

    (a, b) = (b, a);
    println!("a: {}; b: {}", a, b);
    
    a += b;
    b = a-b;
    a = a-b;
    println!("a: {}; b: {}", a, b);
}*/
//de la un oras la altul viteza medie
/*fn main() {
    let mut x = introduce("introdu distanta: ") as f32;
    let mut a = introduce("introdu timpul: ") as f32;

    while x<=0.0 {
        println!("introdu din nou");
        x = introduce("introdu distanta: ") as f32;
    }

    while a<=0.0 {
        println!("introdu din nou");
        a = introduce("introdu distanta: ") as f32;
    }
    println!("timp mediu: {}", x/a);
}*/
//de la un oras la altul combustibil pretul si distanta
/*fn main() {
    let mut x = introduce("introdu distanta: ") as f32;
    let mut y = introduce("introdu litri la 100km: ") as f32;
    let mut pret = introduce("introdu pretul: ") as f32;

    while x<=0.0 {
        println!("introdu din nou");
        x = introduce("introdu distanta: ") as f32;
    }

    while y<=0.0 {
        println!("introdu din nou");
        y = introduce("introdu litri la 100km: ") as f32;
    }

    while pret<=0.0 {
        println!("introdu din nou");
        pret = introduce("introdu pretul: ") as f32;
    }

    println!("pretul deplasarii: {}", (x/100.0)*y*pret);
}*/
//km/h la nava
/*fn main() {
    let mut d = introduce("introdu distanta: ") as f32;
    while d<=0.0 {
        println!("introdu din nou");
        d = introduce("introdu distanta: ") as f32;
    }
    
    let mut a = introduce("introdu weeks: ") as f32;
    while a<0.0 {
        println!("introdu din nou");
        a = introduce("introdu weeks: ") as f32;
    }
    
    let mut b = introduce("introdu zile: ") as f32;
    while b < 0.0 || (a == 0.0 && b == 0.0) {
        println!("introdu din nou");
        b = introduce("introdu zile: ") as f32;
    }

    if b >= 7.0 {
        a += (b / 7.0).floor(); // ((b/7.0) as i32) as f32;
        b = b % 7.0; // ((b as i32)%7) as f32;
    }

    println!("viteza: {}", d/((a*7.0+b)*24.0));
}*/