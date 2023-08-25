use chrono::{Date, DateTime, NaiveDate};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub extra: String,
}

pub fn add_transaction(conn: &Connection, date: DateTime<chrono::Utc>, extra: &str) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO TRANS (id,time,extra) VALUES (?1,?2,?3)",
        (
            id.as_str(),
            date.format("%Y-%m-%d %H:%M").to_string().as_str(),
            extra,
        ),
    )?;
    Ok(id)
}

pub fn del_transaction(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM TRANS WHERE id=?1", params![id])?;
    Ok(())
}

pub fn get_transactions(conn: &Connection) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare("SELECT * FROM TRANS")?;
    let iter = stmt.query_map([], |row| {
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

    use crate::database::init;

    use super::*;
    #[test]
    fn test_add_transaction() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        let date = chrono::Utc::now();
        init::init(&conn)?;
        add_transaction(&conn, date, "1")?;
        let trans = get_transactions(&conn)?;

        assert_eq!(1,trans.len());
        Ok(())
    }

    #[test]
    fn test_del_transaction() -> Result<()>{
        let conn = Connection::open_in_memory()?;
        let date = chrono::Utc::now();
        init::init(&conn)?;
        add_transaction(&conn, date, "1")?;
        add_transaction(&conn, date, "2")?;

        let trans = get_transactions(&conn)?;

        assert_eq!(2,trans.len());

        del_transaction(&conn, &trans[0].id)?;

        let trans = get_transactions(&conn)?;
        assert_eq!(1,trans.len());
        Ok(())
    }

    #[test]
    fn test_change_extra()->Result<()>{
        let conn = Connection::open_in_memory()?;
        let date = chrono::Utc::now();
        init::init(&conn)?;
        add_transaction(&conn, date, "unchange")?;
        let trans = get_transactions(&conn)?;

        assert_eq!(1,trans.len());

        change_extra(&conn, &trans[0].id, "changed")?;

        let trans = get_transactions(&conn)?;
        assert_eq!("changed",trans[0].extra);
        Ok(())
    }
}
