use rusqlite::{Connection,Result};

#[derive(Debug)]
pub struct Details{
    pub id:i32,
    pub trans_id:i32,
    pub account_id:i32,
    pub currency:String,
    pub balance:f32,
}

pub fn add_details(conn: &Connection,trans_id:i32,account_id:i32,currency:String,balance:f32){
    todo!()
}

pub fn get_details_by_trans(conn: &Connection,trans_id:i32) -> Result<Vec<Details>>{
    todo!()
}

pub fn get_details_by_account(conn: &Connection,account_id:i32) -> Result<Vec<Details>>{
    todo!()
}