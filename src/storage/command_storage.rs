use sqlx::{migrate::MigrateDatabase, Error as SqlxError, Sqlite, SqlitePool};
use thiserror::Error;

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
            "CREATE TABLE IF NOT EXISTS commands (id INTEGER PRIMARY KEY NOT NULL UNIQUE, \
            command VARCHAR(250) NOT NULL UNIQUE, executable VARCHAR(50) NOT NULL UNIQUE);",
        )
        .execute(&db)
        .await?;

        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use super::CommandStorageManager;

    #[tokio::test]
    async fn test_db_creation() {
        CommandStorageManager::new("sqlite://sqlite.db")
            .await
            .unwrap();
    }
}
