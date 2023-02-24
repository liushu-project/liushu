use crate::{config::Config, dict::compile_dict_to_db};

pub fn deploy() {
    let config = Config::load();

    for formula in config.formulas {
        let db_path = formula.get_db_path();
        let dict_path = formula.get_dict_path();
        compile_dict_to_db(dict_path, db_path).unwrap();
    }
}
