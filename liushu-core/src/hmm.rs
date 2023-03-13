mod pinyin;

use std::collections::HashMap;
use std::f64::consts::E;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use itertools::Itertools;
use redb::{Database, ReadOnlyTable, ReadableTable, TableDefinition};
use regex::Regex;

use self::pinyin::{py_split, ToPinyin, POSIBLE_PINYINS};
use crate::{
    engine::{InputMethodEngine, SearchResultItem},
    error::LiushuError,
};

const INIT_TABLE: TableDefinition<&str, f64> = TableDefinition::new("init_prob");
const TRANS_TABLE: TableDefinition<(&str, &str), f64> = TableDefinition::new("trans_prob");
const EMISS_TABLE: TableDefinition<(&str, &str), f64> = TableDefinition::new("emiss_prob");
const PINYIN_STATES: TableDefinition<&str, &str> = TableDefinition::new("pinyin_states");
const MIN_F: f64 = -3.14e100;

pub fn train(corpus_file: impl AsRef<Path>, save_to: impl AsRef<Path>) {
    let chinese_re = Regex::new(r#"[\u4e00-\u9fa5]{2,}"#).unwrap();
    let mut file = File::open(corpus_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut seqs = Vec::new();
    for seq in chinese_re.find_iter(&contents) {
        seqs.push(seq.as_str().to_string());
    }

    let db = Database::create(save_to).unwrap();
    count_init(&db, &seqs);
    count_trans(&db, &seqs);
    count_emission(&db, &seqs);
    count_pinyin_states(&db);
}

fn count_init(db: &Database, seqs: &Vec<String>) {
    let mut temp_table: HashMap<String, u64> = HashMap::new();
    let mut num = 0;
    let len = seqs.len();

    for seq in seqs {
        num += 1;
        if num % 10000 == 0 {
            println!("{}/{}", num, len);
        }
        if seq.is_empty() {
            continue;
        }

        let first_char = seq.chars().next().unwrap().to_string();
        if let Some(value) = temp_table.get_mut(&first_char) {
            *value = value.to_owned() + 1;
        } else {
            temp_table.insert(first_char, 1);
        }
    }

    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(INIT_TABLE).unwrap();
        for (key, value) in temp_table {
            let value = (value as f64 / len as f64).log(E);
            table.insert(&key.as_str(), value).unwrap();
        }
    }
    write_txn.commit().unwrap();
}

fn count_trans(db: &Database, seqs: &Vec<String>) {
    let mut temp: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let mut num = 0;
    let len = seqs.len();

    for seq in seqs {
        num += 1;
        if num % 10000 == 0 {
            println!("{}/{}", num, len);
        }
        if seq.is_empty() {
            continue;
        }

        let mut chars: Vec<String> = seq.chars().map(|c| c.to_string()).collect();
        chars.insert(0, "BOS".to_string());
        chars.push("EOS".to_string());

        for (index, post) in chars.iter().enumerate() {
            if index == 0 {
                continue;
            }

            let pre = chars[index - 1].clone();
            if temp.get(post.as_str()).is_none() {
                temp.insert(post.to_owned(), HashMap::new());
            }
            let key = temp.get_mut(post.as_str()).unwrap();
            let pre_ = pre.clone();
            (*key).insert(pre, key.get(pre_.as_str()).unwrap_or(&0).to_owned() + 1);
        }
    }

    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(TRANS_TABLE).unwrap();
        for (post, value) in temp {
            let total = value.values().sum::<u64>();
            for (pre, count) in value {
                let prob = (count as f64 / total as f64).log(E);
                table.insert((post.as_str(), pre.as_str()), prob).unwrap();
            }
        }
    }
    write_txn.commit().unwrap();
}

fn count_emission(db: &Database, seqs: &Vec<String>) {
    let mut temp: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let mut num = 0;
    let len = seqs.len();

    for seq in seqs {
        num += 1;
        if num % 10000 == 0 {
            println!("{}/{}", num, len);
        }
        if seq.is_empty() {
            continue;
        }

        let pinyin = seq.as_str().to_pinyin();
        let zip_iter = pinyin.zip(seq.chars());
        for (py, word) in zip_iter {
            if temp.get(word.to_string().as_str()).is_none() {
                temp.insert(word.to_string(), HashMap::new());
            }
            let key = temp.get_mut(word.to_string().as_str()).unwrap();
            let py_str = py.unwrap().plain();
            (*key).insert(
                py_str.to_string(),
                key.get(py_str).unwrap_or(&0).to_owned() + 1,
            );
        }
    }

    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(EMISS_TABLE).unwrap();
        for (word, pinyins) in temp {
            let total = pinyins.values().sum::<u64>();
            for (py, count) in pinyins {
                let prob = (count as f64 / total as f64).log(E);
                table.insert((word.as_str(), py.as_str()), prob).unwrap();
            }
        }
    }
    write_txn.commit().unwrap();
}

fn count_pinyin_states(db: &Database) {
    let read_txn = db.begin_read().unwrap();
    let write_txn = db.begin_write().unwrap();
    {
        let emission_table = read_txn.open_table(EMISS_TABLE).unwrap();
        let mut pinyin_states_table = write_txn.open_table(PINYIN_STATES).unwrap();
        for (key, _) in emission_table.iter().unwrap() {
            let (word, py) = key.value().to_owned();
            let mut words = pinyin_states_table
                .get(py)
                .unwrap()
                .map(|x| x.value().to_string())
                .unwrap_or("".to_string());
            words.push_str(word);
            pinyin_states_table.insert(py, words.as_str()).unwrap();
        }
    }
    write_txn.commit().unwrap();
}

#[derive(Debug)]
pub struct Hmm {
    db: Database,
}

impl Hmm {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn viterbi(
        pinyin_list: &Vec<String>,
        pinyin_states: &ReadOnlyTable<&str, &str>,
        init_prob: &ReadOnlyTable<&str, f64>,
        trans_prob: &ReadOnlyTable<(&str, &str), f64>,
        emiss_prob: &ReadOnlyTable<(&str, &str), f64>,
    ) -> Vec<(String, f64)> {
        let length = pinyin_list.len();
        let mut viterbi: HashMap<usize, HashMap<String, (f64, String)>> = HashMap::new();
        for i in 0..length {
            viterbi.insert(i, HashMap::new());
        }

        let key = pinyin_list[0].as_str();
        let chars = pinyin_states.get(key).unwrap().unwrap();
        for s in chars.value().chars() {
            let p = viterbi.get_mut(&0).unwrap();
            let init = init_prob
                .get(s.to_string().as_str())
                .unwrap()
                .map(|x| x.value())
                .unwrap_or(MIN_F);
            let emiss = emiss_prob
                .get((s.to_string().as_str(), pinyin_list[0].as_str()))
                .unwrap()
                .map(|x| x.value())
                .unwrap_or(MIN_F);
            p.insert(s.to_string(), (init + emiss, "".to_string()));
        }

        for i in 0..(length - 1) {
            let key = pinyin_list[i + 1].as_str();
            let chars = pinyin_states.get(key).unwrap().unwrap();
            for s in chars.value().chars() {
                let value = pinyin_states
                    .get(pinyin_list[i].as_str())
                    .unwrap()
                    .unwrap()
                    .value()
                    .chars()
                    .map(|c| {
                        let vit = viterbi[&i][c.to_string().as_str()].0;
                        let emission = emiss_prob
                            .get((s.to_string().as_str(), pinyin_list[i + 1].as_str()))
                            .unwrap()
                            .map(|e| e.value())
                            .unwrap_or(MIN_F);
                        let trans = trans_prob
                            .get((s.to_string().as_str(), c.to_string().as_str()))
                            .unwrap()
                            .map(|t| t.value())
                            .unwrap_or(MIN_F);
                        (vit + emission + trans, c.to_string())
                    })
                    .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                    .unwrap();
                let p = viterbi.get_mut(&(i + 1)).unwrap();
                p.insert(s.to_string(), value);
            }
        }

        let key = pinyin_list.last().unwrap().as_str();
        let last = pinyin_states.get(key).unwrap().unwrap();
        for s in last.value().chars() {
            let old = &viterbi[&(length - 1)][s.to_string().as_str()];
            let trans = trans_prob
                .get(("EOS", s.to_string().as_str()))
                .unwrap()
                .map(|x| x.value())
                .unwrap_or(MIN_F);
            let new_value = (old.0 + trans, old.1.clone());
            let p = viterbi.get_mut(&(length - 1)).unwrap();
            p.insert(s.to_string(), new_value);
        }

        viterbi[&(length - 1)]
            .iter()
            .sorted_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap())
            .rev()
            .map(|data| {
                let mut words = vec!["".to_string(); length];
                let mut weight = 0.0;
                if let Some(last) = words.last_mut() {
                    *last = data.0.clone();
                }

                for n in (0..(length - 1)).rev() {
                    let current = &viterbi[&(n + 1)][words[n + 1].to_string().as_str()];
                    words[n] = current.1.clone();
                    weight += current.0;
                }

                (words.join(""), weight)
            })
            .take(10)
            .collect_vec()
    }
}

impl InputMethodEngine for Hmm {
    fn search(&self, code: &str) -> Result<Vec<SearchResultItem>, LiushuError> {
        let possible_pinyins = py_split(code, &POSIBLE_PINYINS);
        let mut result = Vec::new();

        let read_txn = self.db.begin_read()?;
        let init_prob = read_txn.open_table(INIT_TABLE)?;
        let pinyin_states = read_txn.open_table(PINYIN_STATES)?;
        let trans_prob = read_txn.open_table(TRANS_TABLE)?;
        let emiss_prob = read_txn.open_table(EMISS_TABLE)?;

        for pinyins in possible_pinyins {
            result.push(Self::viterbi(
                &pinyins,
                &pinyin_states,
                &init_prob,
                &trans_prob,
                &emiss_prob,
            ));
        }

        Ok(result
            .into_iter()
            .flatten()
            .map(|(text, weight)| SearchResultItem {
                text,
                weight: weight as u64,

                // workaround
                code: "".to_string(),
                stem: None,
                comment: None,
            })
            .collect_vec())
    }
}
