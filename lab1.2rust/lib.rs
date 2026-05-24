use std::f64::consts::PI;

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Circle {
    radius: f64,
}

pub struct Triangle {
    base: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width, height,
        }
    }

    pub fn area(&self) -> f64 {
        let result = (self.width * self.height) as f64;
        println!("Area of Rectangle: {}", result);
        result
    }

    pub fn perimeter(&self) -> f64 {
        let result = (2 * self.width + 2 * self.height) as f64;
        println!("Perimeter of Rectangle: {}", result);
        result
    }
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle {
            radius,
        }
    }

    pub fn area(&self) -> f64 {
        let result = self.radius * self.radius * PI;
        println!("Area of Circle: {}", result);
        result
    }

    pub fn perimeter(&self) -> f64 {
        let result = 2.0 * self.radius * PI;
        println!("Perimeter of Circle: {}", result);
        result
    }
}

impl Triangle {
    pub fn new(base: f64, height: f64,) -> Triangle {
        Triangle {
            base, height,
        }
    }

    pub fn area(&self) -> f64 {
        let result = 0.5 * self.base * self.height;
        println!("Area of Triangle: {}", result);
        result
    }

    pub fn perimeter(&self) -> f64 {
        let inlog = (self.base / 2.0) * (self.base / 2.0) + self.height * self.height;
        let result = self.base + 2.0 * inlog.log2();
        println!("Perimeter of Triangle: {}", result);
        result
    }
}

pub fn compare_areas(area_a: f64, area_b: f64) -> f64 {
    println!("Area 1: {:.2}", area_a);
    println!("Area 2: {:.2}", area_b);

    if area_a > area_b {
        println!("Primul obiect are aria mai mare");
        area_a
    } else if area_b > area_a {
        println!("Al doilea obiect are aria mai mare");
        area_b
    } else {
        println!("Ariile sunt egale");
        area_b
    }
}