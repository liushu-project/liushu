use anyhow::Result;
use rusqlite::{params, Connection, Result as SqlResult, Row};

use crate::dirs::PROJECT_DIRS;

pub struct SearchEngine {
    conn: Connection,
}

impl SearchEngine {
    pub fn new() -> Self {
        // TODO: load db by config
        let db_dir = &PROJECT_DIRS.target_dir;
        let db_path = db_dir.join("sunman.db3");

        Self {
            conn: Connection::open(db_path).unwrap(),
        }
    }

    pub fn search(&self, mut code: String) -> Result<Vec<SearchResultItem>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT * FROM (SELECT * FROM dict WHERE code LIKE ?1 ORDER BY weight DESC) GROUP BY text",
        )?;

        code.push('%');
        let rows = stmt.query_map(params![code], |row| SearchResultItem::try_from(row))?;

        let mut result = Vec::new();
        for text_result in rows {
            result.push(text_result?);
        }

        Ok(result)
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct SearchResultItem {
    pub text: String,
    pub code: String,
    pub weight: u64,
    pub stem: Option<String>,
    pub comment: Option<String>,
}

impl TryFrom<&Row<'_>> for SearchResultItem {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> SqlResult<Self> {
        Ok(Self {
            text: row.get("text")?,
            code: row.get("code")?,
            weight: row.get("weight")?,
            stem: row.get("stem").ok(),
            comment: row.get("comment").ok(),
        })
    }
}
