use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BotCommand {
    GenerateCode { user_id: String },
    UploadPrimeImage,
    UploadPaintedImage,
}