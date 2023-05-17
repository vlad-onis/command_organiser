use sqlx::{migrate::MigrateDatabase, Error as SqlxError, Sqlite, SqlitePool};
use thiserror::Error;

use crate::model::command::Command;

#[derive(Debug, Error)]
pub enum CommandStorageError {
    #[error("Failed to open the connection to the db: {0}")]
    OpenConnection(#[from] SqlxError),
}

#[allow(dead_code)]
pub struct CommandStorageManager {
    connection_pool: SqlitePool,
}

impl CommandStorageManager {
    #[allow(dead_code)]
    pub async fn new(db_url: &str) -> Result<CommandStorageManager, CommandStorageError> {
        let pool = CommandStorageManager::db_setup(db_url).await?;

        Ok(CommandStorageManager {
            connection_pool: pool,
        })
    }

    #[allow(dead_code)]
    pub async fn db_setup(db_url: &str) -> Result<SqlitePool, CommandStorageError> {
        // Create the db file
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            Sqlite::create_database(db_url).await?;
        } else {
            println!("Database already exists");
        }

        // create the db connection pool
        let db = SqlitePool::connect(db_url).await?;

        // Create the command tables
        let _query_result = sqlx::query(
            "CREATE TABLE IF NOT EXISTS commands \
            (command VARCHAR(250) NOT NULL UNIQUE, \
            executable VARCHAR(50) NOT NULL UNIQUE, \
            alias VARCHAR(20) NOT NULL UNIQUE, \
            description VARCHAR(300) NULL);",
        )
        .execute(&db)
        .await?;

        Ok(db)
    }

    pub async fn get_all_commands(&self) -> Result<Vec<Command>, CommandStorageError> {
        let commands = sqlx::query_as::<_, Command>("SELECT * FROM commands")
            .fetch_all(&self.connection_pool)
            .await?;

        Ok(commands.into_iter().collect())
    }

    pub async fn get_commands_by_executable(
        &self,
        executable: String,
    ) -> Result<Vec<Command>, CommandStorageError> {
        let commands = sqlx::query_as::<_, Command>("SELECT * FROM commands where executable=?")
            .bind(executable)
            .fetch_all(&self.connection_pool)
            .await?;

        Ok(commands.into_iter().collect())
    }

    pub async fn get_command(&self, command: Command) -> Result<Command, CommandStorageError> {
        let command = sqlx::query_as::<_, Command>("SELECT * FROM commands where command=?")
            .bind(command.command)
            .fetch_one(&self.connection_pool)
            .await?;

        Ok(command)
    }

    pub async fn insert_command(&self, command: Command) -> Result<(), CommandStorageError> {
        let query_result = sqlx::query(
            "INSERT INTO commands(executable, command, alias, description) VALUES(?, ?, ?, ?);",
        )
        .bind(command.executable)
        .bind(command.command)
        .bind(command.alias)
        .bind(command.description)
        .execute(&self.connection_pool)
        .await?;

        Ok(())
    }

    pub async fn delete_command(&self, command: Command) -> Result<(), CommandStorageError> {
        let query_result = sqlx::query("DELETE FROM commands WHERE command=?;")
            .bind(command.command)
            .execute(&self.connection_pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::CommandStorageManager;
    use crate::model::command::Command;

    #[tokio::test]
    #[serial]
    async fn test_manager_flow() {
        let manager = CommandStorageManager::new("sqlite://sqlite.db")
            .await
            .unwrap();

        let command = Command {
            executable: "git".to_string(),
            command: "git pull".to_string(),
            alias: "git_pull".to_string(),
            description: None,
        };

        manager.insert_command(command.clone()).await.unwrap();

        let commands = manager
            .get_commands_by_executable(command.executable.clone())
            .await
            .unwrap();

        println!("{:?}", commands);

        assert_eq!(commands.len(), 1);

        manager.delete_command(command).await.unwrap();

        let command = Command {
            executable: "ssh".to_string(),
            command: "ssh --version".to_string(),
            alias: "ssh_version".to_string(),
            description: None,
        };

        manager.insert_command(command.clone()).await.unwrap();

        let command = manager.get_command(command).await.unwrap();
        println!("Single command: {command:?}");

        assert_eq!(command.executable, "ssh".to_string());

        let _ = std::fs::remove_file("sqlite.db");
        let _ = std::fs::remove_file("sqlite.db-shm");
        let _ = std::fs::remove_file("sqlite.db-wal");
    }
}
