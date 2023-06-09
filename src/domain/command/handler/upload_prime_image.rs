use async_trait::async_trait;
use crate::domain::command::CommandHandler;
use crate::domain::handler::Handler;
use crate::domain::command::entry_command::EntryCommand;
use crate::domain::event::entry_event::EntryEvent;
use crate::domain::error::EntryError;


pub struct UploadPrimeImageCommand {}

impl Handler<EntryCommand> for UploadPrimeImageCommand {
    fn supports(&self, handleable: &EntryCommand) -> bool {
        return *handleable == EntryCommand::UploadPrimeImage
    }
}

#[async_trait]
impl CommandHandler<EntryCommand, EntryEvent, EntryError> for UploadPrimeImageCommand {
    async fn handle(&self, command: &EntryCommand) -> Result<Vec<EntryEvent>, EntryError> {
        println!("Upload prime image with command {:?}", command);
        todo!()
    }
}

