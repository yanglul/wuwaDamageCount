use rusqlite::{Connection, Result};
use std::path::Path;

const db_file:&str= "app.db";
#[derive(Debug)]
struct ModuleList {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}


pub fn is_exist()->bool{
    let file_p = Path::new(db_file);
    if file_p.exists(){
        return true;
    }
    false
}

pub fn create_table()->Result<()>{
    let conn  = Connection::open(db_file)?;
    let _ = conn.execute("CREATE TABLE person (
        id    INTEGER PRIMARY KEY,
        name  TEXT NOT NULL,
        url  TEXT NOT NULL
    )",());

    Ok(())
}




