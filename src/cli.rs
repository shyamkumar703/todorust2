use clap::Parser;
use uuid::Uuid;

#[derive(Parser, Debug)]
pub struct CreateTodo {
    #[arg(short, long)]
    pub name: String
}

#[derive(Parser, Debug)]
pub struct GetTodos {
    #[arg(short, long)]
    pub id: Option<String>
}

#[derive(Debug)]
pub struct Todo {
    pub name: String,
    pub is_complete: bool,
    pub id: String,
}

impl Todo {
    pub fn new(name: String, is_complete: u8, id: String) -> Todo {
        let is_complete = if is_complete == 0 { false } else { true };
        Self { name, is_complete, id }
    }

    pub fn get_is_complete_int(&self) -> u8 {
        if self.is_complete {
            return 1;
        } else {
            return 0;
        } 
    }
}

impl From<CreateTodo> for Todo {
    fn from(todo_cli: CreateTodo) -> Self {
        Self { name: todo_cli.name, is_complete: false,  id: Uuid::new_v4().to_string() }
    }
}
