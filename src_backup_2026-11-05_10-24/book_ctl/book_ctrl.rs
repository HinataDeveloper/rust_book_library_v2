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

    /// # Errors
    ///
    /// returns a ``Result<(), InsertBookError>``.
    /// if any error occurs `Result` has ``InsertBookError`` error object
    pub fn book_save(&mut self, book: BookEntity) -> Result<(), InsertBookError> {
        self.book_bl.book_register(book)
    }

    /// # Errors
    ///
    /// returns a ``Result<(), UpdateBookError>``.
    /// if any error occurs `Result` has ``UpdateBookError`` error object
    pub fn book_change(&mut self, book: BookEntity) -> Result<(), UpdateBookError> {
        self.book_bl.book_edit(book)
    }

    /// # Errors
    ///
    /// returns a ``Result<(), DeleteBookError>``.
    /// if any error occurs `Result` has ``DeleteBookError`` error object
    pub fn book_erase(&mut self, id: u64) -> Result<(), DeleteBookError> {
        self.book_bl.book_remove(id)
    }

    /// # Errors
    ///
    /// returns a ``Result<&BookEntity, SelectBookError>``.
    /// if any error occurs `Result` has ``SelectBookError`` error object
    pub fn book_pick(&self, id: u64) -> Result<&BookEntity, SelectBookError> {
        self.book_bl.book_choose(id)
    }

    /// # Errors
    ///
    /// returns a ``Result<(), ListBookError>``.
    /// if any error occurs `Result` has ``ListBookError`` error object
    pub fn book_cataloging(&self) -> Result<Values<'_, u64, BookEntity>, ListBookError> {
        self.book_bl.book_list()
    }
}
