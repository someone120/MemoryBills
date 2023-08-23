use rusqlite::{Connection,Result, params};
use uuid::Uuid;

#[derive(Debug)]
pub struct Account{
    pub id:String,
    pub name:String,
    pub balance:f32,
    pub currency:String,
}
/// add a new account
pub fn add_account(conn:&Connection,name:&str,currency:&str)->Result<()>{
    let id=Uuid::new_v4().to_string();
    add_account_with_id(conn, name, currency,id.as_str())
}

fn add_account_with_id(conn:&Connection,name:&str,currency:&str,id:&str)->Result<()>{

    conn.execute(
        "INSERT INTO ACCOUNT (id,name,currency,balance) VALUES (?1,?2,?3,?4)",
        (id,name,currency,0.0),
    )?;
    Ok(())
}

/// Get info of account
pub fn read_account(conn:&Connection,id:&str)->Result<Vec<Account>>{
    let mut stmt=conn.prepare("SELECT * FROM ACCOUNT WHERE id = ?")?;
    let iter = stmt.query_map(params![id], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            currency: row.get(2)?,
            balance: row.get(3)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter{
        result.push(i?)
    }
    Ok(result)
}

/// Remove account
pub fn del_account(conn:&Connection,id:&str)-> Result<()>{
    conn.execute("DELETE FROM ACCOUNT WHERE id=?1", params![id])?;
    Ok(())
}

/// Get all account
pub fn get_accounts(conn:&Connection)->Result<Vec<Account>>{
    let mut stmt=conn.prepare("SELECT * FROM ACCOUNT")?;
    let iter = stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            currency: row.get(2)?,
            balance: row.get(3)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter{
        result.push(i?)
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::database::init::init;

    use super::*;
    #[test]
    fn test_add_account() -> Result<()>{
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "someone", "CNY")?;
        
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
    fn test_read_account() -> Result<()>{
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b")?;
        let accounts=get_accounts(&conn)?;
        let a=read_account(&conn,&accounts[0].id)?;
        println!("{:?}",a);

        Ok(())
    }
    #[test]
    fn test_get_last_id() -> Result<()>{
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b")?;
        
        let accounts=get_accounts(&conn)?;

        assert_eq!(1,accounts.len());
        Ok(())
    }
    #[test]
    fn test_delete()->Result<()>{

        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b")?;
        add_account(&conn, "a", "b")?;
        let id=get_accounts(&conn)?;
        del_account(&conn, &id[0].id)?;
        
        assert_eq!(1,get_accounts(&conn)?.len());
        Ok(())
    }
}
