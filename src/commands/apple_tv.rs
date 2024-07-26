use crate::api::{api_input, Input};
use crate::commands::base::{Command, CommandType};

pub async fn command_apple_tv() -> Result<(), reqwest::Error> {
    let res = api_input(&Input::AppleTV).await;

    println!("Input changed to AppleTV ✨");

    res
}

pub fn details_apple_tv() -> Command {
    Command {
        command_type: CommandType::AppleTV,
        commands: vec!["appletv".to_string()],
        description: "Change input to Apple TV".to_string(),
    }
}
