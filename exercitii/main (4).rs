//EX7
// struct Stiva<T> {
//     nums: Vec<T>,
// }

// impl<T: std::fmt::Debug> Stiva<T> {
//     fn new() -> Self {
//         Stiva { nums: Vec::new()}
//     }

//     fn push(&mut self, el: T) {
//         self.nums.push(el);
//     }

//     fn pop(&mut self) -> Option<T> {
//         self.nums.pop()
//     }

//     fn peek(&self) {
//         match self.nums.last() {
//             Some(val) => {
//                 println!("last e = {}", val);
//             },
//             None => println!("e goala stiva"),
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.nums.is_empty()
//     }

//     fn print(&self) {
//         if self.is_empty() {
//             print!("stiva e goala");
//         } else {
//             for el in &self.nums {
//                 print!("{:?} ", el);
//             }
//         }
//         println!("");
//     }
// }

// fn main() {
//     let mut sir = Stiva::new();

//     println!("{:?}",sir.is_empty());
//     sir.push(10);
//     sir.push(20);
//     sir.push(40);
//     sir.print();
//     sir.peek();
//     sir.pop();
//     sir.print();
//     println!("{:?}",sir.is_empty());
// }