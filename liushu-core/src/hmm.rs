use std::collections::HashMap;
use std::f64::consts::E;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use itertools::{iproduct, Itertools};
use pinyin::ToPinyin;
use redb::{Database, ReadableTable, TableDefinition};
use regex::Regex;

const INIT_TABLE: TableDefinition<&str, f64> = TableDefinition::new("init_prob");
const TRANS_TABLE: TableDefinition<(&str, &str), f64> = TableDefinition::new("trans_prob");
const EMISS_TABLE: TableDefinition<(&str, &str), f64> = TableDefinition::new("emiss_prob");
const PINYIN_STATES: TableDefinition<&str, &str> = TableDefinition::new("pinyin_states");

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

pub struct Hmm {
    py_list: Vec<String>,
    db: Database,
}

impl Hmm {
    pub fn new(db: Database) -> Self {
        let sm_list = "b,p,m,f,d,t,n,l,g,k,h,j,q,x,z,c,s,r,zh,ch,sh,y,w".split(',');
        let ym_list = "a,o,e,i,u,v,ai,ei,ui,ao,ou,iu,ie,ve,er,an,en,in,un,ang,eng,ing,ong,uai,ia,uan,uang,uo,ua".split(',');
        let ztrd_list = "a,o,e,ai,ei,ao,ou,er,an,en,ang,zi,ci,si,zhi,chi,shi,ri,yi,wu,yu,yin,ying,yun,ye,yue,yuan".split(',');
        let mut py_list = Vec::new();
        for (s, y) in iproduct!(sm_list, ym_list) {
            let temp = s.to_string() + y;
            if !py_list.contains(&temp) {
                py_list.push(temp);
            }
        }

        for z in ztrd_list {
            if !py_list.contains(&z.to_string()) {
                py_list.push(z.to_string());
            }
        }

        Self { db, py_list }
    }

    pub fn trans(&self, code: &str) -> Vec<String> {
        let possible_pinyins = pysplict(code, &self.py_list);
        let min_f = -3.14e100;
        let mut result: Vec<(usize, String)> = Vec::new();

        let read_txn = self.db.begin_read().unwrap();
        let init_prob = read_txn.open_table(INIT_TABLE).unwrap();
        let pinyin_states = read_txn.open_table(PINYIN_STATES).unwrap();
        let trans_table = read_txn.open_table(TRANS_TABLE).unwrap();
        let emiss_prob = read_txn.open_table(EMISS_TABLE).unwrap();

        for pinyins in possible_pinyins {
            let length = pinyins.len();
            let mut viterbi: HashMap<usize, HashMap<String, (f64, String)>> = HashMap::new();
            for i in 0..length {
                viterbi.insert(i, HashMap::new());
            }

            let key = pinyins[0].as_str();
            let chars = pinyin_states.get(key).unwrap().unwrap();
            for s in chars.value().chars() {
                let p = viterbi.get_mut(&0).unwrap();
                let init = init_prob
                    .get(s.to_string().as_str())
                    .unwrap()
                    .map(|x| x.value())
                    .unwrap_or(min_f);
                let emiss = emiss_prob
                    .get((s.to_string().as_str(), pinyins[0].as_str()))
                    .unwrap()
                    .map(|x| x.value())
                    .unwrap_or(min_f);
                p.insert(s.to_string(), (init + emiss, "".to_string()));
            }

            for i in 0..(length - 1) {
                let key = pinyins[i + 1].as_str();
                let chars = pinyin_states.get(key).unwrap().unwrap();
                for s in chars.value().chars() {
                    let value = pinyin_states
                        .get(pinyins[i].as_str())
                        .unwrap()
                        .unwrap()
                        .value()
                        .chars()
                        .map(|c| {
                            let vit = viterbi[&i][c.to_string().as_str()].0;
                            let emission = emiss_prob
                                .get((s.to_string().as_str(), pinyins[i + 1].as_str()))
                                .unwrap()
                                .map(|e| e.value())
                                .unwrap_or(min_f);
                            let trans = trans_table
                                .get((s.to_string().as_str(), c.to_string().as_str()))
                                .unwrap()
                                .map(|t| t.value())
                                .unwrap_or(min_f);
                            (vit + emission + trans, c.to_string())
                        })
                        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                        .unwrap();
                    let p = viterbi.get_mut(&(i + 1)).unwrap();
                    p.insert(s.to_string(), value);
                }
            }

            let key = pinyins.last().unwrap().as_str();
            let last = pinyin_states.get(key).unwrap().unwrap();
            for s in last.value().chars() {
                let old = &viterbi[&(length - 1)][s.to_string().as_str()];
                let trans = trans_table
                    .get(("EOS", s.to_string().as_str()))
                    .unwrap()
                    .map(|x| x.value())
                    .unwrap_or(min_f);
                let new_value = (old.0 + trans, old.1.clone());
                let p = viterbi.get_mut(&(length - 1)).unwrap();
                p.insert(s.to_string(), new_value);
            }

            let words_list = viterbi[&(length - 1)]
                .iter()
                .sorted_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap())
                .rev()
                .take(100);

            for (idx, data) in words_list.enumerate() {
                let mut words = vec!["".to_string(); length];
                if let Some(last) = words.last_mut() {
                    *last = data.0.clone();
                }

                for n in (0..(length - 1)).rev() {
                    words[n] = viterbi[&(n + 1)][words[n + 1].to_string().as_str()]
                        .1
                        .clone();
                }

                result.push((idx, words.join("")));
            }
        }

        result.sort_by_key(|x| x.0);
        result.iter().map(|x| x.1.clone()).collect_vec()
    }
}

fn pysplict(word: &str, word_list: &Vec<String>) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    dp(&mut res, word, word_list, "".to_string());
    res.sort_by_key(|x| x.len());
    res
}

fn dp(res: &mut Vec<Vec<String>>, word: &str, word_list: &Vec<String>, pinyin_list_str: String) {
    let len = word.len();
    for i in 0..=len {
        let mut p_list: Vec<String> = pinyin_list_str.split(',').map(|x| x.to_string()).collect();
        let sub_word = word[0..i].to_string();
        if word_list.contains(&sub_word) {
            if i == len {
                p_list.push(sub_word);
                res.push(p_list[1..].iter().map(|x| x.to_string()).collect());
            } else {
                p_list.push(sub_word);
                dp(res, &word[i..], word_list, p_list.join(","));
            }
        }
    }
}
