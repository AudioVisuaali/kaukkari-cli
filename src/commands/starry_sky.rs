use crate::api::api_starry_sky;
use crate::commands::base::{Command, CommandType};

pub async fn command_starry_sky() -> Result<(), reqwest::Error> {
    let res = api_starry_sky().await;

    println!("Toggled lights ✨");

    res
}

pub fn details_starry_sky() -> Command {
    Command {
        command_type: CommandType::StarrySky,
        commands: vec!["starrysky".to_string(), "valot".to_string()],
        description: "Toggle power to fridge".to_string(),
    }
}
