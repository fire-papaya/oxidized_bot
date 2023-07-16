mod enums;
mod domain;
mod service;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use std::env;
use std::error::Error;
use cqrs_es::DomainEvent;
use domain::command::entry_command::EntryCommandHandler;
use domain::command::CommandHandler;
use domain::command::GenericCommand;

pub struct PaintContestBot {
    bot: Bot,
    command_handler: dyn CommandHandler<dyn GenericCommand, dyn DomainEvent, dyn Error>,
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
    command_handler: Option<dyn CommandHandler<dyn GenericCommand, dyn DomainEvent, dyn Error>>,
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
            command_handler: self.command_handler,
            bot_name: self.bot_name.unwrap(),
            creator: self.creator.unwrap(),
            token: self.token.unwrap(),
        }
    }
}




#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();

    let bot_name = &args[1];
    let creator = &args[2];
    let token = &args[3];

    let paint_bot = PaintContestBot::builder()
        .name(bot_name)
        .token(token)
        .creator(creator)
        .build();

    log::info!("Hello, {}", bot_name);
    log::info!("Made by {}", creator);
    log::info!("With token {}", token);

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These domain are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
    };

    Ok(())
}