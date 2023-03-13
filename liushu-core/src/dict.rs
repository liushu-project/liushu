use std::path::Path;

use rusqlite::{params, Connection};
use serde::Deserialize;

use crate::error::LiushuError;

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

pub fn compile_dicts_to_db<P: AsRef<Path>>(
    dict_paths: Vec<P>,
    db_path: P,
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
