use rusqlite::{Connection,Result, params};

#[derive(Debug)]
pub struct Account{
    pub id:i32,
    pub name:String,
    pub balance:f32,
    pub currency:String,
}
/// add a new account
/// 
pub fn add_account(conn:&Connection,name:&str,currency:&str)->Result<()>{
    conn.execute(
        "INSERT INTO ACCOUNT (name,currency,balance) VALUES (?1,?2,?3)",
        (name,currency,0.0),
    )?;
    Ok(())
}

pub fn get_last_id(conn:&Connection)->Result<i32>{
    let mut stmt=conn.prepare("select max(ID) from ACCOUNT;")?;
    let mut iter = stmt.query([])?;

    Ok(iter.next()?.unwrap().get(0)?)
}

pub fn read_account(conn:&Connection,id:i32)->Result<Vec<Account>>{
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

pub fn recalc_account(conn:&Connection,id:i32){
    todo!()
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
        
        read_account(&conn, get_last_id(&conn)?);

        Ok(())
    }
    #[test]
    fn test_get_last_id() -> Result<()>{
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b")?;
        
        assert_eq!(1,get_last_id(&conn)?);
        Ok(())

    }
}