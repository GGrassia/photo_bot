mod commands;
mod base;
mod weather_data_model;

use dotenv::dotenv;
use base::{Command, State};
use commands::{hello, help, weather};
use teloxide::{
    prelude::*,
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    println!("Starting the photo_bot");
    log::info!("Starting the photo_bot");


    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]

            .branch(dptree::case![Command::Hello].endpoint(hello::handle))
            .branch(dptree::case![Command::Help].endpoint(help::handle))
            .branch(dptree::case![Command::Weather(query)].endpoint(weather::handle)),
        );

    let message_handler = Update::filter_message()
    .branch(command_handler)
    .branch(dptree::endpoint(commands::invalid_state));

    let callback_handler = Update::filter_callback_query()
        .branch(
            case![State::AwaitingLocationSelection { query }].endpoint(commands::weather::handle_callback),
        );

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_handler)

}
