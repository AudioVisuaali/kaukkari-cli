use crate::api::{api_amplifier_state, api_on};
use crate::commands::base::{Command, CommandType};

pub async fn command_off() -> Result<(), reqwest::Error> {
    let amplifier = api_amplifier_state().await?;

    if !amplifier.powered {
        println!("Already off 🌙");
        return Ok(());
    }

    let res = api_on().await;
    println!("Powering off ✨");

    res
}

pub fn details_off() -> Command {
    Command {
        command_type: CommandType::Off,
        commands: vec!["off".to_string()],
        description: "Turns off".to_string(),
    }
}
