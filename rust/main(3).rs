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
//1.1
// fn suma(a:u32, b:u32) -> u32 {
//     let suma = a+b;
//     return suma;
// }

//1.2
// fn produs(a:u32, b:u32) -> u32 {
//     let produs = a*b;
//     return produs;
// }

//1.3
// fn media(a:f32, b:f32) -> f32 {
//     let produs = (a+b)/2.0;
//     return produs;
// }

//1.4
// fn divizor_mare2(mut a:u32, mut b:u32) -> u32 {
//     if a==b {
//         return a;
//     }

//     if b>a {
//         let temp = a;
//         a=b;
//         b=temp;
//     }
//     let mut nr:u32=1;
//     for i in (1..=b).rev() {
//         nr=i as u32;
//         if a%nr==0 && b%nr==0 {
//             return nr;
//         }
//     }
//     return nr;
// }

//1.5
// fn multiplu_mic2(mut a:u32, mut b:u32) -> u32 {
//     if a==b {
//         return a;
//     }

//     if b>a {
//         let temp = a;
//         a=b;
//         b=temp;
//     }
//     let mut nr:u32=a*b;
//     for i in b..=a*b {
//         let x = i as u32;
//         if i%a==0 && i%b==0 {
//             nr = x;
//             return nr;
//         }
//     }
//     return nr;
// }

//1.6
// fn mai_mic(a:u32, b:u32) -> u32 {
//     if a<=b {
//         return a;
//     } else {
//         return b;
//     }
// }

//1.7
// fn mai_mare(a:u32, b:u32) -> u32 {
//     if a>=b {
//         return a;
//     } else {
//         return b;
//     }
// }

//1.8
// fn care_se_contin(mut a:u32, mut b:u32) -> bool {
//     let mut vector1 = Vec::new();
//     let mut vector2 = Vec::new();
//     while a!=0 {
//         let x = a%10;
//         vector1.push(x);
//         a/=10;
//     }
//     while b!=0 {
//         let x = b%10;
//         vector2.push(x);
//         b/=10;
//     }
//     for i in &vector1 {
//         for j in &vector2 {
//             if i==j {
//                 return true;
//             }
//         }
//     }
//     return false;
// }

//1.9
// fn in_primul(mut a:u32, mut b:u32) -> Vec<u32> {
//     let mut vector1 = Vec::new();
//     let mut vector2 = Vec::new();
//     let mut vector3 = Vec::new();

//     while a!=0 {
//         let x = a%10;
//         vector1.push(x);
//         a/=10;
//     }

//     while b!=0 {
//         let x = b%10;
//         vector2.push(x);
//         b/=10;
//     }

//     'sare: for i in &vector1 {
//         for j in &vector2 {
//             if i==j {
//                 continue 'sare;
//             }
//         }
//         vector3.push(*i as u32);
//     }
//     vector3.reverse();
//     return vector3;
// }

//1.10
// fn numar_maxim(mut a:u32, mut b:u32) -> Vec<u32> {
//     let mut vector1 = Vec::new();

//     while a!=0 {
//         let x = a%10;
//         vector1.push(x);
//         a/=10;
//     }

//     while b!=0 {
//         let x = b%10;
//         vector1.push(x);
//         b/=10;
//     }

//     let v = vector1.len();
//     for i in 0..v-1 {
//         for j in i+1..v {
//             if vector1[i] < vector1[j] {
//                 vector1.swap(i, j);
//             }
//         }
//     }
//     return vector1;
// }

//1.11
// fn div_mare3(a:u32, b:u32, c:u32) -> u32 {
//     let r = a.min(b.min(c));

//     let mut nr=1;
//     for i in (1..=r).rev() {
//         nr=i as u32;
//         if a%nr==0 && b%nr==0 && c%nr==0 {
//             return nr;
//         }
//     }
//     return nr;
// }

//1.12
// fn cmmmc3(a:u32, b:u32, c:u32) -> u32 {
//     let r = a.min(b.min(c));

//     let mut nr:u32=a*b;
//     for i in r..=a*b*c {
//         let x = i as u32;
//         if i%a==0 && i%b==0 && i%c==0 {
//             nr = x;
//             return nr;
//         }
//     }
//     return nr;
// }

//1.13
// fn mai_mare3(a:u32, b:u32, c:u32) -> u32 {
//     let r = a.max(b.max(c));

//     return r;
// }

//1.14
// fn mai_mic3(a:u32, b:u32, c:u32) -> u32 {
//     let r = a.min(b.min(c));

//     return r;
// }

//1.15
// fn div_3(a:u32, b:u32, c:u32) -> Vec<u32> {
//     let r = a.min(b.min(c));
//     let mut vector1 = Vec::new();

//     let mut nr;
//     for i in (1..=r).rev() {
//         nr=i as u32;
//         if a%nr==0 && b%nr==0 && c%nr==0 {
//             vector1.push(nr);
//         }
//     }
//     return vector1;
// }

//1.16
// fn c3mmmc3(a:u32, b:u32, c:u32) -> [u32; 3] {
//     let r = a.min(b.min(c));
//     let mut arr: [u32; 3] = [0; 3];

//     let mut nr:u32;
//     let mut index = 0;
//     for i in r..=a*b*c*3 {
//         let x = i as u32;
//         if i%a==0 && i%b==0 && i%c==0 {
//             nr = x;
//             arr[index]=nr;
//             index+=1;
//             if index>=3 {
//                 break;
//             }
//         }
//     }
//     return arr;
// }

//1.17
// fn triunghi(a:u32, b:u32, c:u32) -> bool {
//     if a+b>c && a+c>b && b+c>a {
//         return true
//     }
//     return false;
// }

//1.18
// fn media3(a:u32, b:u32, c:u32) -> f32 {
//     let med = ((a as f32)+(b as f32)+(c as f32))/3.0;
//     return med;
// }

//1.19
// fn med3(mut a:u32, mut b:u32, mut c:u32) -> u32 {
//     if a<b {
//         let temp = a;
//         a=b;
//         b=temp;
//     }
//     if b<c {
//         let temp = b;
//         b=c;
//         c=temp;
//     }
//     if a<b {
//         let temp = a;
//         a=b;
//         b=temp;
//     }
//     return b;
// }

//1.20
fn if2prime(a:u32, b:u32, c:u32) -> bool {
    let mut rs=0;
    let mut nr = 0;
    for i in 2..a {
        if a%i!=0 {
            continue;
        } else {
            nr+=1;
        }
    }
    if a==2 || a==3 {
        nr=0;
    } else if a==1 {
        nr=1;
    }
    if nr==0 {
        rs+=1;
    }

    nr = 0;
    for i in 2..b {
        if b%i!=0 {
            continue;
        } else {
            nr+=1;
        }
    }
    if b==2 || b==3 {
        nr=0;
    } else if b==1 {
        nr=1;
    }
    if nr==0 {
        rs+=1;
    }

    nr = 0;
    for i in 2..c {
        if c%i!=0 {
            continue;
        } else {
            nr+=1;
        }
    }
    if c==2 || c==3 {
        nr=0;
    } else if c==1 {
        nr=1;
    }
    if nr==0 {
        rs+=1;
    }

    if rs>=2 {
        return true;
    }
    return false

}

//main
fn main() {
    let a = introduce_u32("inrodu a:");
    let b = introduce_u32("inrodu b:");
    let c = introduce_u32("inrodu c:");
    //1
    //let sum = suma(a, b);
    //println!("{}", sum);
    //2
    //let prod = produs(a, b);
    //println!("{}", prod);
    //3
    // let med = media(a, b);
    // println!("{}", med);
    //4
    // let div = divizor_mare2(a, b);
    // println!("{}", div);
    //5
    //let cmmmc = multiplu_mic2(a, b);
    //println!("{}", cmmmc);
    //6
    // let mic = mai_mic(a, b);
    // println!("{}", mic);
    //7
    // let mare = mai_mare(a, b);
    // println!("{}", mare);
    //8
    // let nr = care_se_contin(a, b);
    // println!("{}", nr);
    //9
    // let numere = in_primul(a, b);
    // println!("{:?}", numere);
    //10
    // let numere = numar_maxim(a, b);
    // println!("{:?}", numere);
    //11
    // let numere = div_mare3(a, b, c);
    // println!("{:?}", numere);
    //12
    // let cmmmc3 = cmmmc3(a, b, c);
    // println!("{}", cmmmc3);
    //13
    // let da = mai_mare3(a, b, c);
    // println!("{}", da);
    //14
    // let da = mai_mic3(a, b, c);
    // println!("{}", da);
    //15
    // let numere = div_3(a, b, c);
    // println!("{:?}", numere);
    //16
    // let numere = c3mmmc3(a, b, c);
    // println!("{:?}", numere);
    //17
    // let numere = triunghi(a, b, c);
    // println!("{}", numere);
    //18
    // let numere = media3(a, b, c);
    // println!("{}", numere);
    //19
    // let numere = med3(a, b, c);
    // println!("{}", numere);
    //20
    let numere = if2prime(a, b, c);
    println!("{}", numere);
}