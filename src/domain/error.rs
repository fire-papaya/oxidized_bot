use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct EntryError(String);

impl Display for EntryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl std::error::Error for EntryError {}

impl From<&str> for EntryError {
    fn from(message: &str) -> Self {
        EntryError(message.to_string())
    }
}