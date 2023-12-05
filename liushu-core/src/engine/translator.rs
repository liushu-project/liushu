use itertools::Itertools;

use crate::dict::Dictionary;

use super::candidates::Candidate;

pub trait Translator {
    fn translate(&self, code: &str) -> Vec<Candidate>;
}

impl Translator for Dictionary {
    fn translate(&self, code: &str) -> Vec<Candidate> {
        if code.is_empty() {
            return vec![];
        }

        self.iter_prefix(code)
            .flat_map(|(_, value)| {
                value.iter().map(|item| Candidate {
                    text: item.text.clone(),
                    code: item.code.clone(),
                    comment: item.comment.clone(),
                    weight: item.weight,
                })
            })
            .unique_by(|i| i.text.clone())
            .sorted_by_key(|i| std::cmp::Reverse(i.weight))
            .collect()
    }
}
