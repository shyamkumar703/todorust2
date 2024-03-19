use clap::Parser;
use uuid::Uuid;

#[derive(Parser, Debug)]
pub struct TodoCli {
    #[arg(short, long)]
    pub name: String
}

pub struct Todo {
    pub name: String,
    pub id: String
}

impl From<TodoCli> for Todo {
    fn from(todo_cli: TodoCli) -> Self {
        Self { name: todo_cli.name, id: Uuid::new_v4().to_string() }
    }
}
