pub struct Node {
    stiva: Vec<i32>,
}

impl Node {
    pub fn new() -> Node {
        Node {stiva: Vec::new(),}
    }

    pub fn is_empty_node(&self) -> bool {
        let lenght = self.stiva.len();
        if lenght == 0 {
            println!("Vector is empty!");
            return true;
        }
        println!("Vector has {} elements!", lenght);
        false
    }

    pub fn push_node(&mut self, num: i32) {
        self.stiva.push(num);
        println!("{} was added in vector!", num);
    }

    pub fn front_node(&self) -> i32 {
        println!("First element in vector is {}!", self.stiva[0]);
        self.stiva[0]
    }

    pub fn pop_node(&mut self) {
        let lenght = self.stiva.len();
        if !self.is_empty_node() {
            if lenght >= 3 {
                for i in 0..lenght-1 {
                    self.stiva[i] = self.stiva[i+1];
                }
                self.stiva.pop();
            } else if lenght == 2 {
                self.stiva[0] = self.stiva[1];
                self.stiva.pop();
            } else if lenght == 1 {
                self.stiva.pop();
            }
            println!("First element was eliminated from vector!");
        }
    }
}