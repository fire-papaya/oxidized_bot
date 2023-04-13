use serde::Deserialize;
use crate::domain::command::GenericCommand;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum EntryCommand {
    GenerateCode,
    UploadPrimeImage,
    UploadPaintedImage,
}

impl GenericCommand for EntryCommand {}