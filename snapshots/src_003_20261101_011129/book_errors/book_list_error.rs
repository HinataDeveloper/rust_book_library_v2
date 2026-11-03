use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct ListBookError;

impl Display for ListBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "List book error ...")
    }
}

impl Debug for ListBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListBookError").finish()
    }
}

impl Error for ListBookError {}
