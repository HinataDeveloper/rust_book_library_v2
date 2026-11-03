use std::collections::hash_map::Values;

use crate::book_errors::{
    DeleteBookError, InsertBookError, ListBookError, SelectBookError, UpdateBookError,
};
use crate::database::BookRepository;
use crate::entity::BookEntity;
// ramin
struct BookBL {
    book_repo: BookRepository,
}
// ramin hoobakht 
impl BookBL {
    #[must_use]
    pub const fn new(book_repo: BookRepository) -> Self {
        Self { book_repo }
    }

    pub fn book_register(&mut self, book: BookEntity) -> Result<(), InsertBookError> {
        self.book_repo
            .insert(book)
            .map_or(Err(InsertBookError), |_book| Ok(()))
    }

    pub fn book_remove(&mut self, book: BookEntity) -> Result<(), DeleteBookError> {
        self.book_repo
            .delete(book)
            .map_or(Err(DeleteBookError), |_book| Ok(()))
    }

    pub fn book_edit(&mut self, book: BookEntity) -> Result<(), UpdateBookError> {
        self.book_repo
            .update(book)
            .map_or(Err(UpdateBookError), |_book| Ok(()))
    }

    pub fn book_choose(&self, book: BookEntity) -> Result<(), SelectBookError> {
        self.book_repo
            .select(book)
            .map_or(Err(SelectBookError), |_book| Ok(()))
    }

    pub fn book_list(&self) -> Result<Values<'_, u64, BookEntity>, ListBookError> {
        self.book_repo.list()
    }
}
