use rusqlite::Connection;
use error::Error;

mod database;
mod error;

pub fn recalcute(conn:&Connection)->Result<(),Error>{
    database::account::get_accounts(conn)?;
    todo!()
}

pub fn verification(conn:&Connection){
    todo!()
}

