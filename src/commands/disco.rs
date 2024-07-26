use crate::api::api_disco;
use crate::commands::base::{Command, CommandType};

pub async fn command_disco() -> Result<(), reqwest::Error> {
    api_disco().await
}

pub fn details_disco() -> Command {
    Command {
        command_type: CommandType::Disco,
        commands: vec!["disco".to_string()],
        description: "Toggle power to disco".to_string(),
    }
}
