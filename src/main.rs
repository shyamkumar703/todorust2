mod cli;
mod db;

use clap::Parser;
use cli::TodoCli;
use cli::Todo;

fn main() {
    let args: Todo = TodoCli::parse().into();
    println!("{:?}", args.name);
    
}
