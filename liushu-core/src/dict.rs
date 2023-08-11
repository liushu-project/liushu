use std::{fs::File, path::Path};

use patricia_tree::PatriciaMap;
use redb::TableDefinition;
use serde::Deserialize;

use crate::error::LiushuError;

pub const DICTIONARY: TableDefinition<&str, (u64, Option<&str>)> =
    TableDefinition::new("dictionary");

#[derive(Debug, Deserialize)]
pub struct DictItem {
    pub text: String,
    pub code: String,
    pub weight: u64,
    pub comment: Option<String>,
}

pub fn build<F, D>(inputs: Vec<F>, target_dir: D, dict_name: &str) -> Result<(), LiushuError>
where
    F: AsRef<Path>,
    D: AsRef<Path>,
{
    let redb_path = target_dir.as_ref().join(format!("{}.redb", dict_name));
    let db = redb::Database::create(redb_path)?;
    let tx = db.begin_write()?;
    let mut trie = PatriciaMap::new();
    {
        let mut dict_table = tx.open_table(DICTIONARY)?;
        for dict_path in inputs {
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

    let trie_path = target_dir.as_ref().join(format!("{}.trie", dict_name));
    let trie_writer = File::create(trie_path)?;
    bincode::serialize_into(trie_writer, &trie)?;

    Ok(())
}
