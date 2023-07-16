use async_trait::async_trait;
use crate::domain::command::CommandHandler;
use crate::domain::handler::Handler;
use crate::domain::command::entry_command::EntryCommand;
use crate::domain::event::ContestEvent;
use crate::domain::error::EntryError;


pub struct GenerateCodeCommand {}

impl Handler<EntryCommand> for GenerateCodeCommand {
    fn supports(&self, handleable: &EntryCommand) -> bool {
        return *handleable == EntryCommand::GenerateCode
    }
}

#[async_trait]
impl CommandHandler<EntryCommand, ContestEvent, EntryError> for GenerateCodeCommand {
    async fn handle(&self, command: &EntryCommand) -> Result<Vec<ContestEvent>, EntryError> {
        println!("Generate code with command {:?}", command);
        todo!()
    }
}

