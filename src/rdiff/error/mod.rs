use std::{error::Error, fmt::Display};

pub mod messages;

pub type RdiffError = Box<dyn Error>;

#[derive(Debug, PartialEq)]
pub struct RollingHashError {
    message: String,
}

impl RollingHashError {
    pub fn new(msg: &str) -> RollingHashError {
        let message = String::from(msg);
        RollingHashError { message }
    }

    pub fn rdiff_error(msg: &str) -> RdiffError {
        let error = RollingHashError::new(msg);
        Box::new(error)
    }
}

impl Error for RollingHashError {}

impl Display for RollingHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests;
