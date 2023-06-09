pub mod entry_command;
mod handler;

use async_trait::async_trait;
use cqrs_es::DomainEvent;
use std::error::Error;
use crate::domain::handler::Handler;

trait GenericCommand {}

#[async_trait]
pub trait CommandHandler<C: GenericCommand, V: DomainEvent, E: Error>: Handler<C> {
    async fn handle(
        &self,
        command: &C,
    ) -> Result<Vec<V>, E>;
}
