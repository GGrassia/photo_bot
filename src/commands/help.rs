use teloxide::{prelude::*, utils::command::BotCommands};
use crate::base::Command;

pub async fn handle(bot: Bot, msg: Message) -> ResponseResult<()> {
    let descriptions = Command::descriptions();
    bot.send_message(
        msg.chat.id,
        format!("Available commands: \n{}", descriptions)).await?;
    Ok(())
}