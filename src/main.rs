mod cli;
mod model;
mod service;
mod storage;
mod ui;

use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

fn set_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    set_tracing();

    info!("Starting the command organiser...");

    let populated = cli::populate_db().await;

    if let Err(e) = populated {
        error!("Failed to populate the db from file: {e}");
    }

    let _ = ui::tui::run_terminal().await;
}

#[cfg(test)]
pub mod tests {

    use crate::model::command::Command;
    use crate::storage::command_storage::CommandStorageManager;

    #[tokio::test]
    #[ignore]
    async fn populate_db() {
        let manager = CommandStorageManager::new("commands.db").await.unwrap();

        let command1 = Command {
            executable: "git".to_string(),
            command: "git pull".to_string(),
            alias: "git_pull".to_string(),
            description: Some("Just a git pull".to_string()),
        };

        let command2 = Command {
            executable: "ssh".to_string(),
            command: "ssh --version".to_string(),
            alias: "ssh_version".to_string(),
            description: Some("Just a ssh version".to_string()),
        };

        let command3 = Command {
            executable: "ls".to_string(),
            command: "ls .".to_string(),
            alias: "ls_current".to_string(),
            description: Some("Just a ls".to_string()),
        };

        let command4 = Command {
            executable: "ls".to_string(),
            command: "ls -a".to_string(),
            alias: "ls_all".to_string(),
            description: Some("Just a ls all".to_string()),
        };
        let command5 = Command {
            executable: "ls".to_string(),
            command: "ls ..".to_string(),
            alias: "ls_previous".to_string(),
            description: Some("Just a ls previous".to_string()),
        };
        let command6 = Command {
            executable: "ls".to_string(),
            command: "ls --version".to_string(),
            alias: "ls_version".to_string(),
            description: Some("Just a ls version".to_string()),
        };

        manager.insert_command(command1.clone()).await.unwrap();
        manager.insert_command(command2.clone()).await.unwrap();
        manager.insert_command(command3.clone()).await.unwrap();
        manager.insert_command(command4.clone()).await.unwrap();
        manager.insert_command(command5.clone()).await.unwrap();
        manager.insert_command(command6.clone()).await.unwrap();
    }
}
