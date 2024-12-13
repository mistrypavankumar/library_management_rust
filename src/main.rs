use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
    available: bool,
}

struct Library {
    books: HashMap<u32, Book>,
    borrowed_books: HashMap<u32, Book>, // Tracks borrowed books
    next_id: u32,
}

impl Library {
    fn new() -> Library {
        let mut library = Library {
            books: HashMap::new(),
            borrowed_books: HashMap::new(),
            next_id: 1,
        };

        // Pre-defined books
        library.add_book("The Rust Programming Language", "Steve Klabnik and Carol Nichols");
        library.add_book("Clean Code", "Robert C. Martin");
        library.add_book("The Pragmatic Programmer", "Andrew Hunt and David Thomas");
        library.add_book("Design Patterns", "Erich Gamma, Richard Helm, Ralph Johnson, John Vlissides");

        library
    }

    fn add_book(&mut self, title: &str, author: &str) {
        let book = Book {
            id: self.next_id,
            title: title.to_string(),
            author: author.to_string(),
            available: true,
        };
        self.books.insert(self.next_id, book);
        self.next_id += 1;
    }

    fn list_books(&self) {
        if self.books.is_empty() {
            println!("No books available in the library.");
        } else {
            println!("\nAvailable Books:");
            for book in self.books.values() {
                if book.available {
                println!("ID: {} \nTitle: {} \nAuthor: {}\n----------------------\n", book.id, book.title, book.author);
                }
            }
        }
    }

    fn list_borrowed_books(&self) {
        if self.borrowed_books.is_empty() {
            println!("No books have been borrowed.");
        } else {
            println!("\nBorrowed Books:");
            for book in self.borrowed_books.values() {
                println!("ID: {}, Title: {}, Author: {}", book.id, book.title, book.author);
            }
        }
    }

    fn borrow_book(&mut self, id: u32) {
        if let Some(book) = self.books.get_mut(&id) {
            if book.available {
                book.available = false;
                self.borrowed_books.insert(id, book.clone()); 
                println!("You have successfully borrowed '{}'.", book.title);
            } else {
                println!("The book '{}' is currently not available.", book.title);
            }
        } else {
            println!("Book with ID {} does not exist.", id);
        }
    }

    fn return_book(&mut self, id: u32) {
        if let Some(book) = self.books.get_mut(&id) {
            if !book.available {
                book.available = true;
                self.borrowed_books.remove(&id); 
                println!("You have successfully returned '{}'.", book.title);
            } else {
                println!("The book '{}' was not borrowed.", book.title);
            }
        } else {
            println!("Book with ID {} does not exist.", id);
        }
    }
}

fn main() {
    let mut library = Library::new();
    loop {
        println!("\nLibrary Management System");
        println!("1. Add Book");
        println!("2. List Available Books");
        println!("3. List Borrowed Books");
        println!("4. Borrow Book");
        println!("5. Return Book");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let mut title = String::new();
                let mut author = String::new();

                print!("Enter book title: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).expect("Failed to read line");

                print!("Enter book author: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut author).expect("Failed to read line");

                library.add_book(&title.trim(), &author.trim());
            }
            2 => library.list_books(),
            3 => library.list_borrowed_books(),
            4 => {
                let mut id = String::new();
                print!("Enter book ID to borrow: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                library.borrow_book(id);
            }
            5 => {
                let mut id = String::new();
                print!("Enter book ID to return: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                library.return_book(id);
            }
            6 => break,
            _ => println!("Invalid choice! Please enter a number between 1 and 6."),
        }
    }
}
