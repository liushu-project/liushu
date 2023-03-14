use redb::TableDefinition;
use serde::Deserialize;
pub const DICTIONARY: TableDefinition<&str, (u64, Option<&str>)> =
    TableDefinition::new("dictionary");

pub const CREATE_DICT_TABLE_SQL: &str = r#"
    CREATE TABLE dict (
        id INTEGER PRIMARY KEY,
        text TEXT NOT NULL,
        code TEXT NOT NULL,
        weight INTEGER NOT NULL,
        comment TEXT,
        UNIQUE(text, code)
    )
"#;

#[derive(Debug, Deserialize)]
pub struct DictItem {
    pub text: String,
    pub code: String,
    pub weight: u64,
    pub comment: Option<String>,
}
