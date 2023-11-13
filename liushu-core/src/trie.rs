use std::cmp::Ordering;

use sucds::mii_sequences::{EliasFano, EliasFanoBuilder};

#[derive(Default)]
pub struct EliasFanoTrieArray {
    token_ids: EliasFano,
    pointers: EliasFano,
}

impl EliasFanoTrieArray {
    pub fn build(token_ids: Vec<usize>, pointers: Vec<usize>) -> Self {
        if token_ids.is_empty() {
            return Self::default();
        }

        let token_ids = Self::build_token_sequence(token_ids, &pointers);
        let pointers = Self::build_pointers(pointers);

        Self {
            token_ids,
            pointers,
        }
    }

    fn token_id(&self, i: usize) -> usize {
        let pos = self.pointers.rank(i + 1).unwrap() - 1;
        let (b, _) = self.range(pos);
        let base = if b == 0 {
            0
        } else {
            self.token_ids.select(b - 1).unwrap()
        };
        self.token_ids.select(i).unwrap() - base
    }

    #[inline(always)]
    fn range(&self, pos: usize) -> (usize, usize) {
        (
            self.pointers.select(pos).unwrap(),
            self.pointers.select(pos + 1).unwrap(),
        )
    }

    #[inline(always)]
    fn find_token(&self, pos: usize, id: usize) -> Option<usize> {
        let (b, e) = self.range(pos);
        let base = if b == 0 {
            0
        } else {
            self.token_ids.select(b - 1).unwrap()
        };
        for i in b..e {
            let token_id = self.token_ids.select(i).unwrap() - base;
            match token_id.cmp(&id) {
                Ordering::Equal => return Some(i),
                Ordering::Greater => break,
                _ => {}
            }
        }
        None
    }

    fn num_tokens(&self) -> usize {
        self.token_ids.len()
    }

    fn num_pointers(&self) -> usize {
        self.pointers.len()
    }

    fn build_token_sequence(mut token_ids: Vec<usize>, pointers: &[usize]) -> EliasFano {
        assert_eq!(token_ids.len(), *pointers.last().unwrap());

        let mut sampled_id = 0;
        for i in 0..pointers.len() - 1 {
            let (b, e) = (pointers[i], pointers[i + 1]);
            debug_assert!(b <= e);

            for token_id in token_ids.iter_mut().take(e).skip(b) {
                *token_id += sampled_id;
            }
            if e != 0 {
                sampled_id = token_ids[e - 1];
            }
        }

        let mut token_efb = EliasFanoBuilder::new(sampled_id + 1, token_ids.len()).unwrap();
        token_efb.extend(token_ids).unwrap();
        token_efb.build()
    }

    fn build_pointers(pointers: Vec<usize>) -> EliasFano {
        let mut pointer_efb =
            EliasFanoBuilder::new(pointers.last().unwrap() + 1, pointers.len()).unwrap();
        pointer_efb.extend(pointers).unwrap();
        pointer_efb.build().enable_rank()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elias_fano_trie_array() {
        let token_ids = vec![0, 2, 1, 2, 3, 0, 3, 1, 3];
        let pointers = vec![0, 2, 5, 7, 9];
        let ta = EliasFanoTrieArray::build(token_ids.clone(), pointers.clone());

        for (i, &x) in token_ids.iter().enumerate() {
            assert_eq!(ta.token_id(i), x);
        }
        for i in 0..pointers.len() - 1 {
            assert_eq!(ta.range(i), (pointers[i], pointers[i + 1]));
        }

        assert_eq!(ta.find_token(1, 3), Some(4));
        assert_eq!(ta.find_token(1, 1), Some(2));
        assert_eq!(ta.find_token(1, 4), None);

        assert_eq!(ta.num_tokens(), 9);
        assert_eq!(ta.num_pointers(), 5);
    }
}
