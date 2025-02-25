use teloxide::prelude::*;

pub async fn handle(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        format!(
            "Hi! I'm Giulio's photo helper bot! Use /help to see all commands."
        ),
    )
        .await?;

    Ok(())
}