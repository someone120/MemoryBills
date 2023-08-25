use rusqlite::{params, Connection, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct Details {
    pub id: String,
    pub trans_id: String,
    pub account_id: String,
    pub currency: String,
    pub balance: f32,
}

pub fn add_details(
    conn: &Connection,
    trans_id: &str,
    account_id: &str,
    currency: &str,
    balance: f32,
) -> Result<()> {
    
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO DETAIL (id,trans_id,account_id,currency,balance) values (?1,?2,?3,?4,?5)",
        params![id,trans_id, account_id, currency, balance],
    )?;

    Ok(())
}

pub fn get_details_by_trans(conn: &Connection, trans_id: &str) -> Result<Vec<Details>> {
    let mut stmt = conn.prepare(
        "SELECT id,trans_id,account_id,currency,balance FROM DETAIL WHERE trans_id=?1",
    )?;
    let iter = stmt.query_map(params![trans_id], |row| {
        Ok(Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account_id: row.get(2)?,
            currency: row.get(3)?,
            balance: row.get(4)?,
        })
    })?;

    let mut result = Vec::new();
    for i in iter {
        result.push(i?);
    }

    Ok(result)
}

pub fn get_details_by_account(conn: &Connection, account_id: &str) -> Result<Vec<Details>> {
    let mut stmt = conn.prepare(
        "SELECT id,trans_id,account_id,currency,balance FROM DETAIL WHERE account_id=?1",
    )?;
    let iter = stmt.query_map(params![account_id], |row| {
        Ok(Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account_id: row.get(2)?,
            currency: row.get(3)?,
            balance: row.get(4)?,
        })
    })?;

    let mut result = Vec::new();
    for i in iter {
        result.push(i?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {}
