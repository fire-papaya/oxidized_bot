use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum EntryCommand {
    GenerateCode { user_id: String },
    UploadPrimeImage,
    UploadPaintedImage,
}