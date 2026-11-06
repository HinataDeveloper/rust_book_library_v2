use std::collections::{HashMap, hash_map::Values};

use crate::{
    book_errors::{
        DeleteBookError, InsertBookError, ListBookError, SelectBookError, UpdateBookError,
    },
    entity::BookEntity,
};

#[derive(Debug, Default)]
pub struct BookRepository {
    book_list: HashMap<u64, BookEntity>,
}

impl BookRepository {
    #[must_use]
    pub fn new() -> Self {
        Self {
            book_list: HashMap::new(),
        }
    }

    /// # Errors
    ///
    /// returns a ``Result<(), InsertBookError>``.
    /// if any error occurs `Result` has ``InsertBookError`` error object
    pub fn insert(&mut self, book: BookEntity) -> Result<(), InsertBookError> {
        let result = self.book_list.insert(book.id(), book);

        result.map_or(Err(InsertBookError), |_book| Ok(()))
    }

    /// # Errors
    ///
    /// returns a ``Result<(), UpdateBookError>``.
    /// if any error occurs `Result` has ``UpdateBookError`` error object
    pub fn update(&mut self, book: BookEntity) -> Result<(), UpdateBookError> {
        if self.book_list.contains_key(&book.id()) {
            self.book_list.entry(book.id()).or_insert(book);
            Ok(())
        } else {
            Err(UpdateBookError)
        }
    }

    /// # Errors
    ///
    /// returns a ``Result<(), DeleteBookError>``.
    /// if any error occurs `Result` has ``DeleteBookError`` error object
    pub fn delete(&mut self, id: u64) -> Result<(), DeleteBookError> {
        self.book_list
            .remove(&id)
            .map_or(Err(DeleteBookError), |_book| Ok(()))
    }

    /// # Errors
    ///
    /// returns a ``Result<&BookEntity, SelectBookError>``.
    /// if any error occurs `Result` has ``SelectBookError`` error object
    pub fn select(&self, id: u64) -> Result<&BookEntity, SelectBookError> {
        self.book_list.get(&id).ok_or(SelectBookError)
    }

    /// # Errors
    ///
    /// returns a ``Result<(), ListBookError>``.
    /// if any error occurs `Result` has ``ListBookError`` error object
    pub fn select_all(&self) -> Result<Values<'_, u64, BookEntity>, ListBookError> {
        if self.book_list.is_empty() {
            Err(ListBookError)
        } else {
            Ok(self.book_list.values())
        }
    }
}
