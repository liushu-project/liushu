use crate::{config::Config, dirs::PROJECT_DIRS};

pub fn deploy() {
    let targe_path = &PROJECT_DIRS.data_dir;
    let config = Config::load();

    for formula in config.formulas {
        let dict_db_path = targe_path.join(&formula.id);
        formula.compile_dict_to(dict_db_path).unwrap();
    }
}
