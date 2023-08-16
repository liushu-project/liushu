pub mod candidates;
mod segmentor;

use std::fs::File;

use itertools::Itertools;
use patricia_tree::PatriciaMap;

use crate::{dict::DictItem, dirs::MyProjectDirs, error::LiushuError};

use self::candidates::{Candidate, CandidateSource};

pub trait InputMethodEngine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError>;
}

#[derive(Debug)]
pub struct Engine {
    trie: PatriciaMap<Vec<DictItem>>,
}

impl Engine {
    pub fn init(proj_dirs: &MyProjectDirs) -> Result<Self, LiushuError> {
        let trie: PatriciaMap<Vec<DictItem>> =
            bincode::deserialize_from(File::open(proj_dirs.target_dir.join("sunman.trie"))?)?;

        Ok(Self { trie })
    }
}

impl InputMethodEngine for Engine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError> {
        if code.is_empty() {
            return Ok(vec![]);
        }

        let result: Vec<Candidate> = self
            .trie
            .iter_prefix(code.as_bytes())
            .flat_map(|(_, value)| {
                value.iter().map(|item| Candidate {
                    text: item.text.clone(),
                    code: item.code.clone(),
                    comment: item.comment.clone(),
                    weight: item.weight,
                    source: CandidateSource::CodeTable,
                })
            })
            .unique_by(|i| i.text.clone())
            .sorted_by_key(|i| std::cmp::Reverse(i.weight))
            .collect();

        Ok(result)
    }
}
