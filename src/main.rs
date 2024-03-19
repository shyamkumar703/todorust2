mod cli;
mod db;

use clap::Parser;
use cli::TodoCli;
use cli::Todo;
use db::{insert, get_connection};

fn main() {
    let todo: Todo = TodoCli::parse().into();
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(error) => {
            println!("ERROR: {:?}", error);
            return;
        }
    };
    match insert(&conn, &todo) {
        Ok(_) => println!("Added '{}' to list", &todo.name),
        Err(error) => println!("ERROR: {:?}", error),
    }
}
