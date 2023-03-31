use cqrs_es::DomainEvent;
use crate::enums::events::*;

impl DomainEvent for ContestEntryEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            ContestEntryEvent::CodeGenerated { .. } => "Create entry",
            ContestEntryEvent::PrimeImageUploaded { .. } => "Upload prime image",
            ContestEntryEvent::FinalImageUploaded { .. } => "Upload final image",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "0.0.1".to_string()
    }
}