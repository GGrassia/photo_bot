mod weather;

use teloxide::{
    prelude::*,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use weather::{forecast_callback_handler, search_handler};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the photo_bot");

    let bot = Bot::from_env();

    let handler = Update::filter_message().branch(
        dptree::entry()
        .filter_command::<String>()
        .endpoint(|bot: Bot, msg: Message, query: String| async move {
            search_handler(bot, msg, &query).await
        }),
    )
    .branch(Update::filter_callback_queSry().endpoint(forecast_callback_handler));

    //Command::repl(bot, answer).await;
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

