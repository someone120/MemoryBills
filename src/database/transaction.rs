use std::time::TryFromFloatSecsError;

use chrono::{Date, NaiveDate};
use rusqlite::Connection;

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub extra: String,
}

pub fn add_transaction(conn: &Connection, date: NaiveDate, extra: &str) {
    todo!()
}

pub fn del_transaction(conn: &Connection, id: &str) {
    todo!()
}

pub fn get_transactions(conn: &Connection, id: &str) {
    todo!()
}

pub fn change_extra(conn: &Connection, id: &str, extra: &str) {
    todo!()
}
