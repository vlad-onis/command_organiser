use sqlx::FromRow;

// todo: introduce alias for commands

#[derive(Clone, FromRow, Debug)]
pub struct Command {
    pub alias: String,
    pub executable: String,
    pub command: String,
    pub description: Option<String>,
}

impl Command {
    pub fn new(
        executable: String,
        command: String,
        alias: String,
        description: Option<String>,
    ) -> Command {
        // todo: validate that the executable is installed
        // todo: Perform a dryrun to ensure command is correct.
        Command {
            alias,
            executable,
            command,
            description,
        }
    }
}
