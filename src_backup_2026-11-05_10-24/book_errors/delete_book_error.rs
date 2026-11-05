use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct DeleteBookError;

impl Display for DeleteBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Delete book error ...")
    }
}

impl Debug for DeleteBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteBookError").finish()
    }
}

impl Error for DeleteBookError {}
