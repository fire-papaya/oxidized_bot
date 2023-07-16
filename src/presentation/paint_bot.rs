use std::error::Error;

use cqrs_es::DomainEvent;
use teloxide::Bot;

use crate::domain::command::CommandHandler;
use crate::domain::command::GenericCommand;


pub struct PaintContestBot {
    bot: Bot,
    command_handler: Box<dyn CommandHandler<dyn GenericCommand, dyn DomainEvent, dyn Error>>,
    bot_name: String,
    creator: String,
    token: String
}

impl PaintContestBot {
    pub fn builder () -> BotBuilder {
        return BotBuilder::new()
    }
}

#[derive(Default)]
struct BotBuilder {
    command_handler: Option<Box<dyn CommandHandler<dyn GenericCommand, dyn DomainEvent, dyn Error>>>,
    bot_name: Option<String>,
    creator: Option<String>,
    token: Option<String>
}

impl BotBuilder {
    pub fn new() -> BotBuilder {
        BotBuilder {
            command_handler: None,
            bot_name: None,
            creator: None,
            token: None,
        }
    }

    pub fn name(mut self, bot_name: &String) -> BotBuilder {
        self.bot_name = Some(bot_name.to_string());
        self
    }

    pub fn creator(mut self, creator: &String) -> BotBuilder {
        self.creator = Some(creator.to_string());
        self
    }

    pub fn token(mut self, token: &String) -> BotBuilder {
        self.token = Some(token.to_string());
        self
    }

    pub fn build(self) -> PaintContestBot {

        if self.command_handler.is_none() {
            panic!("ERROR: CommandHandler for bot is not specified")
        }

        if self.token.is_none() {
            panic!("ERROR: TelegramApi token is not specified")
        }

        if self.creator.is_none() {
            panic!("ERROR: Telegram bot creator is not specified")
        }

        if self.bot_name.is_none() {
            panic!("ERROR: Telegram BotName is not specified")
        }


        log::info!("Hello, it's {}", self.bot_name.unwrap());
        log::info!("Made by {}", self.creator.unwrap());
        log::info!("With token {}", self.token.unwrap()[..6] + "...");

        let bot = Bot::new(self.token);

        PaintContestBot {
            bot: bot,
            command_handler: self.command_handler.unwrap(),
            bot_name: self.bot_name.unwrap(),
            creator: self.creator.unwrap(),
            token: self.token.unwrap(),
        }
    }
}