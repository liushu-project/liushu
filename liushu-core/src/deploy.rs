use crate::{config::Config, dirs::PROJECT_DIRS, error::LiushuError};

pub fn deploy() -> Result<(), LiushuError> {
    let config = Config::load();

    for formula in config.formulas {
        formula.compile(&PROJECT_DIRS.config_dir, &PROJECT_DIRS.target_dir)?;
        formula.compile2(&PROJECT_DIRS.config_dir, &PROJECT_DIRS.target_dir)?;
    }

    Ok(())
}
