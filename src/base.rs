use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Say hello to the bot")]
    Hello,

    #[command(description = "Show the available commands")]
    Help,

    #[command(description = "Meteosource API for forecast")]
    Weather(String),
}