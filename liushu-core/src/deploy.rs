use std::{
    fs::{self, File, OpenOptions},
    path::Path,
};

use crate::{config::Config, dirs::PROJECT_DIRS, engine::state::State, error::LiushuError};

pub fn deploy() -> Result<(), LiushuError> {
    let config = Config::load();
    let state_path = PROJECT_DIRS.data_dir.join(".state");
    if no_need_to_redeploy(&state_path, &config) {
        return Ok(());
    }

    fs::create_dir_all(&PROJECT_DIRS.target_dir)?;

    for formula in &config.formulas {
        formula.compile(&PROJECT_DIRS.config_dir, &PROJECT_DIRS.target_dir)?;
    }

    let state = State::from(&config);
    let state_writer = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&state_path)?;
    bincode::serialize_into(state_writer, &state)?;

    Ok(())
}

fn no_need_to_redeploy(state_path: impl AsRef<Path>, config: &Config) -> bool {
    File::open(state_path)
        .map_err(LiushuError::from)
        .and_then(|r| bincode::deserialize_from::<File, State>(r).map_err(LiushuError::from))
        .map(|s| s.digest == config.digest())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use crate::config::Formula;

    use super::*;

    #[test]
    fn test_no_need_to_redeploy() {
        let config1 = Config {
            formulas: vec![Formula {
                id: "test".to_string(),
                name: None,
                use_hmm: false,
                dictionaries: vec![],
            }],
        };
        let config2 = Config { formulas: vec![] };

        // Create a temporary file for testing
        let state_path = std::env::temp_dir().join("test.state");
        let state = State::from(&config1);
        let state_writer = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&state_path)
            .unwrap();
        bincode::serialize_into(state_writer, &state).unwrap();

        assert!(no_need_to_redeploy(&state_path, &config1));
        assert!(!no_need_to_redeploy(&state_path, &config2));

        fs::remove_file(&state_path).unwrap();
    }
}
