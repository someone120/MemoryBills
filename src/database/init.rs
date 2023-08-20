use rusqlite::{self, Connection, Result};

pub fn init(conn: &Connection) -> Result<()> {
    init_account(conn)?;
    init_transaction(conn)?;
    init_details(conn)?;
    Ok(())
}

fn init_account(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE ACCOUNT(
        id INTEGER PRIMARY KEY autoincrement,
        name VARCHAR(255) NOT NULL,
        currency VARCHAR(255) NOT NULL,
        balance float NOT NULL
    )
    ",
        (),
    )?;
    Ok(())
}

fn init_details(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE DETAIL(
        id INTEGER PRIMARY KEY autoincrement,
        trans_id INTEGER NOT NULL,
        account_id INTEGER NOT NULL,
        currency VARCHAR(255) NOT NULL,
        balance float NOT NULL
    )
    ",
        (),
    )?;
    Ok(())
}

fn init_transaction(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE TRANS(
        id INTEGER PRIMARY KEY autoincrement,
        extra VARCHAR(255)
    )
    ",
        (),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Result};

    use crate::database::{
        account::{self, Account},
        details::Details, transaction::Transaction,
    };

    use super::*;
    #[test]
    fn test_account() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_account(&conn)?;
        conn.execute(
            "INSERT INTO ACCOUNT (id,name,currency,balance) VALUES (?1,?2,?3,?4)",
            (0, "a", "b", 0.1),
        )?;
        let mut stmt = conn.prepare("SELECT id,name,currency,balance FROM ACCOUNT")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Account {
                id: row.get(0)?,
                name: row.get(1)?,
                currency: row.get(2)?,
                balance: row.get(3)?,
            })
        })?;
        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }

    #[test]
    fn test_details() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_details(&conn)?;
        conn.execute(
            "INSERT INTO DETAIL (id,trans_id,account_id,currency,balance) VALUES (?1,?2,?3,?4,?5)",
            (0, 1, 2, "CNY", 0.1),
        )?;
        let mut stmt =
            conn.prepare("SELECT id,trans_id,account_id,currency,balance FROM DETAIL")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Details {
                id: row.get(0)?,
                trans_id: row.get(1)?,
                account_id: row.get(2)?,
                currency: row.get(3)?,
                balance: row.get(4)?,
            })
        })?;

        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }

    #[test]
    fn test_trans() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_transaction(&conn)?;
        conn.execute(
            "INSERT INTO TRANS (id,extra) VALUES (?1,?2)",
            (0,"CNY"),
        )?;
        let mut stmt =
            conn.prepare("SELECT id,extra FROM TRANS")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                extra: row.get(1)?,
            })
        })?;

        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }
}
