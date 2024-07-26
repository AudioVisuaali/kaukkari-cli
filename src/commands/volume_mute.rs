use crate::api::api_volume_set;
use crate::commands::base::{Command, CommandType};

pub async fn command_volume_mute() -> Result<(), reqwest::Error> {
    let res = api_volume_set(&(10.0)).await;
    println!("Volume muted ✨");
    res
}

pub fn details_volume_mute() -> Command {
    Command {
        command_type: CommandType::Mute,
        commands: vec!["mute".to_string()],
        description: "Mute volume".to_string(),
    }
}
