use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Command {
    pub command: String,
    pub executable: String,
}
