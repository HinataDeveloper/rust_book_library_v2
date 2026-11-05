use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct InsertBookError;

impl Display for InsertBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Insert book error ...")
    }
}

impl Debug for InsertBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InsertBookError").finish()
    }
}

impl Error for InsertBookError {}
