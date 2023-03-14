use std::path::Path;

use redb::TableDefinition;
use rusqlite::{params, Connection};
use serde::Deserialize;

use crate::error::LiushuError;

pub const DICTIONARY: TableDefinition<(&str, &str), (u64, Option<&str>)> =
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

pub fn compile_dicts_to_db(
    dict_paths: impl IntoIterator<Item = impl AsRef<Path>>,
    db_path: impl AsRef<Path>,
) -> Result<(), LiushuError> {
    let mut conn = Connection::open(db_path)?;
    conn.execute(CREATE_DICT_TABLE_SQL, ())?;
    let tx = conn.transaction()?;
    for dict_path in dict_paths {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .comment(Some(b'#'))
            .from_path(dict_path)?;
        for result in rdr.deserialize() {
            let dict: DictItem = result?;
            tx.execute(
                "INSERT INTO dict (text, code, weight, comment) VALUES (?1, ?2, ?3, ?4)",
                params![dict.text, dict.code, dict.weight, dict.comment],
            )?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn compile_dicts_to_db2(
    dict_paths: impl IntoIterator<Item = impl AsRef<Path>>,
    db_path: impl AsRef<Path>,
) -> Result<(), LiushuError> {
    let table = redb::Database::create(db_path)?;
    let tx = table.begin_write()?;
    {
        let mut dict_table = tx.open_table(DICTIONARY)?;
        for dict_path in dict_paths {
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .comment(Some(b'#'))
                .from_path(dict_path)?;
            for result in rdr.deserialize() {
                let dict_item: DictItem = result?;
                dict_table.insert(
                    &(dict_item.code.as_str(), dict_item.text.as_str()),
                    (dict_item.weight, dict_item.comment.as_deref()),
                )?;
            }
        }
    }
    tx.commit()?;

    Ok(())
}
