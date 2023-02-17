use std::io;

use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DictItem {
    text: String,
    code: String,
    weight: u64,
    stem: Option<String>,
    comment: Option<String>,
}

pub fn query_code(mut code: String, page: u32) -> Result<Vec<String>> {
    let conn = Connection::open("./sunman.db3")?;
    let mut stmt = conn.prepare(
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

pub fn compile_dict() -> Result<()> {
    let mut conn = Connection::open("./sunman.db3")?;
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
