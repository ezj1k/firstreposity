use lab1::Book;
use lab1::line_to_vars;
use std::fs;

fn main() {
    let content = fs::read_to_string("poem.txt").expect("Cant read file!");
    let mut books: Vec<Book> = Vec::new();

    for line in content.lines() {
        books.push(line_to_vars(line));
    }

    books[0].borrow();
    books[0].info();
    books[1].info();
    books[2].info();
    books[0].return_book();
    books[0].info();
    books[1].info();
    books[2].info();
}