pub mod candidates;
mod segmentor;

use std::fs::File;

use itertools::Itertools;
use patricia_tree::PatriciaMap;
use redb::{Database, ReadableTable};

use crate::{dict::DICTIONARY, dirs::MyProjectDirs, error::LiushuError};

use self::candidates::{Candidate, CandidateSource};

pub trait InputMethodEngine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError>;
}

#[derive(Debug)]
pub struct Engine {
    db: Database,
    trie: PatriciaMap<Vec<String>>,
}

impl Engine {
    pub fn init(proj_dirs: &MyProjectDirs) -> Result<Self, LiushuError> {
        let db = Database::open(proj_dirs.target_dir.join("sunman.redb"))?;
        let trie: PatriciaMap<Vec<String>> =
            bincode::deserialize_from(File::open(proj_dirs.target_dir.join("sunman.trie"))?)?;

        Ok(Self { db, trie })
    }
}

impl InputMethodEngine for Engine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError> {
        let tx = self.db.begin_read()?;
        let dictionary = tx.open_table(DICTIONARY)?;

        let result: Vec<Candidate> = self
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
            .unique_by(|i| i.text.clone())
            .sorted_by_key(|i| std::cmp::Reverse(i.weight))
            .collect();

        Ok(result)
    }
}
