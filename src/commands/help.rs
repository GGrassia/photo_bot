use teloxide::{prelude::*, utils::command::BotCommands};
use crate::{base, HandlerResult};
use crate::base::Command;

pub async fn handle(bot: Bot, msg: Message) -> HandlerResult {
    let descriptions = Command::descriptions();
    let _ = base::to_error(bot.send_message(
        msg.chat.id,
        format!("Available commands: \n{}", descriptions)).await)?;
    Ok(())
}