use std::collections::hash_map::Values;

use crate::book_da::BookRepository;
use crate::book_errors::{
    DeleteBookError, InsertBookError, ListBookError, SelectBookError, UpdateBookError,
};
use crate::entity::BookEntity;

#[derive(Debug, Default)]
pub struct BookBL {
    book_repo: BookRepository,
}

impl BookBL {
    #[must_use]
    pub const fn new(book_repo: BookRepository) -> Self {
        Self { book_repo }
    }

    /// # Errors
    ///
    /// returns a ``Result<(), InsertBookError>``.
    /// if any error occurs `Result` has ``InsertBookError`` error object
    pub fn book_register(&mut self, book: BookEntity) -> Result<(), InsertBookError> {
        self.book_repo.insert(book)
    }

    /// # Errors
    ///
    /// returns a ``Result<(), UpdateBookError>``.
    /// if any error occurs `Result` has ``UpdateBookError`` error object
    pub fn book_edit(&mut self, book: BookEntity) -> Result<(), UpdateBookError> {
        self.book_repo.update(book)
    }

    /// # Errors
    ///
    /// returns a ``Result<(), DeleteBookError>``.
    /// if any error occurs `Result` has ``DeleteBookError`` error object
    pub fn book_remove(&mut self, id: u64) -> Result<(), DeleteBookError> {
        self.book_repo.delete(id)
    }

    /// # Errors
    ///
    /// returns a ``Result<(), SelectBookError>``.
    /// if any error occurs `Result` has ``SeleteBookError`` error object
    pub fn book_choose(&self, id: u64) -> Result<&BookEntity, SelectBookError> {
        self.book_repo.select(id)
    }

    /// # Errors
    ///
    /// returns a ``Result<(), ListBookError>``.
    /// if any error occurs `Result` has ``ListBookError`` error object
    pub fn book_list(&self) -> Result<Values<'_, u64, BookEntity>, ListBookError> {
        self.book_repo.select_all()
    }
}
