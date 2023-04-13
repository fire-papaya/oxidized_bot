use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use crate::domain::command::entry_command::EntryCommand;
use crate::domain::event::entry_event::EntryEvent;
use crate::domain::error::EntryError;
use crate::service::entry_service::EntryService;


#[derive(Serialize, Default, Deserialize)]
pub struct Entry {
    entry_id: i32,
    code: String,
}

#[async_trait]
impl Aggregate for Entry {
    type Command = EntryCommand;
    type Event = EntryEvent;
    type Error = EntryError;
    type Services = EntryService;

    // This identifier should be unique to the system.
    fn aggregate_type() -> String {
        "Entry".to_string()
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    // computations, checks, 3rd-party calls here
    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    // update here
    fn apply(&mut self, event: Self::Event) {
        match event {
            EntryEvent::EntryCreated { .. } => {
                self.entry_id = 0;
                self.code = String::from("111111");
            }

            EntryEvent::PrimeImageUploaded { .. } => {
                // add image into db and save entry relation
                todo!()
            }

            EntryEvent::FinalImageUploaded { .. } => {
                // add image into db and save entry relation
                todo!()
            }
        }

    }
}
