use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use data_encoding::HEXLOWER;
use patricia_tree::PatriciaMap;
use serde::{Deserialize, Serialize};
use serde_dhall::StaticType;
use sha2::{Digest, Sha256};

use crate::{
    dict::{DictItem, DICTIONARY},
    dirs::{MyProjectDirs, PROJECT_DIRS},
    error::LiushuError,
    hmm::train_to_db,
};

#[derive(Debug, Serialize, Deserialize, StaticType)]
pub struct Config {
    pub formulas: Vec<Formula>,
}

impl Config {
    pub fn digest(&self) -> String {
        let string = format!("{:?}", self);
        let mut hasher = Sha256::new();
        hasher.update(string.as_bytes());
        let result = hasher.finalize();

        HEXLOWER.encode(result.as_ref())
    }

    pub fn load() -> Self {
        Self::load_from_path(PROJECT_DIRS.config_dir.join("main.dhall"))
    }

    fn load_from_path<P: AsRef<Path>>(path: P) -> Self {
        serde_dhall::from_file(path)
            .static_type_annotation()
            .parse()
            .unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, StaticType)]
pub struct Formula {
    pub id: String,
    pub name: Option<String>,
    pub use_hmm: bool,
    pub dictionaries: Vec<String>,
}

impl Formula {
    pub fn compile(&self, proj_dirs: &MyProjectDirs) -> Result<(), LiushuError> {
        let self_config_dir = &proj_dirs.config_dir.join(&self.id);
        let db_path = &proj_dirs.target_dir.join(format!("{}.redb", self.id));

        let table = redb::Database::create(db_path)?;
        let tx = table.begin_write()?;
        let mut trie = PatriciaMap::new();
        {
            let mut dict_table = tx.open_table(DICTIONARY)?;
            for dict_path in &self.dictionaries {
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

        if self.use_hmm {
            // TODO: remove hardcord
            train_to_db(self_config_dir.join("corpus.tsv"), &table, &mut trie)?;
        }

        let trie_path = &proj_dirs.target_dir.join(format!("{}.trie", self.id));
        let trie_writer = File::create(trie_path)?;
        bincode::serialize_into(trie_writer, &trie)?;
        Ok(())
    }

    pub fn get_dictionaries_digest(&self, config_dir: impl AsRef<Path>) -> Option<String> {
        sha256_files(
            self.dictionaries
                .iter()
                .map(|p| config_dir.as_ref().join(&self.id).join(p)),
        )
        .ok()
    }
}

fn sha256_files(paths: impl IntoIterator<Item = PathBuf>) -> Result<String, LiushuError> {
    let mut hasher = Sha256::new();
    for path in paths {
        let input = File::open(path)?;
        let mut reader = BufReader::new(input);

        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
    }
    let digest = hasher.finalize();
    Ok(HEXLOWER.encode(digest.as_ref()))
}
