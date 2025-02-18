use reqwest;
use serde::Deserialize;
use std::env;
use teloxide::{
    prelude::*,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv::dotenv().ok();
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(Deserialize)]
struct Location {
    id: &Str,
    name: &Str,
}

#[derive(Deserialize)]
struct Forecast {
    forecast: &str,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available Commands:")]
enum Command {
    #[command(description = "Display help message.")]
    Help,
    #[command(description = "Say hello!")]
    Start,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Hello! I'm Giulio's PhotoBot!")
                .await?
        }
    };
    Ok(())
}
