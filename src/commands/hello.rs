use teloxide::prelude::*;
use crate::{base, HandlerResult};

pub async fn handle(bot: Bot, msg: Message) -> HandlerResult {
    let _ = base::to_error(bot.send_message(
        msg.chat.id, "Hi! I'm Giulio's photo helper bot! Use /help to see all commands."
    )
        .await)?;

    Ok(())
}