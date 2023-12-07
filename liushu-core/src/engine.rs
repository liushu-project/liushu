pub mod candidates;
pub mod segmentor;
pub mod translator;

use std::{fs::File, path::Path};

use crate::{dict::Dictionary, dirs::MyProjectDirs, error::LiushuError};

use self::{candidates::Candidate, segmentor::Segmentor, translator::Translator};

pub trait InputMethodEngine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError>;
}

#[derive(Debug)]
pub struct Engine {
    trie: Dictionary,
}

impl Engine {
    pub fn new(dict_path: impl AsRef<Path>) -> Result<Self, LiushuError> {
        let trie: Dictionary = bincode::deserialize_from(File::open(dict_path)?)?;

        Ok(Self { trie })
    }

    pub fn init(proj_dirs: &MyProjectDirs) -> Result<Self, LiushuError> {
        let trie: Dictionary =
            bincode::deserialize_from(File::open(proj_dirs.target_dir.join("sunman.trie"))?)?;

        Ok(Self { trie })
    }
}

impl InputMethodEngine for Engine {
    fn search(&self, code: &str) -> Result<Vec<Candidate>, LiushuError> {
        Ok(self.trie.translate(code))
    }
}

impl Segmentor for Engine {
    fn segment(&self, code: &str) -> Vec<String> {
        self.trie.segment(code)
    }
}
