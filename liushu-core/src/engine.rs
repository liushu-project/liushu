pub mod state;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use patricia_tree::PatriciaMap;
use redb::{Database, ReadableTable};

use crate::{dict::DICTIONARY, error::LiushuError, hmm::pinyin_to_sentence};

use self::state::State;

pub trait InputMethodEngine {
    fn search(&self, code: &str) -> Result<Vec<SearchResultItem>, LiushuError>;
}

#[derive(Debug)]
pub struct Engine {
    target_dir: PathBuf,
    state: State,
    db: Database,
    trie: PatriciaMap<Vec<String>>,
}

impl Engine {
    pub fn init(data_dir: impl AsRef<Path>) -> Result<Self, LiushuError> {
        let data_dir = data_dir.as_ref();
        let target_dir = data_dir.join("target");
        let state: State = bincode::deserialize_from(File::open(data_dir.join(".state"))?)?;

        let active_formula = state.get_active_formula().unwrap();
        let db = Database::open(target_dir.join(format!("{}.redb", active_formula.id)))?;
        let trie: PatriciaMap<Vec<String>> = bincode::deserialize_from(File::open(
            target_dir.join(format!("{}.trie", active_formula.id)),
        )?)?;

        Ok(Self {
            target_dir,
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

impl InputMethodEngine for Engine {
    fn search(&self, code: &str) -> Result<Vec<SearchResultItem>, LiushuError> {
        let tx = self.db.begin_read()?;
        let dictionary = tx.open_table(DICTIONARY)?;

        let mut result: Vec<SearchResultItem> = self
            .trie
            .iter_prefix(code.as_bytes())
            .flat_map(|(key, value)| {
                let dictionary = &dictionary;
                value.iter().map(move |text| {
                    let code = String::from_utf8_lossy(&key);
                    dictionary.get(text.as_str()).map(|a| {
                        a.map(|v| {
                            let (weight, comment) = v.value();
                            SearchResultItem {
                                code: code.to_string(),
                                text: text.clone(),
                                weight,
                                comment: comment.map(|c| c.to_owned()),
                            }
                        })
                    })
                })
            })
            .filter_map(|v| v.ok().flatten())
            .sorted_by_key(|i| i.weight)
            .collect();

        let active_formula = self.state.get_active_formula().unwrap();
        if active_formula.use_hmm && code.len() > 6 {
            // TODO: better split method
            let mut pinyin_sequence = Vec::new();
            let mut code = code;
            let mut try_count = 0;
            while !code.is_empty() && try_count < 15 {
                if let Some((bytes, _)) = self.trie.get_longest_common_prefix(code) {
                    let matched = String::from_utf8_lossy(bytes);
                    let matched = matched.trim();
                    pinyin_sequence.push(matched.to_string());
                    code = &code[matched.len()..];
                }
                try_count += 1;
            }
            let predict = pinyin_to_sentence(&pinyin_sequence, &self.db, &self.trie)?;
            result.insert(
                0,
                SearchResultItem {
                    code: code.to_string(),
                    text: predict,
                    weight: 0,
                    comment: None,
                },
            );
        }
        Ok(result)
    }
}

#[derive(Debug, PartialEq)]
pub struct SearchResultItem {
    pub text: String,
    pub code: String,
    pub weight: u64,
    pub comment: Option<String>,
}
