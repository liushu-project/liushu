mod candidates;
mod segmentor;
pub mod state;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use patricia_tree::PatriciaMap;
use redb::{Database, ReadableTable};

use crate::{dict::DICTIONARY, error::LiushuError, hmm::pinyin_to_sentence};

use self::{
    candidates::{Candidate, CandidateSource},
    state::State,
};

pub trait InputMethodEngine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError>;
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
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError> {
        let tx = self.db.begin_read()?;
        let dictionary = tx.open_table(DICTIONARY)?;

        let mut result: Vec<Candidate> = self
            .trie
            .iter_prefix(code.as_bytes())
            .flat_map(|(key, value)| {
                let dictionary = &dictionary;
                value.iter().map(move |text| {
                    let code = String::from_utf8_lossy(&key);
                    dictionary.get(text.as_str()).map(|a| {
                        a.map(|v| {
                            let (weight, comment) = v.value();
                            Candidate {
                                code: code.to_string(),
                                text: text.clone(),
                                weight,
                                comment: comment.map(|c| c.to_owned()),
                                source: CandidateSource::CodeTable,
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
            let pinyin_sequence = segmentor::split_pinyin(code, &self.trie);
            let predict = pinyin_to_sentence(&pinyin_sequence, &self.db, &self.trie)?;
            result.insert(
                0,
                Candidate {
                    code: code.to_string(),
                    text: predict,
                    weight: u64::MAX,
                    comment: None,
                    source: CandidateSource::Hmm,
                },
            );
        }
        Ok(result)
    }
}
