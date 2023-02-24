use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::dirs::PROJECT_DIRS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub formulas: Vec<Formula>,
}

impl Config {
    pub fn load() -> Self {
        Self::load_from_path(&PROJECT_DIRS.config_dir.join("main.dhall"))
    }

    fn load_from_path<P: AsRef<Path>>(path: P) -> Self {
        serde_dhall::from_file(path).parse().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Formula {
    id: String,
    dictionary: String,
}

impl Formula {
    pub fn get_db_path(&self) -> PathBuf {
        let targe_dir = &PROJECT_DIRS.target_dir;
        targe_dir.join(format!("{}.db3", self.id))
    }

    pub fn get_dict_path(&self) -> PathBuf {
        let config_dir = &PROJECT_DIRS.config_dir;
        config_dir
            .join(&self.id)
            .join(format!("{}.dict.csv", self.dictionary))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude() {
        let config = Config::load_from_path("../prelude/main.dhall");

        assert_eq!(config.formulas.len(), 1);

        assert_eq!(config.formulas[0].id, String::from("sunman"));
        assert_eq!(config.formulas[0].dictionary, String::from("words"));

        assert!(config.formulas[0]
            .get_dict_path()
            .ends_with("sunman/words.dict.csv"));
        assert!(config.formulas[0].get_db_path().ends_with("sunman.db3"));
    }
}
