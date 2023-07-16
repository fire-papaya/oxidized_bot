mod enums;
mod domain;
mod service;
mod presentation;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use std::env;
use std::error::Error;
use presentation::paint_bot::PaintContestBot;

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

    // Command::repl(bot, answer).await;
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