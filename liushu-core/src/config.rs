use std::path::{Path, PathBuf};

use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::dict::DictItem;
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
    pub id: String,
    dictionary: String,
}

impl Formula {
    pub fn compile_dict_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE dict (
                id INTEGER PRIMARY KEY,
                text TEXT NOT NULL,
                code TEXT NOT NULL,
                weight INTEGER NOT NULL,
                stem TEXT,
                comment TEXT,
                UNIQUE(text, code)
            )",
            (),
        )?;
        let tx = conn.transaction()?;
        let mut rdr = csv::Reader::from_path(self.get_dict_path())?;
        for result in rdr.deserialize() {
            let dict: DictItem = result?;
            tx.execute(
                "INSERT INTO dict (text, code, weight, stem, comment) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![dict.text, dict.code, dict.weight, dict.stem, dict.comment],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_db_path(&self) -> PathBuf {
        PathBuf::from(format!("{}.db3", self.id))
    }

    fn get_dict_path(&self) -> PathBuf {
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
        assert_eq!(
            config.formulas[0].get_db_path(),
            PathBuf::from("sunman.db3")
        );
    }
}
