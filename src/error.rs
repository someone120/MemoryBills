use std::fmt;

#[derive(Debug)]
pub enum Error {
    DetailSumNotZero(String, f32),
    RusqliteError(rusqlite::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DetailSumNotZero(trans_id, diff) => write!(
                f,
                "Transaction sum not zero: 
transaction id: {},
diff: {}",
                trans_id, diff
            ),
            Error::RusqliteError(error) => write!(f,"RusqliteError:{}",error.to_string())
        }
    }
}

impl std::error::Error for Error {}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Error::RusqliteError(value)
    }
}
