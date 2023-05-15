use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct State {
    active_formula_id: String,
    avaliable_formulas: HashMap<String, Formula>,
    pub digest: String,
}

impl State {
    pub fn get_active_formula_id(&self) -> &String {
        &self.active_formula_id
    }

    pub fn set_active_formula(&mut self, formula_id: &str) {
        if self.avaliable_formulas.get(formula_id).is_some() {
            self.active_formula_id = formula_id.to_owned();
        }
    }

    pub fn get_active_formula(&self) -> &Formula {
        &self.avaliable_formulas[&self.active_formula_id]
    }

    pub fn get_avaliable_formula(&self, formula_id: &str) -> Option<&Formula> {
        self.avaliable_formulas.get(formula_id)
    }

    pub fn set_dictionaries_digest(&mut self, formula_id: &str, digest: &str) {
        if let Some(x) = self.avaliable_formulas.get_mut(formula_id) {
            x.dict_digest = digest.to_string();
        }
    }

    pub fn set_hmm_model_digest(&mut self, formula_id: &str, digest: &str) {
        if let Some(x) = self.avaliable_formulas.get_mut(formula_id) {
            x.hmm_model_digest = digest.to_string();
        }
    }
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Formula {
    pub id: String,
    pub name: Option<String>,
    pub use_hmm: bool,
    pub hmm_model_digest: String,
    pub dict_digest: String,
}

impl From<&Config> for State {
    fn from(config: &Config) -> Self {
        Self {
            active_formula_id: config.formulas[0].id.clone(),
            avaliable_formulas: HashMap::from_iter(config.formulas.iter().map(|f| {
                (
                    f.id.clone(),
                    Formula {
                        id: f.id.clone(),
                        name: f.name.clone(),
                        use_hmm: f.use_hmm,
                        ..Default::default()
                    },
                )
            })),
            digest: config.digest(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formula_setter_getter() {
        let formula1_id = "Formula1";
        let formula2_id = "Formula2";
        let formula3_id = "Formula3";
        let mut state = State {
            active_formula_id: formula1_id.to_string(),
            avaliable_formulas: HashMap::from([
                (
                    formula1_id.to_string(),
                    Formula {
                        id: formula1_id.to_string(),
                        use_hmm: true,
                        ..Default::default()
                    },
                ),
                (
                    formula2_id.to_string(),
                    Formula {
                        id: formula2_id.to_string(),
                        ..Default::default()
                    },
                ),
            ]),
            digest: "".to_string(),
        };

        assert_eq!(
            state.get_active_formula(),
            &Formula {
                id: formula1_id.to_string(),
                use_hmm: true,
                name: None,
                ..Default::default()
            }
        );

        state.set_active_formula(formula2_id);
        assert_eq!(
            state.get_active_formula(),
            &Formula {
                id: formula2_id.to_string(),
                use_hmm: false,
                name: None,
                ..Default::default()
            }
        );

        state.set_active_formula(formula3_id);
        assert_eq!(&state.active_formula_id, formula2_id);
        assert_eq!(
            state.get_active_formula(),
            &Formula {
                id: formula2_id.to_string(),
                use_hmm: false,
                name: None,
                ..Default::default()
            }
        );
    }
}
