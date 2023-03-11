use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiushuError {
    #[error("{0}")]
    Other(String),
}

impl From<rusqlite::Error> for LiushuError {
    fn from(value: rusqlite::Error) -> Self {
        LiushuError::Other(format!("sqlite error: {}", value))
    }
}

impl From<csv::Error> for LiushuError {
    fn from(value: csv::Error) -> Self {
        LiushuError::Other(format!("csv error: {}", value))
    }
}

impl From<redb::Error> for LiushuError {
    fn from(value: redb::Error) -> Self {
        LiushuError::Other(format!("redb error: {}", value))
    }
}
