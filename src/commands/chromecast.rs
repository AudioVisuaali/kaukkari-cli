use crate::api::{api_input, Input};
use crate::commands::base::{Command, CommandType};

pub async fn command_chromecast() -> Result<(), reqwest::Error> {
    let res = api_input(&Input::Chromecast).await;

    println!("Input changed to Chromecast ✨");

    res
}

pub fn details_chromecast() -> Command {
    Command {
        command_type: CommandType::Chromecast,
        commands: vec!["chromecast".to_string()],
        description: "Change input to Chromecast".to_string(),
    }
}
