use crate::commands::base::{Command, CommandType};
use crate::commands::{
    apple_tv, chromecast, disco, fridge, off, on, starry_sky, volume, volume_down, volume_mute,
    volume_up,
};

pub fn find_commands<'a>(input: &'a str, commands: &'a Vec<Command>) -> Vec<&'a Command> {
    let mut filtered: Vec<&'a Command> = vec![];

    for detail in commands {
        for alias in &detail.commands {
            if alias.starts_with(input) {
                filtered.push(&detail);
                break;
            }
        }
    }

    return filtered;
}

pub async fn run_command(_type: &CommandType, command: &str) -> Result<(), reqwest::Error> {
    return match _type {
        CommandType::Help => crate::commands::help::command_help().await,
        CommandType::On => on::command_on().await,
        CommandType::Off => off::command_off().await,
        CommandType::AppleTV => apple_tv::command_apple_tv().await,
        CommandType::Chromecast => chromecast::command_chromecast().await,
        CommandType::Fridge => fridge::command_fridge().await,
        CommandType::StarrySky => starry_sky::command_starry_sky().await,
        CommandType::Disco => disco::command_disco().await,
        CommandType::Mute => volume_mute::command_volume_mute().await,
        CommandType::VolumeUp => volume_up::command_volume_up(&command).await,
        CommandType::VolumeDown => volume_down::command_volume_down(&command).await,
        CommandType::Volume => volume::command_volume().await,
    };
}
