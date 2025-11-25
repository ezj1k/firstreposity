const PI: f64 = 3.141598;

struct Rectangle {
    width: u32,
    height: u32,
}

struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        2 * self.width + 2 * self.height
    }
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }

    fn perimeter(&self) -> f64 {
        self.radius * 2.0 * PI
    }
}

impl Triangle {
    fn new(base: f64, height: f64) -> Triangle {
        Triangle { base, height }
    }

    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }

    fn perimeter(&self) -> f64 {
        self.base * 3.0
    }
}

fn compare_areas(x: u32, y: f64, z: f64) {
    if (x as f64) == y && (x as f64) == z {
        println!("toate 3 figuri au arie asemenea");
    } else if ((x as f64) > y && y >= z) || ((x as f64) > z && z >= y) {
        println!("Dreptunghiul are arie mai mare");
    } else if (y > (x as f64) && (x as f64) >= z) || (y > z && z >= (x as f64)) {
        println!("Cercul are arie mai mare");
    } else {
        println!("Triunghiul are arie mai mare");
    }
}

pub fn main() {
    let dreptunghi = Rectangle::new(10, 20);
    let cerc = Circle::new(10.0);
    let triunghi = Triangle::new(10.0, 8.66);

    println!(
        "Dreptunghi area: {}, Dreptungi perimeter: {}",
        dreptunghi.area(),
        dreptunghi.perimeter()
    );
    println!(
        "Cerc area: {}, Cerc perimeter: {}",
        cerc.area(),
        cerc.perimeter()
    );
    println!(
        "Triunghi area: {}, Triunghi perimeter: {}",
        triunghi.area(),
        triunghi.perimeter()
    );

    compare_areas(dreptunghi.area(), cerc.area(), triunghi.area());
}
