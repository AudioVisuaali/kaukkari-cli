use crate::api::{api_amplifier_state, api_volume_set};
use crate::commands::base::{Command, CommandType};

pub async fn command_volume_down(command: &str) -> Result<(), reqwest::Error> {
    let amount = match command {
        "-" => 1.0,
        "--" => 2.0,
        "---" => 3.0,
        "----" => 4.0,
        "-----" => 5.0,
        _ => 1.0,
    };
    let state = api_amplifier_state().await?;

    let multiplier = match state.volume.value {
        x if x >= 0.0 && x < 20.0 => 3.0,
        x if x >= 20.0 && x <= 40.0 => 2.0,
        _ => 1.0,
    };

    let res = api_volume_set(&(state.volume.value - (amount * multiplier))).await;
    println!("Volume decreased ✨");
    res
}

pub fn details_volume_down() -> Command {
    Command {
        command_type: CommandType::VolumeDown,
        commands: vec![
            "-".to_string(),
            "--".to_string(),
            "---".to_string(),
            "----".to_string(),
            "-----".to_string(),
        ],
        description: "Turn volume down".to_string(),
    }
}
