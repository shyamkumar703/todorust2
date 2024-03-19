use rusqlite::{Connection, Result, Error}; use crate::cli::Todo;
use rusqlite::NO_PARAMS;

pub enum InsertError {
    OpenConnectionError(Error)
}

pub struct DBSuccess;

pub fn create_table() -> Result<DBSuccess, Error> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id text primary key, 
            name text not null
        )",
        NO_PARAMS
    )?;

    Ok(DBSuccess)
}


pub fn insert(todo: &Todo) -> Result<DBSuccess, InsertError> {
    let conn = match Connection::open("test.db") {
        Ok(connection) => connection,
        Err(error) => return Err(InsertError::OpenConnectionError(error))
    };
    conn.execute("INSERT INTO test (name) VALUES (?1)", (&todo.name,));

    Ok(DBSuccess)
}

pub enum GetError {
    DBError(Error),
    NoResults,
}

impl From<Error> for GetError {
    fn from(value: Error) -> Self {
        Self::DBError(value)
    }
}

pub fn get(id: &str) -> Result<Todo, GetError> {
    let conn = match Connection::open("test.db") {
        Ok(connection) => connection,
        Err(error) => return Err(error.into())
    };

    let raw_statement = format!("SELECT * FROM test WHERE id={}", id);
    let mut statement = conn.prepare(raw_statement.as_str())?;
    let mut todo_iter = statement.query_map([], |row| {
        Ok(
            Todo { name: row.get(1)?, id: row.get(0)? }
        )
    })?;

    match todo_iter.next() {
        Some(todo_result) => {
            match todo_result {
                Ok(todo) => return Ok(todo),
                Err(error) => return Err(error.into()),
            }
        } 
        None => return Err(GetError::NoResults)
    }
}

