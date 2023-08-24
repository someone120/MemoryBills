use std::time::TryFromFloatSecsError;

use chrono::{Date, NaiveDate};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub extra: String,
}

pub fn add_transaction(conn: &Connection, date: NaiveDate, extra: &str) -> Result<()> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO TRANS (id,time,extra) VALUES (?1,?2,?3)",
        (id.as_str(), date.format("%Y-%m-%d %H:%").to_string(), extra),
    )?;
    Ok(())
}

pub fn del_transaction(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM TRANS WHERE id=?1", params![id])?;
    Ok(())
}

pub fn get_transactions(conn: &Connection, id: &str) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare("SELECT * FROM TRANS WHERE id=?1")?;
    let iter = stmt.query_map(params![id], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            date: row.get(1)?,
            extra: row.get(2)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}

pub fn change_extra(conn: &Connection, id: &str, extra: &str) -> Result<()> {
    conn.execute(
        "UPDATE TRANS SET extra = ?1 WHERE id = ?2;",
        params![extra, id],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_transaction() {
        todo!()
    }

    #[test]
    fn test_del_transaction() {}
}
