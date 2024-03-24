mod cli;
mod db;

use clap::Parser;
use cli::{CreateTodo, GetTodos};
use cli::Todo;
use db::{create_table, get, get_connection, insert, get_all};
use rusqlite::Connection;

fn main() {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(error) => {
            println!("ERROR: {:?}", error);
            return;
        }
    };
    
    create_table(&conn).unwrap();

    if let Ok(value) = GetTodos::try_parse() {
        execute_get_todos(&conn, value);
        return;
    }

    if let Ok(todo) = CreateTodo::try_parse() {
        execute_insert_todo(&conn, todo.into());
        return;
    }

    println!("Invalid command.");
    return;
}

fn execute_get_todos(conn: &Connection, value: GetTodos) {
    if let Some(id) = value.id {
        match get(&conn, &id) {
            Ok(todo) => println!("{:?}", todo),
            Err(error) => println!("ERROR: {:?}", error),
        }
    } else {
        println!("Getting all...");
        match get_all(&conn) {
            Ok(todo_vec) => println!("{:?}", todo_vec),
            Err(error) => println!("ERROR: {:?}", error), 
        }
    }
}

fn execute_insert_todo(conn: &Connection, todo: Todo) {
    match insert(&conn, &todo) {
        Ok(_) => println!("Successfully added todo {:?}", todo),
        Err(error) => println!("ERROR: {:?}", error),
    }
}
