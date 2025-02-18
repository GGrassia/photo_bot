pub mod base;
pub mod weather_commands;

use base::BotCommand;
use inventory;

pub fn get_all_commands() -> Vec<&'static dyn BotCommand> {
    inventory::iter::<dyn BotCommand>.collect()
}