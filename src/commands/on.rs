use crate::api::{api_amplifier_state, api_on};
use crate::commands::base::{Command, CommandType};

pub async fn command_on() -> Result<(), reqwest::Error> {
    let state = api_amplifier_state().await?;

    if state.powered {
        println!("Already on ✨");
        return Ok(());
    }

    let res = api_on().await;
    println!("Turning on ✨");

    res
}

pub fn details_on() -> Command {
    Command {
        command_type: CommandType::On,
        commands: vec!["on".to_string()],
        description: "Turns on".to_string(),
    }
}
