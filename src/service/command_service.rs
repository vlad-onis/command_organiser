use thiserror::Error;

use crate::model::command::Command;
use crate::storage::command_storage::{CommandStorageError, CommandStorageManager};

#[derive(Debug, Error)]
pub enum CommandServiceError {
    #[error("Failed to construct the storage manager : {0}")]
    StorageManagerConstruction(CommandStorageError),

    #[error("Failed to insert a command : {0}")]
    StorageManagerInsert(CommandStorageError),

    #[error("Unable to parse the executable out of the given command")]
    NoExecutable,
}

pub struct CommandService {
    storage_manager: CommandStorageManager,
}

impl CommandService {
    pub async fn new(db_url: &str) -> Result<Self, CommandServiceError> {
        let storage_manager = CommandStorageManager::new(db_url)
            .await
            .map_err(|e| CommandServiceError::StorageManagerConstruction(e))?;

        Ok(CommandService { storage_manager })
    }

    pub async fn insert_command(
        &self,
        command: &str,
        alias: &str,
        description: Option<String>,
    ) -> Result<Command, CommandServiceError> {
        let executable = command
            .split(' ')
            .collect::<Vec<&str>>()
            .first()
            .ok_or(CommandServiceError::NoExecutable)?
            .to_owned();

        let command = Command::new(
            executable.to_string(),
            command.to_string(),
            alias.to_string(),
            description,
        );
        self.storage_manager
            .insert_command(command.clone())
            .await
            .map_err(|e| CommandServiceError::StorageManagerInsert(e))?;

        Ok(command)
    }
}

#[cfg(test)]
mod tests {

    use serial_test::serial;

    use super::CommandService;

    #[tokio::test]
    #[serial]
    async fn test_service_construction() {
        let _ = CommandService::new("test.sqlite").await.unwrap();

        assert!(std::fs::metadata("test.sqlite").unwrap().is_file());
        std::fs::remove_file("test.sqlite").unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_insertion() {
        let service = CommandService::new("test.sqlite").await.unwrap();

        let inserted = service
            .insert_command("test command arguments", "my_test", None)
            .await
            .unwrap();

        assert_eq!(inserted.executable, "test".to_string());
        assert_eq!(inserted.command, "test command arguments".to_string());

        std::fs::remove_file("test.sqlite").unwrap();
    }
}
