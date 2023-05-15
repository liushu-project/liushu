use std::fs::{self, File, OpenOptions};

use crate::{config::Config, dirs::PROJECT_DIRS, engine::state::State, error::LiushuError};

pub fn deploy() -> Result<(), LiushuError> {
    let config = Config::load();
    let mut state = State::from(&config);

    let state_path = PROJECT_DIRS.data_dir.join(".state");
    let old_state: State = bincode::deserialize_from(File::open(&state_path)?)?;

    fs::create_dir_all(&PROJECT_DIRS.target_dir)?;

    for formula in &config.formulas {
        let old_dict_digest = old_state
            .get_avaliable_formula(&formula.id)
            .map(|f| f.dict_digest.clone());
        let new_dict_digest = formula.get_dictionaries_digest(&PROJECT_DIRS.config_dir);
        if old_dict_digest != new_dict_digest {
            formula.compile(&PROJECT_DIRS.config_dir, &PROJECT_DIRS.target_dir)?;
            state.set_dictionaries_digest(&formula.id, &new_dict_digest.unwrap());
        }
    }

    let state_writer = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&state_path)?;
    bincode::serialize_into(state_writer, &state)?;

    Ok(())
}
