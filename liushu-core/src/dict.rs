use std::{fs::File, path::Path};

use patricia_tree::PatriciaMap;
use serde::{Deserialize, Serialize};

use crate::error::LiushuError;

#[derive(Debug, Deserialize, Serialize)]
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
    let mut trie = PatriciaMap::new();
    for dict_path in inputs {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .comment(Some(b'#'))
            .from_path(dict_path)?;
        for result in rdr.deserialize() {
            let item: DictItem = result?;
            let code = item.code.clone();

            if trie.get(&code).is_none() {
                trie.insert_str(code.as_str(), vec![item]);
            } else if let Some(entry) = trie.get_mut(code.as_str()) {
                entry.push(item);
            }
        }
    }

    let trie_path = target_dir.as_ref().join(format!("{}.trie", dict_name));
    let trie_writer = File::create(trie_path)?;
    bincode::serialize_into(trie_writer, &trie)?;

    Ok(())
}
