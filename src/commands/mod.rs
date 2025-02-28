pub mod hello;
pub mod help;
pub mod weather;
use crate::base;

use teloxide::{prelude::*, error_handlers::LoggingErrorHandler};
use crate::HandlerResult;

pub async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    let _ = base::to_error(bot.send_message(
        msg.chat.id,
        "I'm not sure what to do with that message. Try using /help to see available commands."
    )
        .await)?;

    Ok(())
}