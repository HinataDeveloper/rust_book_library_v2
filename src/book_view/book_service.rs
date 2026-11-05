use crate::core::get_string;
use crate::{BookCtrl, BookRepository};

pub struct BookView {
    book_service: BookCtrl,
}

impl BookView {
    #[must_use]
    pub const fn new(book_repo: BookRepository) -> Self {
        Self {
            book_service: BookCtrl::new(book_repo),
        }
    }

    //
    pub fn book_store(&mut self) {}
}
