use redb::TableDefinition;
use serde::Deserialize;

pub const DICTIONARY: TableDefinition<&str, (u64, Option<&str>)> =
    TableDefinition::new("dictionary");

#[derive(Debug, Deserialize)]
pub struct DictItem {
    pub text: String,
    pub code: String,
    pub weight: u64,
    pub comment: Option<String>,
}
