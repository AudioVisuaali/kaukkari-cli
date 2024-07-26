pub mod api;
pub mod commands;
pub mod matcher;

use commands::help::print_help;
use matcher::{find_commands, run_command};
use std::process;

use crate::commands::base::Command;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        invalid_no_arguments();
        process::exit(1);
    }

    let input_command = &args[1];

    if is_help(&args) {
        print_help();
        return;
    }

    let details: Vec<Command> = commands::generate_details();
    let commands = find_commands(input_command, &details);

    if commands.len() == 0 {
        invalid_command_not_found();
        process::exit(1);
    }

    if commands.len() > 1 {
        invalid_command_too_many_options();
        process::exit(1);
    }

    let res = run_command(&commands[0].command_type, input_command).await;

    match res {
        Ok(_) => (),
        Err(e) => {
            eprintln!("\nAre you connected to the right WIFI?\n");
            if e.is_timeout() {
                eprintln!("Timeout error, check your connection and try again");
                process::exit(1);
            } else {
                eprintln!("Error: {:?}", e);
            }
            process::exit(1);
        }
    }
}

fn invalid_no_arguments() {
    eprintln!("No arguments passed, use --help for all the commands");
}

fn invalid_command_not_found() {
    println!("Command not found, use --help for all the commands");
}

fn invalid_command_too_many_options() {
    println!("Multiple commands found, use more specific command or --help for all the commands");
}

pub fn is_help(args: &Vec<String>) -> bool {
    if args.len() != 2 {
        return false;
    }

    return args[1] == "-h" || args[1] == "--help";
}
