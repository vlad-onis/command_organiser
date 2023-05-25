use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use tracing::{info, trace, warn, Level};

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

pub async fn read_commands_from_file(file: String) -> Result<Vec<Command>> {
    let input_file_path = Path::new(&file);
    if !input_file_path.is_file() {
        return Err(anyhow!("path is not a file"));
    }

    trace!("Parsing the file: {file}");

    let toml_string = std::fs::read_to_string(input_file_path)
        .with_context(|| format!("Failed to read the contents from {input_file_path:?}"))?;

    let commands: HashMap<String, Vec<Command>> = toml::from_str(&toml_string)
        .with_context(|| format!("Failed to deserialise the commands from file"))?;

    let commands = commands["commands"].clone();

    Ok(commands)
}

pub async fn populate_db() -> Result<()> {
    let args = Args::parse();

    if let Some(file) = args.file {
        info!("Populating the db from input file: {}", file);
        let commands = read_commands_from_file(file).await?;

        info!("Creating the command service...");
        let command_service = CommandService::new(&args.db_file)
            .await
            .with_context(|| format!("Failed to create the Command Service"))?;

        for command in commands {
            let inserted = command_service
                .insert_command(&command.command, &command.alias, command.description)
                .await;

            if let Err(e) = inserted {
                warn!(
                    "Could not insert command {} because: {:?}",
                    command.alias, e
                );
            }
        }
    }

    Ok(())
}
