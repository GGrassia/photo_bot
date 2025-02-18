mod commands;

use commands::base::BotCommand;
use commands::get_all_commands;
use teloxide::{ dispatching::dialogue::GetChatId, prelude::*, types::Message};


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the photo_bot");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
    .branch(
        dptree::entry()
        .filter_command::<String>()
        .endpoint(|bot: Bot, msg: Message, query: String| async move {
            let command = query.trim_start_matches('/').to_lowercase();
            let commands = get_all_commands();

            if let Some(cmd) = 
            commands.iter().find(|c| c.name() == command)
            {
                cmd.execute(bot, msg).await?;
            } else {
                bot.send_message(msg.chat_id(), "Command not found, sorry!");
            }

            respond(())
        }),
    );
    .branch(Update::filter_callback_query().endpoint(forecast_callback_handler));

}
