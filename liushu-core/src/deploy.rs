use std::fs::File;

use patricia_tree::PatriciaMap;

use crate::{
    dict::{DictItem, DICTIONARY},
    dirs::MyProjectDirs,
    error::LiushuError,
};

pub fn deploy(proj_dirs: &MyProjectDirs) -> Result<(), LiushuError> {
    let self_config_dir = &proj_dirs.config_dir.join("sunman");
    let db_path = &proj_dirs.target_dir.join("sunman.redb");

    let table = redb::Database::create(db_path)?;
    let tx = table.begin_write()?;
    let mut trie = PatriciaMap::new();
    {
        let mut dict_table = tx.open_table(DICTIONARY)?;
        let dictionaries = [
            "words.dict.tsv",
            "phrases.brief.dict.tsv",
            "phrases.core.dict.tsv",
        ];
        for dict_path in dictionaries {
            let dict_path = self_config_dir.join(dict_path);
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
    let trie_path = &proj_dirs.target_dir.join("sunman.trie");
    let trie_writer = File::create(trie_path)?;
    bincode::serialize_into(trie_writer, &trie)?;

    Ok(())
}
