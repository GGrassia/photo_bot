mod commands;
mod base;

use dotenv::dotenv;
use base::Command;
use commands::{hello, help, weather};
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the photo_bot");

    dotenv().ok();

    let bot = Bot::from_env();

    let command_handler = Update::filter_message()
        .filter_command::<Command>()
        .branch(dptree::case![Command::Hello].endpoint(hello::handle))
        .branch(dptree::case![Command::Help].endpoint(help::handle))
        .branch(dptree::case![Command::Weather(query)].endpoint(weather::handle));

    let callback_handler = Update::filter_callback_query()
        .endpoint(weather::handle_callback);

    let handler = dptree::entry()
        .branch(command_handler)
        .branch(callback_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

}
