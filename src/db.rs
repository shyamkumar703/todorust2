use rusqlite::{Connection, Result, Error};
use crate::cli::Todo;

#[derive(Debug)]
pub enum InsertError {
    OpenConnectionError(Error),
    InsertError(Error),
    CreationError(Error),
}

pub struct DBSuccess;

pub fn get_connection() -> Result<Connection, Error> {
   Connection::open("test.db")
}

pub fn create_table(conn: &Connection) -> Result<DBSuccess, Error> {
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id text primary key, 
            name text not null
        )",
       () 
    )?;

    Ok(DBSuccess)
}


pub fn insert(conn: &Connection, todo: &Todo) -> Result<DBSuccess, InsertError> {
    match create_table(conn) {
        Err(error) => return Err(InsertError::CreationError(error)),
        Ok(_) => (),
    };
    let result = conn.execute("INSERT INTO todo (id, name) VALUES (?1, ?2)", (&todo.id, &todo.name));
    match result {
        Ok(_) => return Ok(DBSuccess),
        Err(error) => return Err(InsertError::InsertError(error))
    }
}

#[derive(Debug)]
pub enum GetError {
    DBError(Error),
    NoResults,
}

impl From<Error> for GetError {
    fn from(value: Error) -> Self {
        Self::DBError(value)
    }
}

pub fn get(conn: &Connection, id: &str) -> Result<Todo, GetError> {
    let raw_statement = format!("SELECT * FROM todo WHERE id='{}'", id);
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

#[cfg(test)]
mod tests {
    use crate::cli::Todo;
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_create_table() {
        let conn = get_connection().unwrap();
        create_table(&conn).unwrap();
        drop_table(&conn);
    }

    #[test]
    fn test_insert_todo() {
        let conn = get_connection().unwrap();
        create_table(&conn).unwrap();
        let id = Uuid::new_v4().to_string();
        let todo = Todo {
            name: "test todo".into(),
            id: id.into(),
        };
        insert(&conn, &todo).unwrap();
        drop_table(&conn);
    }

    #[test]
    fn test_get_todo() {
        let conn = get_connection().unwrap();
        create_table(&conn).unwrap();
        let id = Uuid::new_v4().to_string();
        let todo = Todo {
            name: "test todo".into(),
            id: id.clone(),
        };
        insert(&conn, &todo).unwrap();
        let todo = get(&conn, id.as_str()).unwrap();
        drop_table(&conn);
        assert_eq!(todo.id, id);
        assert_eq!(todo.name, "test todo".to_owned());
    }

    #[test]
    fn test_get_todo_not_existing() {
        let conn = get_connection().unwrap();
        create_table(&conn).unwrap();
        let id = Uuid::new_v4().to_string();
        match get(&conn, id.as_str()) {
            Ok(_) => {
                drop_table(&conn);
                assert!(false, "getting todo that doesn't exist should error")
            },
            Err(_) => {
                drop_table(&conn);
                assert!(true)
            },
        }
    }

    fn drop_table(conn: &Connection) {
        conn.execute("DROP TABLE todo", ()).unwrap();
    }
}
