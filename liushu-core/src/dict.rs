use std::fs::create_dir;
use std::io;

use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Deserialize;

use crate::dirs::PROJECT_DIRS;

#[derive(Debug, Deserialize)]
struct DictItem {
    text: String,
    code: String,
    weight: u64,
    stem: Option<String>,
    comment: Option<String>,
}

pub struct SearchEngine {
    conn: Connection,
}

impl SearchEngine {
    pub fn new() -> Self {
        let data_dir = &PROJECT_DIRS.data_dir;
        let db_path = data_dir.join("sunman.db3");

        Self {
            conn: Connection::open(db_path).unwrap(),
        }
    }

    pub fn search(&self, mut code: String, page: u32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT text FROM sunman WHERE code LIKE ?1 ORDER BY weight DESC Limit 9 OFFSET ?2",
        )?;

        code.push('%');
        let offset = (page - 1) * 9;
        let rows = stmt.query_map(params![code, offset], |row| row.get("text"))?;

        let mut result = Vec::new();
        for text_result in rows {
            result.push(text_result?);
        }

        Ok(result)
    }
}

pub fn compile_dict() -> Result<()> {
    let data_dir = &PROJECT_DIRS.data_dir;
    if !data_dir.exists() {
        create_dir(data_dir)?;
    }
    let db_path = data_dir.join("sunman.db3");
    let mut conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE sunman (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            code TEXT NOT NULL,
            weight INTEGER NOT NULL,
            stem TEXT,
            comment TEXT
        )",
        (),
    )?;
    let tx = conn.transaction()?;
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let dict: DictItem = result?;
        tx.execute(
            "INSERT INTO sunman (text, code, weight, stem, comment) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![dict.text, dict.code, dict.weight, dict.stem, dict.comment],
        )?;
    }
    tx.commit()?;
    Ok(())
}
