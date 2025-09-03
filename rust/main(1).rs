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

//1-100 se imparte exact la 6
/*fn main() {
    for i in 1..101 {
        if i%6==0 {
            println!("{},",i);
        }
    }
}*/

//a cu n zerouri
/*fn main() {
    let mut a = introduce("introdu a:");
    let mut x = introduce("introdu x:");

    while a<0 {
        println!{"incorect a"}
        a = introduce("introdu a:");
    }

    while x<1 {
        println!{"incorect x"}
        x = introduce("introdu x:");
    }
    let n = 10_u32.pow(x as u32);
    println!("{}",a*(n as i32));

    print!("{}",a);
    for i in 1..x+1 {
        print!("0");
    }
}*/

//tabla inmultirii
/*fn main() {
    let mut a = introduce("introdu a:");

    while a<0 || a>10 {
        println!{"incorect a"}
        a = introduce("introdu a:");
    }

    for i in 1..11 {
        println!("{}*{}={}",i,a,i*a);
    }
}*/

//toti divizirii
/*fn main() {
    let mut a = introduce("introdu a:");

    while a<1 {
        println!{"incorect a"}
        a = introduce("introdu a:");
    }

    for i in 1..a+1 {
        if a%i==0 {
            println!("{}",i);
        }
    }
}*/

//daca este prim
/*fn main() {
    let mut a = introduce("introdu a:");

    while a<1 {
        println!{"incorect a"}
        a = introduce("introdu a:");
    }

    let mut x = 0;

    if a==1 || a==2 || a==3 {
        println!("e prim");
    } else {
        for i in 4..a {
            if a%i==0 {
                println!("nu e prim");
                break;
            } else {
                x+=1;
            }
        }
    }
    if x!=0 {
        println!("e prim");
    }
}*/

//ascensor arata etajele parcurse
/*fn main() {
    let mut a = introduce("introdu a:");
    let mut b = introduce("introdu b:");

    while a<1 {
        println!{"incorect a"}
        a = introduce("introdu a:");
    }

    while b<1 {
        println!{"incorect b"}
        b = introduce("introdu b:");
    }

    if a<b {
        for i in a..b+1 {
            println!("{}",i);
        }
    } else if a==b {
        println!("{}",a);
    } else {
        for i in (b..=a).rev() {
            println!("{}",i);
        }
    }
}*/

//n numere se divid cu a sau b si mai mici ca c
/*fn main() {
    let a = introduce("introdu a:");
    let b = introduce("introdu b:");
    let c = introduce("introdu c:");

    println!{"introdu n:"}
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: usize = input.trim().parse().unwrap();

    while n<1 {
        println!{"incorect n"}
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => n = num,
            Err(_) => {
                println!("nr invalid. incearca din nou.");
                continue;
            }
        };
    }

    let mut vec = Vec::new();

    for i in 0..n {
        let mut x = introduce("introdu x:");

        while x<1 {
            println!{"incorect x"}
            x = introduce("introdu x:");
        }

        if (x%a==0 || x%b==0) && x<c {
            vec.push(x);
            arr[i]=x;
        }
    }
    
    println!("{:?}", vec);
}*/

//determin sumele n, 2n, 2n-1, (n-1)*n, 1*...*n
/*fn main() {
    let mut a = introduce("introdu a:");

    while a<1 {
        println!{"incorect a"}
        a = introduce("introdu a:");
    }

    let mut s1 = 0;
    let mut s2 = 0;
    let mut s3 = 0;
    let mut s4 = 0;
    let mut s5 = 0;
//----------------s1
    for i in 1..a+1 {
        s1+=i;
    }
    println!("{};",s1);
//-----------------s2
    for i in 1..a+1 {
        s2+=2*i-1;
    }
    println!("{};",s2);
//-------------------s3
    for i in 1..a+1 {
        s3+=i*2;
    }
    println!("{};",s3);
//--------------------s4
    for i in 1..a+1 {
        s4+=(i-1)*i;
    }
    println!("{};",s4);
//-------------------s5
    let mut x;
    let mut f;
    for i in 1..a+1 {
        f = i;
        x=1;
        while f!=0 {
            x*=f;
            f-=1;
        }
            s5+=x
    }
    println!("{};",s5);
}*/

//temp medie
/*fn main() {
    let mut vec = [0;12];
    
    for i in 0..vec.len() {
        vec[i] = introduce("introdu i:");
    }

    let mut med = 0;
    let mut med_poz = 0;
    let mut med_neg = 0;

    for i in vec {
        med+=i;
    }
    println!("{:.2}", (med as f32)/12.0);

    let mut f = 0.0;
    for i in vec {
        if i>0 {
            med_poz+=i;
            f+=1.0
        }
    }
    println!("{:.2}", (med_poz as f32)/f);

    f = 0.0;
    for i in vec {
        if i<0 {
            med_neg+=i;
            f+=1.0
        }
    }
    println!("{:.2}", (med_neg as f32)/f);
}*/

//media min si max, corigenta
/*fn main() {
    let mut n = introduce("intronu nr de elevi");

    while n<1 {
        println!{"incorect n"}
        n = introduce("introdu n:");
    }

    let mut vec = Vec::new();
    let mut max = f32::MIN;
    let mut min = f32::MAX;
    
    for _i in 0..n {
        let mut m = introduce_float("intronu nota");

        while m<2.0 || m>10.00{
            println!{"incorect m"}
            m = introduce_float("introdu m:");
        }

        vec.push(m);

        if m>=max {
            max = m;
        }

        if m<=min {
            min = m;
        }
    }

    for i in vec {
        if i<5.00 {
            println!("{:.} - corigent", i);
        } else {
            println!("{:.} - nu e corigent", i);
        }
    }

    println!("min - {:.}", min);
    println!("max - {:.}", max);
}*/

//cmmmc si cmmdc
/*fn main() {
    let mut n = introduce("intronu nr:");
    let mut m = introduce("intronu nr2:");
    let mut cmmmc = 0;
    let mut cmmdc = 0;

    while n==0 {
        println!{"incorect n"}
        n = introduce("introdu n:");
    }

    while m==0 {
        println!{"incorect m"}
        m = introduce("introdu m:");
    }

    if m>n {
        let temp = n;
        n=m;
        m=temp;
    }

    for i in 1..n+1 {
        if n%i==0 && m%i==0 {
            cmmdc=i;
        }
    }

    for i in n..n*m+1 {
        if i%n==0 && i%m==0 {
            cmmmc=i;
            break;
        }
    }

    println!("cmmdc = {}; cmmmc = {}", cmmdc, cmmmc);
}*/

//cmmdc 3 nr
/*fn main() {
    let mut n = introduce("intronu nr:");
    let mut m = introduce("intronu nr2:");
    let mut s = introduce("intronu nr3:");
    let mut cmmdc = 0;

    while n==0 {
        println!{"incorect n"}
        n = introduce("introdu n:");
    }

    while m==0 {
        println!{"incorect m"}
        m = introduce("introdu m:");
    }

    while s==0 {
        println!{"incorect s"}
        s = introduce("introdu s:");
    }

    if m>n && n<s {
        if m>s {
            let temp = n;
            n=m;
            m=temp;
        } else {
            let temp = n;
            n=s;
            s=temp;
        }
    }

    for i in 1..n+1 {
        if n%i==0 && m%i==0 && s%i==0 {
            cmmdc=i;
        }
    }

    println!("cmmdc = {};", cmmdc);
}*/

//4 nr - numarator si numitor al fractiei 
/*fn main() {
    let mut n = introduce("intronu nr:");
    let mut m = introduce("intronu nr2:");
    let mut s = introduce("intronu nr3:");
    let mut a = introduce("intronu nr4:");

    while n==0 {
        println!{"incorect n"}
        n = introduce("introdu n:");
    }

    while m==0 {
        println!{"incorect m"}
        m = introduce("introdu m:");
    }

    while s==0 {
        println!{"incorect s"}
        s = introduce("introdu s:");
    }

    while a==0 {
        println!{"incorect a"}
        a = introduce("introdu a:");
    }

    println!("rezultat; {}/{}", n*a+s*m, a*m);

    let mut z = n*a+s*m;
    let mut x = a*m;
    let mut cmmdc = 1;

    if z>x {
        let temp = z;
        z=x;
        x=temp;
    }

    for i in 1..x+1 {
        if x%i==0 && z%i==0 {
            cmmdc=i;
        }
    }

    println!("rezultat; {}/{}", (n*a+s*m)/cmmdc, (a*m)/cmmdc);
}*/

//dreptunghi m pe n
/*fn main() {
    let mut n = introduce("intronu nr:");
    let mut m = introduce("intronu nr2:");

    while n<0 || n>=10 {
        println!{"incorect n"}
        n = introduce("introdu n:");
    }

    while m<0 || m>=10 {
        println!{"incorect m"}
        m = introduce("introdu m:");
    }

    for i in 1..m+1 {
        for j in 1..n+1 {
            print!("{}",i);
        }
        println!("");
    }

    println!("");

    for i in 1..m+1 {
        for j in 1..n+1 {
            print!("{}",j);
        }
        println!("");
    }
}*/

//nr succesiv pana nu e 0 si suma celor pare
/*fn main() {
    let mut x = 0;
    let mut n;
    loop {
        n = introduce("introdu n:");

        while n<0 {
            println!{"incorect n"}
            n = introduce("introdu n:");
        }

        if n%2==0 && n!=0 {
            x+=n;
        } else if n==0 {
            break;
        }
    }
    println!{"x: {}",x};
}*/

//nr succesiv pana nu e 0 si suma celor pare
/*fn main() {
    let mut x = 0;
    let mut n;
    loop {
        n = introduce("introdu n:");

        while n<0 {
            println!{"incorect n"}
            n = introduce("introdu n:");
        }

        if n%3==0 && n!=0 {
            x+=n;
        } else if n==0 {
            break;
        }
    }
    println!{"x: {}",x};
}*/

//triunghi din nr
/*fn main() {
    let mut n = introduce("introdu n:");

    while n<=0 || n>=10 {
        println!{"incorect n"};
        n = introduce("introdu n:");
    }

    for i in 1..n+1 {
        for j in 1..i+1 {
            print!("{}",j);
        }
        println!("");
    }

    println!("");

    for i in (1..n+1).rev() {
        for j in 1..i+1 {
            print!("{}",j);
        }
        println!("");
    }

    println!("");

    while n!=0 {
        for j in 1..n+1 {
            print!("{}",j);
        }
        println!("");
        n-=1;
    }
}*/

//nr impar div cu 3, suma pare si imp
/*fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut n;
    loop {
        n = introduce("introdu n:");

        if n%2==1 && n%3==0 {
            y+=n;
            break;
        } else if n%2==0 {
            x+=n;
        } else if n%2==1 {
            y+=n;
        }
    }
    println!{"x: {}",x};
    println!{"y: {}",y};
}*/

//nr div cu 5, suma pare si imp
/*fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut nr = 0;
    let mut n;
    loop {
        n = introduce("introdu n:");
        nr+=1;
        if n%5==0 {
            y+=n;
            break;
        } else if n%2==0 {
            x+=n;
        } else if n%2==1 {
            y+=n;
        }
    }
    println!{"x: {}",x};
    println!{"y: {}",y};
    println!{"nr: {}",nr};
}*/

//fatfrumos merge
/*fn main() {
    let mut n = introduce("intronu nr:");
    let mut m = introduce("intronu nr2:");
    let mut s = introduce("intronu nr3:");
    let mut a=0;

    while n<=0 {
        println!{"incorect n"}
        n = introduce("introdu n:");
    }

    while m<=0 {
        println!{"incorect m"}
        m = introduce("introdu m:");
    }

    while s<=0 {
        println!{"incorect s"}
        s = introduce("introdu s:");
    }

    while n>0 {
        n-=m;
        if n>0 {
            n+=s;
            a+=1;
        }
    }

    println!("{} zile", a);
}*/

//nr pana suma celor pare e mai mare ca k
/*fn main() {
    let mut x = 0;
    let mut nr=0;
    let mut y = introduce("introdu y:");
    let mut n;
    println!{"y: {}",y};
    loop {
        n = introduce("introdu n:");
        nr+=1;
        if n%2==1 {
            x+=n;
        }
        if x>y {
            break;
        }
    }
    println!{"x: {}",x};
    println!{"nr: {}",nr};
}*/

//min max nr pana la 1k
/*fn main() {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut y = 0;
    println!{"y: {}",y};
    loop {
        let n = introduce("introdu n:");
        y+=n;
        if n<min {
            min = n;
        }
        if n>max {
            max=n;
        }
        if y>1000 {
            break;
        }
    }
    println!{"min: {}",min};
    println!{"max: {}",max};
}*/

//min max pana suma divizibila cu 3
/*fn main() {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut y = 0;
    println!{"y: {}",y};
    loop {
        let n = introduce("introdu n:");
        y+=n;
        if n<min {
            min = n;
        }
        if n>max {
            max=n;
        }
        if y%3==0 {
            break;
        }
    }
    println!{"min: {}",min};
    println!{"max: {}",max};
}*/

//ex25
/*fn main() {
    let mut count=0;
    let mut summa=0;
    let mut n = introduce("Introdu n:");

    while n<2 {
        println!{"incorect"};
        n = introduce("Introdu n:");
    }

    for _ in 1..=n {
        let mut x=introduce("Introdu x:");
        let x2=x;
        let mut y = 0;
        
        while x!=0 {
            y+=x%10;
            x/=10;
        }

        if y%2==0 {
            summa+=x2;
            count+=1;
        }
    }
    let med:f32 = (summa as f32)/(count as f32);
    println!{"suma={}, medi={}", summa,med};
}*/

//ex26
/*fn main() {
    let mut count=0;
    let mut summa=0;
    let mut n = introduce("Introdu n:");

    while n<2 {
        println!{"incorect"};
        n = introduce("Introdu n:");
    }

    for _ in 1..=n {
        let mut x=introduce("Introdu x:");
        let x2=x;
        let mut y = 0;
        
        while x!=0 {
            y+=x%10;
            x/=10;
        }

        if y%3!=0 {
            summa+=x2;
            count+=1;
        }
    }
    let med:f32 = (summa as f32)/(count as f32);
    println!{"suma={}, medi={}", summa,med};
}*/

//ex27
/*fn main() {
    let mut n = introduce("Introdu n:");

    while n<=2 {
        println!("incorect!");
        let n = introduce("Introdu n:");
    }

    let mut aria_max = i32::MIN;
    let mut per_min = i32::MAX;
    let mut per_ar=0;
    let mut aria_per=0;
    let mut h_ar=0;
    let mut w_ar=0;
    let mut h_per=0;
    let mut w_per=0;

    for _ in 1..=n {
        let h = introduce("Introdu h:");
        let w = introduce("Introdu w:");

        let aria = h*w;
        let per = 2*h+2*w;

        if aria > aria_max {
            aria_max = aria;
            h_ar = h;
            w_ar = w;
            per_ar = per;
        }
        if per < per_min {
            per_min = per;
            h_per = h;
            w_per = w;
            aria_per = aria;
        }
        n-=1;
    }
    println!("aria max: h={}, w={}, ar={}, per={}", h_ar, w_ar, aria_max, per_ar);
    println!("per min: h={}, w={}, ar={}, per={}", h_per, w_per, aria_per, per_min);
}*/

//ex28
/*fn main() {
    let mut n = introduce("Introdu n:");

    while n<=2 {
        println!("incorect!");
        let n = introduce("Introdu n:");
    }

    let mut aria_max = i32::MIN;
    let mut ip_min = i32::MAX;
    let mut per_ar=0;
    let mut per_ip=0;
    let mut h_ar=0;
    let mut w_ar=0;
    let mut ip_ar=0;
    let mut aria_ip=0;
    let mut h_ip=0;
    let mut w_ip=0;

    for _ in 1..=n {
        let h = introduce("Introdu h:");
        let w = introduce("Introdu w:");
        let ip_f = ((h*h+w*w) as f32).sqrt();
        let ip = ip_f as i32;

        let aria = h*w/2;
        let per = h+w+ip;

        if aria > aria_max {
            aria_max = aria;
            h_ar = h;
            w_ar = w;
            ip_ar = ip;
            per_ar = per;
        }
        if ip < ip_min {
            ip_min = ip;
            h_ip = h;
            w_ip = w;
            aria_ip = aria;
            per_ip = per;
        }
        n-=1;
    }
    println!("aria max: h={}, w={}, ip={}, ar={}, per={}", h_ar, w_ar, ip_ar, aria_max, per_ar);
    println!("per min: h={}, w={}, ip={}, ar={}, per={}", h_ip, w_ip, ip_min, aria_ip, per_ip);
}*/

//ex29
/*fn main() {
    for c in 1i32..21 {
        for i in 1i32..=20 {
            for j in i+1i32..21 {
                if i.pow(2)+j.pow(2)==c.pow(2) {
                    println!("a={}, b={}, c={};",i,j,c);
                }
            }
        }
    }
}*/

//ex30
fn main() {
    for c in 1i32..21 {
        for i in 1i32..=20 {
            for j in i+1i32..21 {
                if (i+j).pow(3)==c.pow(3) {
                    println!("a={}, b={}, c={};",i,j,c);
                }
            }
        }
    }
}