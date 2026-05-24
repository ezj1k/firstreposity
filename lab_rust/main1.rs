struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

impl Book {
    fn new(title: String, author: String, pages: u32) -> Book {
        Book {
            title,
            author,
            pages,
            available: true,
        }
    }

    fn borrow(&mut self) -> bool {
        self.available = false;
        return self.available;
    }

    fn return_book(&mut self) -> bool {
        self.available = true;
        return self.available;
    }

    fn info(&self) {
        println!(
            "title: {}, author: {}, pages: {}, available: {}",
            self.title, self.author, self.pages, self.available
        );
    }
}

pub fn main() {
    let book1 = Book::new("Rust Programming".to_string(), "Autor1".to_string(), 350);
    let mut book2 = Book::new("Learning Rust".to_string(), "Autor2".to_string(), 280);
    let book3 = Book::new("Advanced Rust".to_string(), "Autor3".to_string(), 420);

    book2.borrow();

    book1.info();
    book2.info();
    book3.info();

    book2.return_book();
}
