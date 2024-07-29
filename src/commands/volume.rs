use crate::api::{api_amplifier_state, AmplifierVolume};
use crate::commands::base::{Command, CommandType};

pub async fn command_volume() -> Result<(), reqwest::Error> {
    let state = api_amplifier_state().await?;

    let stat = format_volume(state.volume);
    println!("Volume: {}", stat);
    Ok(())
}

pub fn details_volume() -> Command {
    Command {
        command_type: CommandType::Volume,
        commands: vec!["volume".to_string()],
        description: "Get volume".to_string(),
    }
}

pub fn format_volume(volume: AmplifierVolume) -> String {
    let percentage = { (volume.value / volume.max) * 100.0 }.ceil();
    format!("{}% ({}dB)", percentage, volume.value)
}
