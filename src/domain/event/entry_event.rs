use cqrs_es::DomainEvent;
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

impl DomainEvent for EntryEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            EntryEvent::EntryCreated { .. } => "EntryCreated",
            EntryEvent::PrimeImageUploaded { .. } => "PrimeImageUploaded",
            EntryEvent::FinalImageUploaded { .. } => "FinalImageUploaded",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "0.0.1".to_string()
    }
}