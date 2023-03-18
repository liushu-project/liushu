use std::fs::File;

use crate::{config::Config, dirs::PROJECT_DIRS, engine::state::State, error::LiushuError};

pub fn deploy() -> Result<(), LiushuError> {
    let config = Config::load();

    for formula in &config.formulas {
        formula.compile(&PROJECT_DIRS.config_dir, &PROJECT_DIRS.target_dir)?;
    }

    let state = State::from(config);
    let state_path = PROJECT_DIRS.data_dir.join(".state");
    let state_writer = File::create(state_path)?;
    bincode::serialize_into(state_writer, &state)?;

    Ok(())
}
