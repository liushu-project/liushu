use std::{fs::File, path::Path};

use fst::{IntoStreamer, Map, MapBuilder, Streamer};
use memmap2::{Mmap, MmapOptions};
use patricia_tree::PatriciaMap;
use serde::{Deserialize, Serialize};

use crate::error::LiushuError;

pub type Dictionary = PatriciaMap<Vec<DictItem>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct DictItem {
    pub text: String,
    pub code: String,
    pub weight: u32,
    pub comment: Option<String>,
}

pub fn build<I, O>(inputs: &Vec<I>, output: O) -> Result<(), LiushuError>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let mut trie = PatriciaMap::new();
    for dict_path in inputs {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .comment(Some(b'#'))
            .from_path(dict_path)?;
        for result in rdr.deserialize() {
            let item: DictItem = result?;
            let code = item.code.clone();

            if trie.get(&code).is_none() {
                trie.insert_str(code.as_str(), vec![item]);
            } else if let Some(entry) = trie.get_mut(code.as_str()) {
                entry.push(item);
            }
        }
    }

    let trie_writer = File::create(output)?;
    bincode::serialize_into(trie_writer, &trie)?;

    Ok(())
}

pub struct Dictionary2 {
    map: Map<Mmap>,
}

impl Dictionary2 {
    pub fn new(bin_path: impl AsRef<Path>) -> Result<Self, LiushuError> {
        let file = File::open(bin_path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let map = Map::new(mmap).unwrap();
        Ok(Self { map })
    }

    pub fn query(self) {
        let mut stream = self.map.range().ge("clar").into_stream();
        let kvs = stream.into_str_vec().unwrap();
    }
}
