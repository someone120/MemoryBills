use std::{error::Error, fmt};

#[derive(Debug)]
pub struct DetailSumNotZero {
    pub trans_id: String,
    pub diff: f32,
}

impl fmt::Display for DetailSumNotZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Details sum not zero. trans_id:{}, diff:{}",
            self.trans_id,
            self.diff
        )
    }
}
