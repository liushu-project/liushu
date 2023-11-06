pub mod candidates;
mod segmentor;
mod translator;

use std::{fs::File, path::Path};

use patricia_tree::PatriciaMap;

use crate::{dict::DictItem, dirs::MyProjectDirs, error::LiushuError};

use self::{candidates::Candidate, translator::Translator};

pub trait InputMethodEngine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError>;
}

#[derive(Debug)]
pub struct Engine {
    trie: PatriciaMap<Vec<DictItem>>,
}

impl Engine {
    pub fn new(dict_path: impl AsRef<Path>) -> Result<Self, LiushuError> {
        let trie: PatriciaMap<Vec<DictItem>> = bincode::deserialize_from(File::open(dict_path)?)?;

        Ok(Self { trie })
    }

    pub fn init(proj_dirs: &MyProjectDirs) -> Result<Self, LiushuError> {
        let trie: PatriciaMap<Vec<DictItem>> =
            bincode::deserialize_from(File::open(proj_dirs.target_dir.join("sunman.trie"))?)?;

        Ok(Self { trie })
    }
}

impl InputMethodEngine for Engine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError> {
        Ok(self.trie.translate(code))
    }
}
