use crate::commands::{
    base::{Command, CommandType},
    generate_details,
};
use colored::Colorize;

pub async fn command_help() -> Result<(), reqwest::Error> {
    print_help();

    Ok(())
}

pub fn details_help() -> Command {
    Command {
        command_type: CommandType::Help,
        commands: vec!["help".to_string()],
        description: "Show all the commands".to_string(),
    }
}

pub fn print_help() {
    let title = "Kaukkari CLI tool".green().to_string();
    let section = "Commands:".bright_green().to_string();
    let description = "Supports full and partial command inputs".to_string();
    let mut message: String = format!("{}\n\n{}\n  {}\n", title, section, description);

    let details = generate_details();

    let mut longest_command = 0;

    for command in &details {
        let length = command.commands.join(", ").len();

        if length > longest_command {
            longest_command = length;
        }
    }

    for command in details {
        let commands = command.commands.join(", ");
        let padding = " ".repeat(longest_command - commands.len());

        let commands: Vec<String> = command
            .commands
            .iter()
            .map(|alias| alias.bright_blue().to_string())
            .collect();

        message.push_str(&format!(
            "  {}  {}{}\n",
            commands.join(", "),
            padding,
            command.description
        ));
    }

    println!("{}", message);
}
