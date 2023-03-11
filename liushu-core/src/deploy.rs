use crate::{config::Config, dict::compile_dicts_to_db, error::LiushuError};

pub fn deploy() -> Result<(), LiushuError> {
    let config = Config::load();

    for formula in config.formulas {
        let db_path = formula.get_db_path();
        let dict_paths = formula.get_dict_paths();
        compile_dicts_to_db(dict_paths, db_path)?;
    }

    Ok(())
}
