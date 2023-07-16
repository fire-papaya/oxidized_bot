use std::error::Error;

use cqrs_es::DomainEvent;
use teloxide::Bot;

use crate::domain::command::CommandHandler;
use crate::domain::command::GenericCommand;
use crate::domain::event::ContestEvent;


pub struct PaintContestBot {
    bot: Bot,
    command_handler: Box<dyn CommandHandler<dyn GenericCommand, ContestEvent, dyn Error>>,
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
pub struct BotBuilder {
    command_handler: Option<Box<dyn CommandHandler<dyn GenericCommand, ContestEvent, dyn Error>>>,
    bot_name: Option<String>,
    creator: Option<String>,
    token: Option<String>
}

impl BotBuilder {
    fn new() -> BotBuilder {
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

        match &self.token {
            Some(t) => log::info!("Token is set to {}", t[..6].to_string() + &String::from("...")),
            None => panic!("ERROR: Telegram bot token is not specified")
        }

        match self.creator {
            Some(ref c) => log::info!("Creator is set to {}", &c),
            None => panic!("ERROR: Telegram bot creator is not specified")
        }

        match self.bot_name {
            Some(ref bn) => log::info!("BotName is set to {}", &bn),
            None => panic!("ERROR: Telegram BotName is not specified")
        }

        let bot = Bot::new(self.token.as_ref().unwrap());

        PaintContestBot {
            bot: bot,
            command_handler: self.command_handler.unwrap(),
            bot_name: self.bot_name.unwrap(),
            creator: self.creator.unwrap(),
            token: self.token.unwrap(),
        }
    }
}