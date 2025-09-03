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

fn introduce_float(mesaj: &str) -> f32 {
    let mut numar = String::new();

    println!("{}", mesaj);
    io::stdin().read_line(&mut numar).expect("eroare de citire");

    let numar: f32 = match numar.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("eroare");
            std::process::exit(1);
        }
    };
    return numar;
}

//fiecare 5 pozitii
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _i in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut j:usize=0;
    while j<=v.len() {
        println!("{}",v[j]);
        j+=5;
    }
}*/

//fiecare 4 pozitii in ordine opusa
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut j=(x-1) as usize;
    while j>=0 {
        println!("{}",v[j]);
        j-=4;
    }
}*/

//doar pare si impare
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    println!("pare");
    for i in 0..x as usize {
        if v[i]%2==0 {
            println!("{}",v[i]);
        }
    }

    println!("impare");
    for i in 0..x as usize {
        if v[i]%2==1 {
            println!("{}",v[i]);
        }
    }
}*/

//mai mari ca x si %y==0 | mai mari ca x si mai mici ca y
/*fn main() {
    let mut x = introduce("introdu nr");
    let a = introduce("introdu nr");
    let b = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    println!("1.5");
    for i in 0..x as usize {
        if v[i]>a && v[i]%b==0 {
            println!("{}",v[i]);
        }
    }

    println!("1.6");
    for i in 0..x as usize {
        if v[i]>a && v[i]<b {
            println!("{}",v[i]);
        }
    }
}*/

//pozitiile impare ale nr negative
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    for i in 0..x as usize {
        if v[i]<0 && i%2==1 {
            println!("{}",v[i]);
        }
    }
}*/

//media nr pare
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut suma=0;
    let mut count=0;
    for i in 0..x as usize {
        if v[i]%2==0 {
            suma+=v[i];
            count+=1;
            println!("{}",v[i]);
        }
    }
    let mut media:f32 = (suma as f32)/(count as f32);
    println!("{}",media);
}*/

//pozitiile nr din 2 cifre
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    for i in 0..x as usize {
        if v[i]/10>0 {
            println!("{}",i);
        }
    }
}*/

//suma nr div cu 3
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut suma=0;
    for i in 0..x as usize {
        if v[i]%3==0 {
            suma+=v[i];
            println!("{}",v[i]);
        }
    }
    println!("{}",suma);
}*/

//cele mai mari 2 nr
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut coef=0;
    let mut mare1=i32::MIN;
    let mut mare2=i32::MIN;
    for i in 0..x as usize {
        if v[i]>=mare1 {
            mare1=v[i];
            coef = i;
        }
    }
    v.remove(coef as usize);
    for i in 0..(x-1) as usize {
        if v[i]>=mare2 {
            mare2=v[i];
        }
    }
    println!("{},{}",mare1,mare2);
}*/

//cele mai mari 2 nr (2)
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut coef=0;
    let mut mare1=i32::MIN;
    let mut mare2=i32::MIN;
    for i in 0..x as usize {
        if v[i]>=mare1 {
            mare2=mare1;
            mare1=v[i];
        }
    }
    println!("{},{}",mare1,mare2);
}*/

// toate in afara de cel mai mare si cel mai mic
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut coef=0;
    let mut mare=i32::MIN;
    let mut mic=i32::MAX;
    for i in 0..x as usize {
        if v[i]>=mare {
            mare=v[i];
        }
        if v[i]<=mic {
            mic=v[i];
        }
    }
    for i in 0..x as usize {
        if v[i]==mare || v[i]==mic {
            continue;
        }
        println!("{}",v[i]);
    }
}*/

//toate in afara de cel mai mare si cel mai mic (2)
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut coef=0;
    let mut mare1=i32::MIN;
    let mut mic=i32::MAX;
    for i in 0..x as usize {
        if v[i]>=mare1 {
            mare1=v[i];
            coef = i;
        }
    }
    v.remove(coef as usize);
    for i in 0..(x-1) as usize {
        if v[i]<=mic {
            mic=v[i];
            coef=i;
        }
    }
    v.remove(coef as usize);
    for i in 0..(x-2) as usize {
        println!("{}",v[i]);
    }
}*/

//+10
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<5 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    for i in 0..x as usize {
            v[i]+=10;
            println!("{}",v[i]);
    }
}*/

//*2 pare | *3 impare
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<1 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    for i in 0..x as usize {
        if v[i]%2==0 {
            v[i]*=2;
        } else if v[i]%2==1 {
            v[i]*=3;
        }
    }
    for i in 0..x as usize {
        println!("{}",v[i]);
    }
}*/

//primul si cel mai mic de schimbat
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<1 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut coef=0;
    let mut mic=i32::MAX;
    let mut primul = v[0];
    for i in 0..(x-1) as usize {
        if v[i]<=mic {
            mic=v[i];
            coef=i;
        }
    }
    v[0]=mic;
    v[coef as usize]=primul;
    for i in 0..x as usize {
        println!("{}",v[i]);
    }
}*/

//al treilea si maximal de schimbat
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<1 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    let mut coef=0;
    let mut mare=i32::MIN;
    let mut treilea = v[2];
    for i in 0..(x-1) as usize {
        if v[i]>=mare {
            mare=v[i];
            coef=i;
        }
    }
    v[2]=mare;
    v[coef as usize]=treilea;
    for i in 0..x as usize {
        println!("{}",v[i]);
    }
}*/

//cu 3 divizori de afisat si toata.
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<1 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    for i in 0..(x-1) as usize {

        let mut count =0;

        if v[i] < 4 {
                continue;
            }

        let mut rad = (v[i] as f32).sqrt() as i32;
        if rad*rad!=v[i] {
            continue;
        } else {
            for j in 2..=rad {
                if v[i]%j==0 {
                    count+=1;
                    if count>1 {
                        break;
                    }
                }
            }
        }
        if count ==1 {
            println!("{}",v[i]);
        }
    }

    println!("---------------");

    for i in 0..x as usize {
        println!("{}",v[i]);
    }
}*/

//-12 la fiecare
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<1 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    for i in 0..x as usize {
        v[i]-=12;
        println!("{}",v[i]);
    }
}*/

//*2 mai mici ca 0 si -15 if >10 
/*fn main() {
    let mut x = introduce("introdu nr");

    while x>1000 || x<1 {
        println!("gresit");
        x = introduce("introdu nr");
    }

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..x {
        v.push(introduce("introdu nr"));
    }

    for i in 0..x as usize {
        if v[i]<0 {
            v[i]*=2;
        } else if v[i]>10 {
            v[i]-=15;
        }
        println!("{}",v[i]);
    }
}*/

//--------------------------------------------------------------------------------------------------------------------------------------

//de facut bidimensional
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    for i in 0..n {
        for j in 0..m {
            println!("{}",v[i][j]);
        }
    }
}*/

//cate nr parte <10
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    let mut count=0;
    for i in 0..n {
        for j in 0..m {
            if v[i][j]<10 && v[i][j]%2==0 {
                count+=1;
            }
        }
    }
    println!("{}",count);
}*/

//dintro cifra
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    let mut count=0;
    for i in 0..n {
        for j in 0..m {
            if v[i][j]/10==0 {
                count+=1;
            }
        }
    }
    println!("{}",count);
}*/

//div cu 3 inlocuim cu max*3
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    let mut max=i32::MIN;
    for i in 0..n {
        for j in 0..m {
            if v[i][j]>max {
                max=v[i][j];
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if v[i][j]%3==0 {
                v[i][j]=max*3;
                println!("{}",v[i][j]);
            }
        }
    }
}*/

//pare cu max*2
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    let mut max=i32::MIN;
    for i in 0..n {
        for j in 0..m {
            if v[i][j]>max {
                max=v[i][j];
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if v[i][j]%2==0 {
                v[i][j]=max*2;
                println!("{}",v[i][j]);
            }
        }
    }
}*/

//coloana 2 si linia 1
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    for i in 0..=0 {
        for j in 0..m {
            println!("{}",v[i][j]);
        }
    }
    println!("-------");
    for i in 0..n {
        for j in 1..=1 {
            println!("{}",v[i][j]);
        }
    }
}*/

//2.8
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");
    let mut a = introduce_u("introdu linia");
    let mut b = introduce_u("introdu coloana");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }
    while a>10 || a<1 {
        println!("gresit");
        a = introduce_u("introdu linia");
    }
    while b>10 || b<1 {
        println!("gresit");
        b = introduce_u("introdu coloana");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

        for j in 0..m {
            println!("{}",v[a][j]);
        }
    println!("-------");
    for i in 0..n {
            println!("{}",v[i][b]);
    }
}*/

//2.9
/*fn main() {
    let mut n = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; n]; n];

    let mut x=0;
    for i in 0..n {
        for j in 0..n {
            x+=2;
            v[i][j]=x;
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.10
/*fn main() {
    let mut n = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; n]; n];

    let mut x=-1;
    for i in 0..n {
        for j in 0..n {
            x+=2;
            v[i][j]=x;
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.11
/*fn main() {
    let mut n = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; n]; n];

    let mut x=0;
    for i in 0..n {
        for j in 0..n {
            x+=5;
            v[i][j]=x;
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.12
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut v1: Vec<i32> = Vec::new();

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    for i in 0..n {
        let mut max=i32::MIN;
        for j in 0..m {
            if v[i][j]>max {
                max=v[i][j];
            }
        }
        v1.push(max);
    }
    v.push(v1);

    for i in 0..=n {
        for j in 0..m {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.13
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut v1: Vec<i32> = Vec::new();

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    for j in 0..m {
        let mut nr=0;
        for i in 0..n {
            nr+=v[i][j];
        }
        v1.push(nr);
    }
    v.push(v1);

    for i in 0..=n {
        for j in 0..m {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.14
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut v1: Vec<i32> = Vec::new();

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    for j in 0..m {
        let mut nr=0;
        for i in 0..2 {
            nr+=v[i][j];
        }
        v1.push(nr);
    }
    v.push(v1);

    for i in 0..=n {
        for j in 0..m {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.15
/*fn main() {
    let mut n = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..n {
            v[i][j] = introduce("introdu nr");
        }
    }

    for i in 0..n {
        v[i][i]+=5;
    }

    for i in 0..=n {
        for j in 0..m {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.16
/*fn main() {
    let mut n = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; n]; n];
    let mut v1: Vec<i32> = Vec::new();

    for i in 0..n {
        for j in 0..n {
            v[i][j] = introduce("introdu nr");
        }
    }

    let mut index = 0;
    for i in 0..n {
        let mut min=i32::MAX;
        for j in 0..n {
            if v[i][j]<min {
                min=v[i][j];
                index=i;
            }
        }
    }

    
    
    for j in 0..n {
        v1.push(v[index][j]);
    }
    v.push(v1);

    for i in 0..=n {
        for j in 0..n {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.17
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut b = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while b>10 || b<1 {
        println!("gresit");
        b = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            v[i][j] = introduce("introdu nr");
        }
    }

    for i in 0..n {
            v[i][b]+=15;
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.18
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");
    let mut a = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }
    while a>10 || a<1 {
        println!("gresit");
        a = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            v[i][j] = introduce("introdu nr");
        }
    }

    for j in 0..m {
            v[a][j]+=10;
    }

    for i in 0..n {
        for j in 0..m {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.19
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut v1: Vec<i32> = Vec::new();

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    let mut sumada=0;
    let mut index=0;
    for i in 0..n {
        let mut suma=0;
        for j in 0..m {
            suma+=v[i][j];
        }
        if suma > sumada {
            sumada=suma;
            index=i;
        }
    }

    for j in 0..m {
        v1.push(v[index][j]);
    }
    v.push(v1);

    for i in 0..=n {
        for j in 0..m {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/

//2.20
/*fn main() {
    let mut n = introduce_u("introdu nr");
    let mut m = introduce_u("introdu nr");

    while n>10 || n<1 {
        println!("gresit");
        n = introduce_u("introdu nr");
    }
    while m>10 || m<1 {
        println!("gresit");
        m = introduce_u("introdu nr");
    }

    let mut v: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut v1: Vec<i32> = Vec::new();

    for i in 0..n {
        for j in 0..m {
            v[i][j] = introduce("introdu nr");
        }
    }

    let mut max=i32::MIN;
    let mut index=0;
    for i in 0..n {
        for j in 0..m {
            if v[i][j]>max {
                max=v[i][j];
                index=j;
            }
        }
    }

    for i in 0..n {
        v1.push(v[i][index]);
    }
    v.push(v1);

    for i in 0..=n {
        for j in 0..m {
            print!("{} ",v[i][j]);
        }
        println!("");
    }
}*/