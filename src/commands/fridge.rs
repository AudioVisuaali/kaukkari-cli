use crate::api::api_fridge;
use crate::commands::base::{Command, CommandType};

pub async fn command_fridge() -> Result<(), reqwest::Error> {
    let res = api_fridge().await;

    println!("Disco toggled ✨");

    res
}

pub fn details_fridge() -> Command {
    Command {
        command_type: CommandType::Fridge,
        commands: vec!["fridge".to_string(), "jaakaappi".to_string()],
        description: "Toggle power to fridge".to_string(),
    }
}
