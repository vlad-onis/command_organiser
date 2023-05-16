mod model;
mod service;
mod storage;
mod ui;

#[tokio::main]
async fn main() {
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
            description: None,
        };

        let command2 = Command {
            executable: "ssh".to_string(),
            command: "ssh --version".to_string(),
            alias: "git_version".to_string(),
            description: None,
        };

        let command3 = Command {
            executable: "ls".to_string(),
            command: "ls .".to_string(),
            alias: "ls_current".to_string(),
            description: None,
        };

        //manager.insert_command(command1.clone()).await.unwrap();
        //manager.insert_command(command2.clone()).await.unwrap();
        manager.insert_command(command3.clone()).await.unwrap();
    }
}
