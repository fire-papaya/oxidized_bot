use async_trait::async_trait;
use crate::domain::command::CommandHandler;
use crate::domain::handler::Handler;
use crate::domain::command::entry_command::EntryCommand;
use crate::domain::event::ContestEvent;
use crate::domain::error::EntryError;


pub struct UploadPaintedImageCommand {}

impl Handler<EntryCommand> for UploadPaintedImageCommand {
    fn supports(&self, handleable: &EntryCommand) -> bool {
        return *handleable == EntryCommand::UploadPaintedImage
    }
}

#[async_trait]
impl CommandHandler<EntryCommand, ContestEvent, EntryError> for UploadPaintedImageCommand {
    async fn handle(&self, command: &EntryCommand) -> Result<Vec<ContestEvent>, EntryError> {
        println!("Upload PAINTED image with command {:?}", command);
        todo!()
    }
}

