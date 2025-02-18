use teloxide::prelude::*;

#[async_trait::async_trait]
pub trait BotCommand: Sync + Send {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn execute(&self, bot: Bot, msg: Message) -> ResponseResult<()>;
}

inventory::collect!(dyn BotCommand);