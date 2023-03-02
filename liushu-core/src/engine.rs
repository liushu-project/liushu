use anyhow::Result;
use rusqlite::{params, Connection, Result as SqlResult, Row};

use crate::dirs::PROJECT_DIRS;

#[derive(Debug)]
pub struct Engine {
    conn: Connection,
}

impl Engine {
    pub fn new(conn: Connection) -> Self {
        // TODO: load db by config
        Self { conn }
    }

    pub fn search(&self, code: &str) -> Result<Vec<SearchResultItem>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT * FROM (SELECT * FROM dict WHERE code LIKE ?1) GROUP BY text ORDER BY weight DESC",
        )?;

        let code = code.to_string() + "%";
        let rows = stmt.query_map(params![code], |row| SearchResultItem::try_from(row))?;

        let mut result = Vec::new();
        for text_result in rows {
            result.push(text_result?);
        }

        Ok(result)
    }
}

impl Default for Engine {
    fn default() -> Self {
        let db_dir = &PROJECT_DIRS.target_dir;
        let db_path = db_dir.join("sunman.db3");
        let conn = Connection::open(db_path).unwrap();
        Self::new(conn)
    }
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use rusqlite::{params, Connection};

    use super::{Engine, SearchResultItem};

    #[test]
    fn test_search() {
        let conn = Connection::open_in_memory().unwrap();
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
        )
        .unwrap();
        conn.execute(
            "INSERT INTO dict (text, code, weight, stem, comment) VALUES (?1, ?2, ?3, ?4, ?5)",
            params!["你好", "ni hao", 1, None::<String>, None::<String>],
        )
        .unwrap();

        let engine = Engine::new(conn);

        let result = engine.search("ni hao").unwrap();
        assert_eq!(
            result,
            vec![SearchResultItem {
                text: "你好".to_string(),
                code: "ni hao".to_string(),
                weight: 1,
                stem: None,
                comment: None,
            }]
        );

        let not_found = engine.search("hello");
        assert!(not_found.is_ok());
        assert_eq!(not_found.unwrap(), Vec::new());
    }
}
