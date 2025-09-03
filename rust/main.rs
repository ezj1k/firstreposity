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

//sortare 2 numere
/*fn main() {
    let x = introduce("introdu 1 sportiv");
    let y = introduce("introdu 2 sportiv");

    if x>y {
        println!("{},{}", y,x);
    } else if x<y {
        println!("{},{}",x, y);
    }
}*/

//1nr*3  2nr*2
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    let mut y = introduce("introdu 2 nr");

    if x>y {
        x*=2;
        y*=3;
        println!("{},{}", x,y);
    } else if x<y {
        x*=3;
        y*=2;
        println!("{},{}",x, y);
    }
}*/

//primul e predecesor al lui aldoilea
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    let mut y = introduce("introdu 2 nr");

    if x==y-1 {
        println!("Da");
    } else {
        println!("Nu");
    }
}*/

//Ionel spune notele mai mari ca 7
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    let mut y = introduce("introdu 2 nr");
    let mut z = introduce("introdu 3 nr");

    while x<1 || x>10 {
        println!("nu exista asa nota");
        x = introduce("introdu 1 nr");
    }

    while y<1 || y>10 {
        println!("nu exista asa nota");
        y = introduce("introdu 2 nr");
    }

    while z<1 || z>10 {
        println!("nu exista asa nota");
        z = introduce("introdu 3 nr");
    }

    if x>=7 {
        print!("{}, ",x);
    }
    if y>=7 {
        print!("{}, ",y);
    }
    if z>=7 {
        print!("{}, ",z);
    }
}*/

//catul dintre 2 nr
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    let mut y = introduce("introdu 2 nr");

    if y==0 {
        println!("Error");
    } else {
        println!("{}",x/y);
    }
}*/

//Ascensor si 2 copii
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    let mut y = introduce("introdu 2 nr");
    
    while x<0 {
        println!("nu exista asa gr");
        x = introduce("introdu 1 nr");
    }

    while y<0 {
        println!("nu exista asa gr");
        y = introduce("introdu 2 nr");
    }

    if x+y<100 {
        println!("Ambii");
    } else {
        println!("pe rand");
    }
}*/

//Ionel si tv 20 ore
/*fn main() {
    let mut summa = 0;
    for i in 1..8 {
        let mesaj = format!("introdu ziua {}:",i);
        let mut x = introduce(&mesaj);
        while x<0 {
            println!("nu exista asa timp");
            x = introduce("introdu din nou:");
        }
        summa+=x;
    }

    if summa<20 {
        println!("Nu va fi");
    } else {
        println!("va fi pedepsit");
    }
}*/

//greutatea ideala
/*fn main() {
    println!("f sau b");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("eroare");
    let mut s2:char = match s.trim().chars().next() {
        Some(c)=>c,
        None => {
            println!("Nu ai introdus niciun caracter.");
            return;
        }
    };

    while s2!='f' && s2!='b' {
        println!("f sau b");
        s = String::new();
        io::stdin().read_line(&mut s).expect("eroare");
        s2 = match s.trim().chars().next() {
            Some(c) => c,
            None => {
                println!("Nu ai introdus niciun caracter.");
                return;
            }
        };
    }

    let mut h = introduce("introdu h");
    while h<0 {
        println!("nu exista h");
        h = introduce("introdu din nou:");
    }
    let f_h = h as f32;

    let mut v = introduce("introdu v");
    while v<0 {
        println!("nu exista v");
        v = introduce("introdu din nou:");
    }
    let f_v = v as f32;

    let gmas:f32 = 50.0+0.75*(f_h-150.0)+(f_v-20.0)/4.0;

    if s2=='f' {
        println!("{}", gmas-10.0);
    } else {
        println!("{}",gmas);
    }
}*/

//par impar
/*fn main() {
    let x = introduce("introdu 1 nr");
    let y = introduce("introdu 2 nr");
    let z = introduce("introdu 3 nr");

    for i in [x,y,z] {
        if i%2==0 {
            println!("{} - par;",i);
        } else {
            println!("{} - impar;",i);
        }
    }
}*/

//Ionel in tabara
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    
    while x<0 {
        println!("nu exista asa gr");
        x = introduce("introdu 1 nr");
    }

    println!("In casuta {}", x/4);
}*/

//repartizarea elevilor
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    
    while x<0 || x>125 {
        println!("nu exista asa gr");
        x = introduce("introdu 1 nr");
    }

    println!("In clasa {}", x/25+1);
}*/

//crescator
/*fn main() {
    let x = introduce("introdu 1 nr");
    let y = introduce("introdu 2 nr");
    let z = introduce("introdu 3 nr");

    if x>=y && x>=z {
        if y>z {
            println!("{}, {}, {}", z,y,x);
        } else {
            println!("{}, {}, {}", y,z,x);
        }
    } else if y>=x && y>=z {
        if x>z {
            println!("{}, {}, {}", z,x,y);
        } else {
            println!("{}, {}, {}", x,z,y);
        }
    } else {
        if y>x {
            println!("{}, {}, {}", x,y,z);
        } else {
            println!("{}, {}, {}", y,x,z);
        }
    }
}*/

//max min
/*fn main() {
    let x = introduce("introdu 1 nr");
    let y = introduce("introdu 2 nr");
    let z = introduce("introdu 3 nr");

    if x>=y && x>=z {
        if y>z {
            println!("max {}, min {}", x,z);
        } else {
            println!("max {}, min {}", x,y);
        }
    } else if y>=x && y>=z {
        if x>z {
            println!("max {}, min {}", y,z);
        } else {
            println!("max {}, min {}", y,x);
        }
    } else {
        if y>x {
            println!("max {}, min {}", z,x);
        } else {
            println!("max {}, min {}", z,y);
        }
    }
}*/

//mid
/*fn main() {
    let x = introduce("introdu 1 nr");
    let y = introduce("introdu 2 nr");
    let z = introduce("introdu 3 nr");

    if x>=y && x>=z {
        if y>z {
            println!("max {}", y);
        } else {
            println!("max {}", z);
        }
    } else if y>=x && y>=z {
        if x>z {
            println!("max {}", x);
        } else {
            println!("max {}", z);
        }
    } else {
        if y>x {
            println!("max {}", y);
        } else {
            println!("max {}", x);
        }
    }
}*/

//consecutive sau nu
/*fn main() {
    let x = introduce("introdu 1 nr");
    let y = introduce("introdu 2 nr");
    let z = introduce("introdu 3 nr");

    if (x-1==y && x-2==z) || (x-1==z && x-2==y) {
        println!("da");
    } else if (y-1==x && y-2==z) || (y-1==z && y-2==x) {
        println!("da");
    } else if (z-1==y && z-2==x) || (z-1==x && z-2==y){
        println!("da");
    } else {
        println!("nu");
    }
}*/

//Andrei spune notele
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    let mut y = introduce("introdu 2 nr");
    let mut z = introduce("introdu 3 nr");

    while x<0 || x>10 {
        println!("nu exista asa gr");
        x = introduce("introdu 1 nr");
    }

    while y<0 || y>10 {
        println!("nu exista asa gr");
        y = introduce("introdu 2 nr");
    }

    while z<0 || z>10 {
        println!("nu exista asa gr");
        z = introduce("introdu 3 nr");
    }

    if z>=8 {
        println!("{},{},{}",x,y,z);
    } else {
        if x>=y {
            println!("{}",x);
        } else {
            println!("{}",y);
        }
    }
}*/

//ex17
/*fn main() {
    let x = introduce("introdu 1 nr");
    let y = introduce("introdu 2 nr");
    let z = introduce("introdu 3 nr");

    if x%2==0 && y%2==0 && z%2==0 {
        if z>=y {
            println!("{}",z);
        } else {
            println!("{}",y);
        }
    } else {
        println!("{}",y+x);
    }
}*/

//cel mai mare par
/*fn main() {
    let x = introduce("introdu 1 nr");
    let y = introduce("introdu 2 nr");

    if x>=y {
        if x%2==0 {
            println!("{}",x);
        } else {
            println!("nui");
        }
    } else {
        if y%2==0 {
            println!("{}",y);
        } else {
            println!("nui");
        }
    }
}*/

//un numar din 3 cifre
/*fn main() {
    let mut x = introduce("introdu 1 nr");
    let mut y = introduce("introdu 2 nr");
    let mut z = introduce("introdu 3 nr");

    while x<=0 {
        println!("nu exista asa gr");
        x = introduce("introdu 1 nr");
    }

    while y<=0 {
        println!("nu exista asa gr");
        y = introduce("introdu 2 nr");
    }

    while z<=0 {
        println!("nu exista asa gr");
        z = introduce("introdu 3 nr");
    }

    if x>=y && x>=z {
        if y>=z {
            print!("{}{}{}",x,y,z);
        } else {
            print!("{}{}{}",x,z,y);
        }
    } else if y>=x && y>=z {
        if x>=z {
            print!("{}{}{}",y,x,z);
        } else {
            print!("{}{}{}",y,z,x);
        }
    } else {
        if y>=x {
            print!("{}{}{}",z,y,x);
        } else {
            print!("{}{}{}",z,x,y);
        }
    }
}*/

//cel mai mic nr posibil
fn main() {
    let mut x = introduce("introdu 1 nr");
    let mut y = introduce("introdu 2 nr");
    let mut z = introduce("introdu 3 nr");

    while x<0 || x>9{
        println!("nu exista asa gr");
        x = introduce("introdu 1 nr");
    }

    while y<0 || y>9{
        println!("nu exista asa gr");
        y = introduce("introdu 2 nr");
    }

    while z<0 || z>9{
        println!("nu exista asa gr");
        z = introduce("introdu 3 nr");
    }

    if x>=y && x>=z && x!=0 {
        if y>z {
            if z!=0 {
                print!("{}{}{}",z,y,x);
            } else {
                print!("{}{}{}",y,z,x);
            }
        } else if y<z {
            if y!=0 {
                print!("{}{}{}",y,z,x);
            } else {
                print!("{}{}{}",z,y,x);
            }
        } else if y==z {
            if y!=0 {
                print!("{}{}{}",y,z,x);
            } else {
                print!("{}{}{}",x,y,z);
            }
        }
    } else if x==0 {
        if y>z {
            if z!=0 {
                print!("{}{}{}",z,x,y);
            } else {
                print!("{}{}{}",y,x,z);
            }
        } else if y<z {
            if y!=0 {
                print!("{}{}{}",y,x,z);
            } else {
                print!("{}{}{}",z,x,y);
            }
        } else if z==y {
            if y!=0 {
                print!("{}{}{}",y,x,z);
            } else {
                print!("000");
            }
        }
    } else if y>=z && y>=x && y!=0 {
        if z>x {
            if x!=0 {
                print!("{}{}{}",x,z,y);
            } else {
                print!("{}{}{}",z,x,y);
            }
        } else if z<x {
            if z!=0 {
                print!("{}{}{}",z,x,y);
            } else {
                print!("{}{}{}",x,z,y);
            }
        } else if z==x {
            if z!=0 {
                print!("{}{}{}",z,x,y);
            } else {
                print!("{}{}{}",y,z,x);
            }
        }
    } else if y==0 {
        if z>x {
            if x!=0 {
                print!("{}{}{}",x,y,z);
            } else {
                print!("{}{}{}",z,y,x);
            }
        } else if z<x {
            if z!=0 {
                print!("{}{}{}",z,y,x);
            } else {
                print!("{}{}{}",x,y,z);
            }
        } else if x==z {
            if z!=0 {
                print!("{}{}{}",z,y,x);
            } else {
                print!("000");
            }
        }
    } else if z>=x && z>=y && z!=0 {
        if x>y {
            if y!=0 {
                print!("{}{}{}",y,x,z);
            } else {
                print!("{}{}{}",x,y,z);
            }
        } else if x<y {
            if x!=0 {
                print!("{}{}{}",x,y,z);
            } else {
                print!("{}{}{}",y,x,z);
            }
        } else if x==y {
            if x!=0 {
                print!("{}{}{}",x,y,z);
            } else {
                print!("{}{}{}",z,x,y);
            }
        }
    } else if z==0 {
        if x>y {
            if y!=0 {
                print!("{}{}{}",y,z,x);
            } else {
                print!("{}{}{}",x,z,y);
            }
        } else if x<y {
            if x!=0 {
                print!("{}{}{}",x,z,y);
            } else {
                print!("{}{}{}",y,z,x);
            }
        } else if y==x {
            if x!=0 {
                print!("{}{}{}",x,z,y);
            } else {
                print!("000");
            }
        }
    }
}