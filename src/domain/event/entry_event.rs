use cqrs_es::DomainEvent;
use crate::enums::events::EntryEvent;

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