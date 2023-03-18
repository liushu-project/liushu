pub mod state;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use patricia_tree::PatriciaMap;
use redb::{Database, ReadableTable};

use crate::{dict::DICTIONARY, error::LiushuError};

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

impl InputMethodEngine for Engine {
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
            .sorted_by_key(|i| i.weight)
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
