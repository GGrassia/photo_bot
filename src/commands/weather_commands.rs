use super::base::BotCommand;
use inventory;
use teloxide::prelude::*;
use crate::weather::search_handler;

pub struct MeteoCommand;
#[async_trait::async_trait]
impl BotCommand for MeteoCommand {
    fn name(&self) -> &'static str {
        "meteo"
    }
    fn description(&self) -> &'static str {
        "Meteosource weather forecast, searches the place after /meteo"
    }
    async fn execute(&self, bot: Bot, msg: Message) -> ResponseResult<()> {
        let text = msg.text().unwrap_or("").trim().to_string();
        if text.is_empty() {
            bot.send_message(msg.chat.id, "Usage: /meteo <location>").await?;
            return Ok(());
        }
        
        search_handler(bot, msg, &self).await?;
        Ok(())
    }
}
inventory::submit!(MeteoCommand);