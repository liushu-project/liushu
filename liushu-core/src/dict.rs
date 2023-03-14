use std::{fs::File, path::Path};

use patricia_tree::PatriciaMap;
use redb::TableDefinition;
use rusqlite::{params, Connection};
use serde::Deserialize;

use crate::{dirs::PROJECT_DIRS, error::LiushuError};

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
    let mut trie = PatriciaMap::new();
    {
        let mut dict_table = tx.open_table(DICTIONARY)?;
        for dict_path in dict_paths {
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .comment(Some(b'#'))
                .from_path(dict_path)?;
            for result in rdr.deserialize() {
                let DictItem {
                    text,
                    code,
                    weight,
                    comment,
                } = result?;
                dict_table.insert(text.as_str(), (weight, comment.as_deref()))?;

                if trie.get(&code).is_none() {
                    trie.insert_str(code.as_str(), vec![text]);
                } else if let Some(entry) = trie.get_mut(code.as_str()) {
                    entry.push(text);
                }
            }
        }
    }
    tx.commit()?;

    // TODO: remove hard code
    let trie_writer = File::create(PROJECT_DIRS.target_dir.join("sunman.trie"))?;
    bincode::serialize_into(trie_writer, &trie)?;

    Ok(())
}
