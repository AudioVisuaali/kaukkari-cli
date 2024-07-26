use crate::api::api_amplifier_state;
use crate::commands::base::{Command, CommandType};

pub async fn command_volume() -> Result<(), reqwest::Error> {
    let state = api_amplifier_state().await?;

    let percentage = { (state.volume.value / state.volume.max) * 100.0 }.ceil();
    println!("{}% ({}dB)", percentage, state.volume.value);
    Ok(())
}

pub fn details_volume() -> Command {
    Command {
        command_type: CommandType::Volume,
        commands: vec!["volume".to_string()],
        description: "Get volume".to_string(),
    }
}
