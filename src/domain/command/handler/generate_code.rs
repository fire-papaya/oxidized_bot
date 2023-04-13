use async_trait::async_trait;
use crate::domain::command::CommandHandler;
use crate::domain::handler::Handler;
use crate::domain::command::entry_command::EntryCommand;
use crate::domain::event::entry_event::EntryEvent;
use crate::domain::error::EntryError;


pub struct GenerateCodeCommand {}

impl Handler<EntryCommand> for GenerateCodeCommand {
    fn supports(handleable: EntryCommand) -> bool {
        return handleable == EntryCommand::GenerateCode
    }
}

#[async_trait]
impl CommandHandler<EntryCommand, EntryEvent, EntryError> for GenerateCodeCommand {
    async fn handle(command: EntryCommand) -> Result<Vec<EntryEvent>, EntryError> {
        todo!()
    }
}

