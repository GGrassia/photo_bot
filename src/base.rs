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

#[derive(Default, Clone)]
pub enum State {
    #[default]
    Start,
    AwaitingLocationSelection {
        query: String,
    },
}

pub fn to_error<T, E>(res: Result<T, E>) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
where
    E: std::error::Error + Send + Sync + 'static,
{
    res.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
}
