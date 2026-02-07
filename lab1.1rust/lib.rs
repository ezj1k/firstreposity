use regex::Regex;

#[derive(Debug)]
pub struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

impl Book {
    pub fn new(title: String, author: String, pages: u32) -> Book {
        Book {
            title, author, pages, available: true,
        }
    }

    pub fn borrow(&mut self) {
        self.available = false;
        println!("{} de {} ({}) - nu mai este disponibila", self.title, self.author, self.pages);
    }

    pub fn return_book(&mut self) {
        self.available = true;
        println!("{} de {} ({}) - a devenit din nou disponibila", self.title, self.author, self.pages);
    }

    pub fn info(&self) {
        println!("{:?}", self);
    }
}

pub fn line_to_vars(line: &str) -> Book {
    let re = Regex::new(r#""(.+)" de (.+?) - (\d+) pagini"#).unwrap();

    let caps = re.captures(line).unwrap();

    let title = &caps[1];
    let author = &caps[2];
    let pages = &caps[3].parse::<u32>().unwrap();

    Book {
        title: title.to_string(), author: author.to_string(), pages: *pages, available: true,
    }
}