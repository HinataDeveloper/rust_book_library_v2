// use crate::book_errors::{
//    DeleteBookError, InsertBookError, ListBookError, SelectBookError, UpdateBookError,
// };
use crate::core::{get_string, get_u64_number};
use crate::entity::BookEntity;
use crate::{BookCtrl, BookRepository, display_message};
// use std::collections::hash_map::Values;

#[derive(Debug, Default)]
pub struct BookView {
    book_srv: BookCtrl,
}

impl BookView {
    #[must_use]
    pub const fn new(book_repo: BookRepository) -> Self {
        Self {
            book_srv: BookCtrl::new(book_repo),
        }
    }

    pub fn new_existence(&mut self) {
        let mut book = BookEntity::default();
        book.set_id(get_u64_number("Enter book id"));
        book.set_name(get_string("Enter book name"));
        book.set_author(get_string("Enter book author name"));
        book.set_translator(get_string("Enter book translator name"));
        book.set_publisher(get_string("Enter book publisher name"));
        book.set_context(get_string("Enter book context"));
        match self.book_srv.book_save(book) {
            Ok(()) => display_message("Book was saved the the database successfully ..."),
            Err(err) => println!("  ***--->>> {err}"),
        }
    }

    pub fn new_survive(&mut self) {
        let mut book = BookEntity::default();
        book.set_id(get_u64_number("Enter book id"));
        book.set_name(get_string("Enter book name"));
        book.set_author(get_string("Enter book author name"));
        book.set_translator(get_string("Enter book translator name"));
        book.set_publisher(get_string("Enter book publisher name"));
        book.set_context(get_string("Enter book context"));
        match self.book_srv.book_change(book) {
            Ok(()) => display_message("Book edited successfully ..."),
            Err(err) => println!("  --->>> {err}"),
        }
    }

    pub fn inexistence(&mut self) {
        let id = get_u64_number("Enter book id");
        match self.book_srv.book_erase(id) {
            Ok(()) => display_message("Book was Removed successfully ..."),
            Err(err) => println!("  --->>> {err}"),
        }
    }

    pub fn demand(&mut self) {
        let id = get_u64_number("Enter book id");
        match self.book_srv.book_pick(id) {
            Ok(book) => println!("  {book}"),
            Err(err) => println!("  --->>> {err}"),
        }
    }

    pub fn mass_expose(&self) {
        match self.book_srv.book_cataloging() {
            Ok(value) => {
                for item in value {
                    println!("{item}");
                }
            }
            Err(err) => println!("  --->>> {err}"),
        }
    }
}
