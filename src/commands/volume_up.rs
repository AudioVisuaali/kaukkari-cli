use crate::api::{api_amplifier_state, api_volume_set, AmplifierVolume};
use crate::commands::base::{Command, CommandType};
use crate::commands::volume::format_volume;

pub async fn command_volume_up(command: &str) -> Result<(), reqwest::Error> {
    let amount = match command {
        "+" => 1.0,
        "++" => 2.0,
        "+++" => 3.0,
        "++++" => 4.0,
        "+++++" => 5.0,
        _ => 1.0,
    };

    let state = api_amplifier_state().await?;

    let multiplier = match state.volume.value {
        x if x >= 0.0 && x < 20.0 => 3.0,
        x if x >= 20.0 && x <= 40.0 => 2.0,
        _ => 1.0,
    };

    let new_volume = state.volume.value + (amount * multiplier);
    let _ = api_volume_set(&new_volume).await;

    let updated_state = api_amplifier_state().await?;
    let stat = format_volume(AmplifierVolume {
        value: updated_state.volume.value,
        max: updated_state.volume.max,
        min: updated_state.volume.min,
    });
    println!("Volume increased ✨ {}", stat);
    Ok(())
}

pub fn details_volume_up() -> Command {
    Command {
        command_type: CommandType::VolumeUp,
        commands: vec![
            "+".to_string(),
            "++".to_string(),
            "+++".to_string(),
            "++++".to_string(),
            "+++++".to_string(),
        ],
        description: "Turn volume up".to_string(),
    }
}
