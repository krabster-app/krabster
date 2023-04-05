use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum DatabaseConfiguration {
    #[serde(rename = "sqlite")]
    SQLite(SQLiteConfiguration),
}

#[derive(Debug, Clone, Deserialize)]
pub struct SQLiteConfiguration {
    db_path: Option<String>,
}

const DB_PATH_DEFAULT: &str = "/etc/krabster/database.db";

impl Default for SQLiteConfiguration {
    fn default() -> Self {
        Self {
            db_path: Some(DB_PATH_DEFAULT.to_string()),
        }
    }
}
