use crate::{config::Config, dirs::PROJECT_DIRS};

pub fn deploy() {
    let targe_dir = &PROJECT_DIRS.target_dir;
    let config = Config::load();

    for formula in config.formulas {
        let dict_db_path = targe_dir.join(formula.get_db_path());
        formula.compile_dict_to(dict_db_path).unwrap();
    }
}
