use std::collections::{HashMap, hash_map::Values};

use crate::{
    book_errors::{
        DeleteBookError, InsertBookError, ListBookError, SelectBookError, UpdateBookError,
    },
    entity::BookEntity,
};

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

    pub fn insert(&mut self, book: BookEntity) -> Result<(), InsertBookError> {
        self.book_list
            .insert(book.id(), book)
            .map_or(Err(InsertBookError), |_book| Ok(()))
    }

    pub fn update(&mut self, book: BookEntity) -> Result<(), UpdateBookError> {
        if self.book_list.contains_key(&book.id()) {
            self.book_list.entry(book.id()).or_insert(book);
            Ok(())
        } else {
            Err(UpdateBookError)
        }
    }

    pub fn delete(&mut self, book: BookEntity) -> Result<(), DeleteBookError> {
        self.book_list
            .remove(&book.id())
            .map_or(Err(DeleteBookError), |_book| Ok(()))
    }

    pub fn select(&self, book: BookEntity) -> Result<&BookEntity, SelectBookError> {
        self.book_list.get(&book.id()).ok_or(SelectBookError)
    }

    pub fn list(&self) -> Result<Values<'_, u64, BookEntity>, ListBookError> {
        if self.book_list.is_empty() {
            Err(ListBookError)
        } else {
            Ok(self.book_list.values())
        }
    }
}
