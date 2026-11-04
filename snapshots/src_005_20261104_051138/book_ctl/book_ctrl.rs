use std::collections::hash_map::Values;

use crate::{
    book_bl::BookBL,
    book_errors::{
        DeleteBookError, InsertBookError, ListBookError, SelectBookError, UpdateBookError,
    },
    database::BookRepository,
    entity::BookEntity,
};

pub struct BookCtrl {
    book_bl: BookBL,
}

impl BookCtrl {
    #[must_use]
    pub const fn new(book_repo: BookRepository) -> Self {
        Self {
            book_bl: BookBL::new(book_repo),
        }
    }

    pub fn book_save(&mut self, book: BookEntity) -> Result<(), InsertBookError> {
        self.book_bl.book_register(book)
    }

    pub fn book_erase(&mut self, book: BookEntity) -> Result<(), DeleteBookError> {
        self.book_bl.book_remove(book)
    }

    pub fn book_change(&mut self, book: BookEntity) -> Result<(), UpdateBookError> {
        self.book_bl.book_edit(book)
    }

    pub fn book_pickup(&self, book: BookEntity) -> Result<(), SelectBookError> {
        self.book_bl.book_choose(book)
    }

    pub fn book_dump(&self) -> Result<Values<'_, u64, BookEntity>, ListBookError> {
        self.book_bl.book_list()
    }
}
