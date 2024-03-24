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
            name text not null,
            is_complete integer not_null
            )",
            () 
            )?;

    Ok(DBSuccess)
}


pub fn insert(conn: &Connection, todo: &Todo) -> Result<DBSuccess, InsertError> {
    let result = conn.execute("INSERT INTO todo (id, name, is_complete) VALUES (?1, ?2, ?3)", (&todo.id, &todo.name, todo.get_is_complete_int()));
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
    let raw_statement = format!("SELECT * FROM todo WHERE id like '%{}%'", id);
    let mut statement = conn.prepare(raw_statement.as_str())?;
    let mut todo_iter = statement.query_map([], |row| {
        Ok(
            Todo::new(row.get(1)?, row.get(2)?, row.get(0)?)
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

pub fn get_all(conn: &Connection) -> Result<Vec<Todo>, GetError> {
    let raw_statement = format!("SELECT * FROM todo");
    let mut statement = conn.prepare(raw_statement.as_str())?;
    let todo_iter = statement.query_map([], |row| {
        Ok(
            Todo::new(row.get(1)?, row.get(2)?, row.get(0)?)
          )
    })?;

    let mut todo_vec: Vec<Todo> = vec![];
    for todo_result in todo_iter {
        if let Ok(todo) = todo_result {
            todo_vec.push(todo);
        }
    }

    Ok(todo_vec)
}

pub fn mark_complete(conn: &Connection, id: &str) -> Result<DBSuccess, Error> {
    let raw_statement = format!("UPDATE todo SET is_complete = 1 WHERE id like '%{}%'", id);
    let _ = conn.execute(&raw_statement, ())?;
    Ok(DBSuccess)
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
            is_complete: false,
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
            is_complete: false,
            id: id.clone(),
        };
        insert(&conn, &todo).unwrap();
        let todo = get(&conn, id.as_str()).unwrap();
        drop_table(&conn);
        assert_eq!(todo.id, id);
        assert_eq!(todo.name, "test todo".to_owned());
        assert_eq!(todo.is_complete, false);
    }

    #[test]
    fn test_mark_todo_as_complete() {
        let conn = get_connection().unwrap();
        create_table(&conn).unwrap();
        let id = Uuid::new_v4().to_string();
        let todo = Todo {
            name: "test todo".into(),
            is_complete: false,
            id: id.clone(),
        };
        insert(&conn, &todo).unwrap();
        mark_complete(&conn, &id).unwrap();
        let todo = get(&conn, &id).unwrap();
        assert_eq!(todo.is_complete, true);
        drop_table(&conn);
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

    #[test]
    fn test_get_todo_with_id_substring() {
        let conn = get_connection().unwrap();
        create_table(&conn).unwrap();
        let id = Uuid::new_v4().to_string();
        let todo = Todo {
            name: "test todo".into(),
            is_complete: false,
            id: id.clone(),
        };
        insert(&conn, &todo).unwrap();
        let id_substring = id.clone().split('-').next().unwrap().to_owned();
        let todo = get(&conn, &id_substring).unwrap();
        drop_table(&conn);
        assert_eq!(todo.id, id);
        assert_eq!(todo.name, "test todo".to_owned());
        assert_eq!(todo.is_complete, false);
    }

    fn drop_table(conn: &Connection) {
        conn.execute("DROP TABLE todo", ()).unwrap();
    }
}
