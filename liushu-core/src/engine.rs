pub mod state;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use patricia_tree::PatriciaMap;
use redb::{Database, ReadableTable};
use rusqlite::{params, Connection, Result as SqlResult, Row};

use crate::{dict::DICTIONARY, dirs::PROJECT_DIRS, error::LiushuError};

use self::state::State;

pub trait InputMethodEngine {
    fn search(&self, code: &str) -> Result<Vec<SearchResultItem>, LiushuError>;
}

#[derive(Debug)]
pub struct ShapeCodeEngine {
    conn: Connection,
}

impl ShapeCodeEngine {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

impl InputMethodEngine for ShapeCodeEngine {
    fn search(&self, code: &str) -> Result<Vec<SearchResultItem>, LiushuError> {
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

impl Default for ShapeCodeEngine {
    fn default() -> Self {
        let db_dir = &PROJECT_DIRS.target_dir;
        let db_path = db_dir.join("sunman.db3");
        let conn = Connection::open(db_path).unwrap();
        Self::new(conn)
    }
}

pub struct EngineWithRedb {
    db: Database,
    trie: PatriciaMap<Vec<String>>,
}

impl EngineWithRedb {
    pub fn with(path: impl AsRef<Path>) -> Result<Self, LiushuError> {
        let path = path.as_ref();
        let db = Database::open(path.join("sunman.redb"))?;
        let trie: PatriciaMap<Vec<String>> =
            bincode::deserialize_from(File::open(path.join("sunman.trie"))?)?;

        Ok(Self { db, trie })
    }
}

impl InputMethodEngine for EngineWithRedb {
    fn search(&self, code: &str) -> Result<Vec<SearchResultItem>, LiushuError> {
        let tx = self.db.begin_read()?;
        let dictionary = tx.open_table(DICTIONARY)?;
        Ok(self
            .trie
            .iter_prefix(code.as_bytes())
            .flat_map(|(key, value)| {
                let dictionary = &dictionary;
                value.iter().map(move |text| {
                    let code = String::from_utf8(key.clone()).unwrap();
                    dictionary.get(text.as_str()).map(|a| {
                        a.map(|v| {
                            let (weight, comment) = v.value();
                            SearchResultItem {
                                code: code.clone(),
                                text: text.clone(),
                                weight,
                                comment: comment.map(|c| c.to_owned()),
                            }
                        })
                    })
                })
            })
            .filter_map(|v| v.ok().flatten())
            .collect())
    }
}

pub struct NewStyleEngine {
    target_dir: PathBuf,
    state: State,
    db: Database,
    trie: PatriciaMap<Vec<String>>,
}

impl NewStyleEngine {
    pub fn init(
        data_dir: impl AsRef<Path>,
        target_dir: impl AsRef<Path>,
    ) -> Result<Self, LiushuError> {
        let data_dir = data_dir.as_ref();
        let target_dir = target_dir.as_ref();
        let state: State = bincode::deserialize_from(File::open(data_dir.join(".state"))?)?;

        let active_formula = state.get_active_formula().unwrap();
        let db = Database::open(target_dir.join(format!("{}.redb", active_formula.id)))?;
        let trie: PatriciaMap<Vec<String>> = bincode::deserialize_from(File::open(
            target_dir.join(format!("{}.trie", active_formula.id)),
        )?)?;

        Ok(Self {
            target_dir: target_dir.to_path_buf(),
            state,
            db,
            trie,
        })
    }

    pub fn set_active_formula(&mut self, formula_id: &str) -> Result<(), LiushuError> {
        self.state.set_active_formula(formula_id);
        let active_formula = self.state.get_active_formula().unwrap();
        let db = Database::open(self.target_dir.join(format!("{}.redb", active_formula.id)))?;
        let trie: PatriciaMap<Vec<String>> = bincode::deserialize_from(File::open(
            self.target_dir.join(format!("{}.trie", active_formula.id)),
        )?)?;

        self.db = db;
        self.trie = trie;

        Ok(())
    }
}

impl InputMethodEngine for NewStyleEngine {
    fn search(&self, code: &str) -> Result<Vec<SearchResultItem>, LiushuError> {
        let tx = self.db.begin_read()?;
        let dictionary = tx.open_table(DICTIONARY)?;
        Ok(self
            .trie
            .iter_prefix(code.as_bytes())
            .flat_map(|(key, value)| {
                let dictionary = &dictionary;
                value.iter().map(move |text| {
                    let code = String::from_utf8(key.clone()).unwrap();
                    dictionary.get(text.as_str()).map(|a| {
                        a.map(|v| {
                            let (weight, comment) = v.value();
                            SearchResultItem {
                                code: code.clone(),
                                text: text.clone(),
                                weight,
                                comment: comment.map(|c| c.to_owned()),
                            }
                        })
                    })
                })
            })
            .filter_map(|v| v.ok().flatten())
            .collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct SearchResultItem {
    pub text: String,
    pub code: String,
    pub weight: u64,
    pub comment: Option<String>,
}

impl TryFrom<&Row<'_>> for SearchResultItem {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> SqlResult<Self> {
        Ok(Self {
            text: row.get("text")?,
            code: row.get("code")?,
            weight: row.get("weight")?,
            comment: row.get("comment").ok(),
        })
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::{params, Connection};

    use crate::dict::CREATE_DICT_TABLE_SQL;

    use super::*;

    #[test]
    fn test_search() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(CREATE_DICT_TABLE_SQL, ()).unwrap();
        conn.execute(
            "INSERT INTO dict (text, code, weight, comment) VALUES (?1, ?2, ?3, ?4)",
            params!["你好", "ni hao", 1, None::<String>],
        )
        .unwrap();

        let engine = ShapeCodeEngine::new(conn);

        let result = engine.search("ni hao").unwrap();
        assert_eq!(
            result,
            vec![SearchResultItem {
                text: "你好".to_string(),
                code: "ni hao".to_string(),
                weight: 1,
                comment: None,
            }]
        );

        let not_found = engine.search("hello");
        assert!(not_found.is_ok());
        assert_eq!(not_found.unwrap(), Vec::new());
    }
}
