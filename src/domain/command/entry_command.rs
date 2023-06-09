use std::boxed::Box;
use serde::Deserialize;
use crate::domain::command::{CommandHandler, GenericCommand};
use crate::domain::event::entry_event::EntryEvent;
use crate::domain::error::EntryError;
use crate::domain::command::handler::*;
use crate::domain::command::handler::generate_code::GenerateCodeCommand;
use crate::domain::command::handler::upload_painted_image::UploadPaintedImageCommand;
use crate::domain::command::handler::upload_prime_image::UploadPrimeImageCommand;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum EntryCommand {
    GenerateCode,
    UploadPrimeImage,
    UploadPaintedImage,
}

impl GenericCommand for EntryCommand {}

pub struct EntryCommandHandler {
    handlers: Vec<Box<dyn CommandHandler<EntryCommand, EntryEvent, EntryError>>>
}

impl EntryCommandHandler {
    pub fn new() -> Self {
        let handlers: Vec<Box<dyn CommandHandler<EntryCommand, EntryEvent, EntryError>>> = vec![
            Box::new(GenerateCodeCommand{}),
            Box::new(UploadPrimeImageCommand{}),
            Box::new(UploadPaintedImageCommand{}),
        ];

        Self { handlers }
    }

    async fn handle(&self, command: &EntryCommand) {
        let found_handler = self.handlers
            .iter()
            .find(|&bx| (*bx).as_ref().supports(command));

        match found_handler {
            Some(b_handler) => (*b_handler).as_ref().handle(command),
            None => panic!("NO_HANDLER"),
        };

    }
}