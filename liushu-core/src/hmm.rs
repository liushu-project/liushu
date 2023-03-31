mod pinyin;

use std::f64::consts::E;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;
use patricia_tree::PatriciaMap;
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

pub fn train_to_db(
    corpus_file: impl AsRef<Path>,
    db: &Database,
    trie: &mut PatriciaMap<Vec<String>>,
) -> Result<(), LiushuError> {
    count_init_prob(corpus_file.as_ref(), db)?;
    count_trans_prob(corpus_file.as_ref(), db)?;
    count_emiss_prob(corpus_file.as_ref(), db)?;
    save_trie(db, trie)?;

    Ok(())
}

fn count_init_prob(corpus_file: impl AsRef<Path>, db: &Database) -> Result<(), LiushuError> {
    let mut initial_counts = HashMap::new();
    let mut total_count = 0;

    let file = File::open(corpus_file)?;
    let reader = BufReader::new(file);

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        let tokens: Vec<&str> = line.split('\t').collect();

        let sentence = tokens[0]
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let first_char = sentence[0].to_string();
        *initial_counts.entry(first_char).or_insert(0) += 1;
        total_count += 1;

        if idx % 5000 == 0 {
            println!("current init count {}", idx);
        }
    }

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(INIT_TABLE)?;
        for (key, value) in initial_counts {
            let value = (value as f64 / total_count as f64).log(E);
            table.insert(&key.as_str(), value)?;
        }
    }
    write_txn.commit()?;

    Ok(())
}

fn count_trans_prob(corpus_file: impl AsRef<Path>, db: &Database) -> Result<(), LiushuError> {
    let mut trans_map: HashMap<String, HashMap<String, u64>> = HashMap::new();

    let file = File::open(corpus_file)?;
    let reader = BufReader::new(file);

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        let tokens: Vec<&str> = line.split('\t').collect();

        let sentence = tokens[0]
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for (word1, word2) in sentence.iter().zip(sentence.iter().skip(1)) {
            let trans_prop = trans_map
                .entry(word1.to_string())
                .or_insert_with(HashMap::new);
            let next_prob = trans_prop.entry(word2.to_string()).or_insert(0);
            *next_prob += 1;

            if idx % 5000 == 0 {
                println!("current trans count {}", idx);
            }
        }
    }

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TRANS_TABLE)?;
        for (post, value) in trans_map {
            let total = value.values().sum::<u64>();
            for (pre, count) in value {
                let prob = (count as f64 / total as f64).log(E);
                table.insert((post.as_str(), pre.as_str()), prob)?;
            }
        }
    }
    write_txn.commit()?;

    Ok(())
}

fn count_emiss_prob(corpus_file: impl AsRef<Path>, db: &Database) -> Result<(), LiushuError> {
    let mut emit_map: HashMap<String, HashMap<String, u64>> = HashMap::new();

    let file = File::open(corpus_file)?;
    let reader = BufReader::new(file);

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        let tokens: Vec<&str> = line.split('\t').collect();

        let sentence = tokens[0]
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let pinyin = tokens[1]
            .split(',')
            .map(|p| p.to_string())
            .collect::<Vec<String>>();
        for (word, py) in sentence.iter().zip(pinyin) {
            let emit_prop = emit_map
                .entry(word.to_string())
                .or_insert_with(HashMap::new);
            let py_prob = emit_prop.entry(py).or_insert(0);
            *py_prob += 1;
        }

        if idx % 5000 == 0 {
            println!("current emiss count {}", idx);
        }
    }

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(EMISS_TABLE)?;
        for (word, pinyins) in emit_map {
            let total = pinyins.values().sum::<u64>();
            for (py, count) in pinyins {
                let prob = (count as f64 / total as f64).log(E);
                table.insert((word.as_str(), py.as_str()), prob)?;
            }
        }
    }
    write_txn.commit()?;

    Ok(())
}

fn save_trie(db: &Database, trie: &mut PatriciaMap<Vec<String>>) -> Result<(), LiushuError> {
    let mut idx = 0;
    let read_txn = db.begin_read().unwrap();
    {
        let emission_table = read_txn.open_table(EMISS_TABLE).unwrap();
        for (key, _) in emission_table.iter()? {
            let (word, py) = key.value().to_owned();

            let py: String = py.split_whitespace().collect();

            if let Some(entry) = trie.get_mut(&py) {
                entry.push(word.to_string());
            } else {
                trie.insert_str(&py, vec![word.to_string()]);
            }

            if idx % 5000 == 0 {
                println!("current init count {}", idx);
            }

            idx += 1;
        }
    }

    Ok(())
}

pub fn pinyin_to_sentence(
    py_sequence: &Vec<String>,
    db: &Database,
    trie: &PatriciaMap<Vec<String>>,
) -> Result<String, LiushuError> {
    let read_txn = db.begin_read()?;
    let init_table = read_txn.open_table(INIT_TABLE)?;
    let trans_table = read_txn.open_table(TRANS_TABLE)?;
    let emiss_table = read_txn.open_table(EMISS_TABLE)?;

    let mut scores = vec![HashMap::new(); py_sequence.len()];
    let mut back_pointers = vec![HashMap::new(); py_sequence.len()];

    // Initialize the first score vector using the initial probabilities
    let first_py = py_sequence[0].clone();
    let mut states = Vec::new();
    if let Some(word) = trie.get(&first_py) {
        states.extend(word);
    }
    for word in states {
        let log_init_prob = init_table
            .get(word.as_str())?
            .map(|x| x.value())
            .unwrap_or(MIN_F);
        let log_emiss_prob = emiss_table
            .get(&(word.as_str(), first_py.as_str()))?
            .map(|x| x.value())
            .unwrap_or(MIN_F);
        let score = log_init_prob + log_emiss_prob;
        scores[0].insert(word.to_string(), score);
    }

    // Iterate over the remaining pinyin tokens, computing the score for each possible hanzi
    for (i, py) in py_sequence.iter().skip(1).enumerate() {
        let i = i + 1;
        let words = trie.get(py).map(|x| x.to_owned()).unwrap_or(vec![]);
        for word in words {
            let mut max_score = f64::NEG_INFINITY;
            let mut max_word = String::new();

            // Compute the score for each possible previous hanzi and choose the maximum
            for (prev_word, prev_score) in &scores[i - 1] {
                let log_trans_prob = trans_table
                    .get(&(word.as_str(), prev_word.as_str()))?
                    .map(|x| x.value())
                    .unwrap_or(MIN_F);
                let score = prev_score + log_trans_prob;
                if score > max_score {
                    max_score = score;
                    max_word = prev_word.to_string();
                }
            }

            // Compute the emission probability for the current hanzi and store the max score and backpointer
            let log_emiss_prob = emiss_table
                .get(&(word.as_str(), py.as_str()))?
                .map(|x| x.value())
                .unwrap_or(MIN_F);
            let score = max_score + log_emiss_prob;
            scores[i].insert(word.to_string(), score);
            back_pointers[i].insert(word.to_string(), max_word);
        }
    }

    // Determine the most probable hanzi sequence by following the back pointers
    let mut max_final_score = f64::NEG_INFINITY;
    let mut max_final_word = String::new();
    for (word, score) in &scores[py_sequence.len() - 1] {
        if *score > max_final_score {
            max_final_score = *score;
            max_final_word = word.to_string();
        }
    }

    let mut result = max_final_word.clone();
    for i in (0..py_sequence.len() - 1).rev() {
        if let Some(prev_word) = back_pointers[i + 1].get(&max_final_word) {
            result.insert_str(0, prev_word);
            max_final_word = prev_word.clone();
        }
    }

    Ok(result)
}

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
                comment: None,
            })
            .collect_vec())
    }
}
