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

    pub fn peek_node(&self) -> i32 {
        let last = self.stiva.len() - 1;
        println!("Last element in vector is {}!", self.stiva[last]);
        self.stiva[last]
    }

    pub fn pop_node(&mut self) {
        if !self.is_empty_node() {
            self.stiva.pop();
            println!("Last element was eliminated from vector!");
        }
    }
}