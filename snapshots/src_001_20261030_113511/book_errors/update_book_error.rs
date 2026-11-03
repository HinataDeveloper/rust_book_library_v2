use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct UpdateBookError;

impl Display for UpdateBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Update book error ...")
    }
}

impl Debug for UpdateBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateBookError").finish()
    }
}

impl Error for UpdateBookError {}
