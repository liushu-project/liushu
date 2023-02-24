use std::path::Path;

use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DictItem {
    pub text: String,
    pub code: String,
    pub weight: u64,
    pub stem: Option<String>,
    pub comment: Option<String>,
}

pub fn compile_dict_to_db<P: AsRef<Path>>(dict_path: P, db_path: P) -> Result<()> {
    let mut conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE dict (
                id INTEGER PRIMARY KEY,
                text TEXT NOT NULL,
                code TEXT NOT NULL,
                weight INTEGER NOT NULL,
                stem TEXT,
                comment TEXT,
                UNIQUE(text, code)
            )",
        (),
    )?;
    let tx = conn.transaction()?;
    let mut rdr = csv::Reader::from_path(dict_path)?;
    for result in rdr.deserialize() {
        let dict: DictItem = result?;
        tx.execute(
            "INSERT INTO dict (text, code, weight, stem, comment) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![dict.text, dict.code, dict.weight, dict.stem, dict.comment],
        )?;
    }
    tx.commit()?;
    Ok(())
}
