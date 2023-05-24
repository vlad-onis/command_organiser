use std::collections::HashMap;
use std::path::Path;

use clap::Parser;
use serde::Deserialize;

use crate::model::command::Command;
use crate::service::command_service::CommandService;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: Option<String>,

    /// Specify the db file name
    #[arg(short, long, default_value = "commands.db")]
    db_file: String,

    /// Populate the db in termianl, interactive mode
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub async fn read_commands_from_file(
    file: String,
) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
    let input_file_path = Path::new(&file);
    if !input_file_path.is_file() {
        return Err("path is not a file".to_string().into());
    }

    let toml_string = std::fs::read_to_string("commands.toml").expect("Failed to read file");

    let commands: HashMap<String, Vec<Command>> =
        toml::from_str(&toml_string).expect("Failed to deserialize TOML");

    let commands = commands["commands"].clone();

    Ok(commands)
}

pub async fn populate_db() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{args:?}");

    if let Some(file) = args.file {
        let commands = read_commands_from_file(file).await?;
        let command_service = CommandService::new(&args.db_file).await?;

        for command in commands {
            let inserted = command_service
                .insert_command(&command.command, &command.alias, command.description)
                .await;

            if inserted.is_err() {
                println!(
                    "Could not insert command {} because: {:?}",
                    command.alias, inserted
                );
            }
        }
    }

    Ok(())
}
