use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_borrowed: bool,
    published_year: String,
    tags: Vec<String>,
}

#[derive(Debug)]
struct LibraryManager {
    books: HashMap<u32, Book>,
    next_id: u32,
}

impl Book {
    fn new<S: Into<String>>(title: S, author: S, published_year: S, tags: Vec<String>) -> Self {
        let var_name = title.into();
        let title = var_name;
        let author = author.into();
        let published_year = published_year.into();
        Book {
            title,
            author,
            published_year,
            is_borrowed: false,
            tags,
        }
    }

    fn describe(&self) {
        let status = if self.is_borrowed {
            "borrowed"
        } else {
            "available"
        };
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Published Year: {}", self.published_year);
        println!("Status: {}", status);
        if !self.tags.is_empty() {
            println!("Tags: {}", self.tags.join(", "));
        }
    }
}

impl LibraryManager {
    fn new() -> Self {
        LibraryManager {
            books: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_book<S: Into<String>>(&mut self, title: S, author: S, published_year: S) {
        let book = Book::new(title, author, published_year, Vec::new());
        self.books.insert(self.next_id, book);
        self.next_id += 1;
    }

    fn list_books(&self) {
        if self.books.is_empty() {
            println!("(no books added)");
            return;
        }
        for (id, book) in &self.books {
            println!("Id: {}", id);
            book.describe();
        }
    }

    fn get_book(&self, id: u32) -> Option<&Book> {
        self.books.get(&id)
    }

    fn filter_books_by_tag(&self, tag: &String) -> Vec<&Book> {
        self.books
            .values()
            .filter(|book| book.tags.contains(tag))
            .collect()
    }

    fn update_book<S: Into<String>>(
        &self,
        id: u32,
        new_title: Option<S>,
        new_author: Option<S>,
        published_year: Option<S>,
    ) {
    }

    fn borrow_book() {}

    fn return_book() {}

    fn add_tag() {}

    fn remove_tag() {}

    fn delete_book() {}

    fn statistics() {}
}

fn main() {}
