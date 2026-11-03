use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct SelectBookError;

impl Display for SelectBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Select book error ...")
    }
}

impl Debug for SelectBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectBookError").finish()
    }
}

impl Error for SelectBookError {}
