/* mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn minus(a: i32, b: i32) -> i32 {
        a - b
    }
} */
mod math;

macro_rules! my_print {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

fn main() {
    my_print!("Hello, Mir");
    let res3 = math::add(10, 5);
    let res4 = math::minus(10, 5);
    println!("Sum {}; Minus {};", res3, res4);
    test();
    add(5, 6);
    let res1 = suma(10, 10);
    println!("Res {}", res1);
    let user = "Alex";
    let mut user2 = String::from("Alex");
    greet_user(user);
    greet_user2(&mut user2);
    println!("Name {}", user2); //va afisa Bob caci este de parca globala
    let res2 = mult(&(4, 5, 6));
    println!("res2 {}", res2);
}

fn test() {
    println!("Hello, world!");
}

fn suma(a: i32, b: i32) -> i32 {
    return a + b;
    //sau scriu pursimlu - a+b (fara ;)
}

fn add(a: i32, b: i32) {
    let res = a + b;
    println!("Res: {}", res);
}

fn greet_user(name: &str) {
    println!("Name: {}", name);
}

fn greet_user2(name: &mut String) {
    *name = String::from("Bob");
    println!("Name: {}", name);
}

fn mult(data: &(i32, i32, i32)) -> i32 {
    data.0 * data.1 * data.2
}
