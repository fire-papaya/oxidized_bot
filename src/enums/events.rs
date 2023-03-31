use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntryEvent {
    EntryCreated {
        user_id: String,
        contest_id: String,
        code: String,
    },
    PrimeImageUploaded {
        contest_id: String,
        image_id: String,
    },
    FinalImageUploaded {
        contest_id: String,
        image_id: String,
    },
}