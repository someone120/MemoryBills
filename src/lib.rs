use database::{
    account::{self, Account},
    details,
};
use error::Error;
use rusqlite::Connection;
mod database;
mod error;

pub fn recalcute(conn: &Connection) -> Result<(), Error> {
    let accounts = account::get_accounts(conn)?;
    for account in accounts {
        let details = details::get_details_by_account(conn, account.id.as_str())?;
        let mut balance = 0.0;
        for i in details {
            balance += i.balance;
        }
        let mut acc = account;
        acc.balance = balance;
        account::update_account(conn, &acc)?;
    }
    Ok(())
}

pub fn verification(conn: &Connection) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::database::{init, transaction};

    use super::*;

    #[test]
    fn test_recalcute() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init::init(&conn)?;
        let bar_id = account::add_account(&conn, "foo", "USD")?;
        let foo_id = account::add_account(&conn, "bar", "USD")?;

        let date = chrono::Utc::now();
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(&conn, id.as_str(), bar_id.as_str(), "USD", -10.0)?;
        details::add_details(&conn, id.as_str(), foo_id.as_str(), "USD", 10.0)?;
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(&conn, id.as_str(), bar_id.as_str(), "USD", -12.0)?;
        details::add_details(&conn, id.as_str(), foo_id.as_str(), "USD", 12.0)?;
        recalcute(&conn)?;

        let accounts = account::get_accounts(&conn)?;

        for account in accounts {
            if account.id == foo_id {
                assert_eq!(22.0, account.balance);
            }
            if account.id == bar_id {
                assert_eq!(-22.0, account.balance);
            }
        }
        Ok(())
    }
}
