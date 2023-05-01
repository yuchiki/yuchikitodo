#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

use clap::{Parser, Subcommand};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum MainError {
    #[error("file IO failed")]
    FileError(#[from] std::io::Error),
    #[error("wrong usage: help サブコマンドを実行してください。")]
    UsageError,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    subcommand: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Log { todo: String },
    List,
    Delete { index: usize },
}

const TODO_FILE_NAME: &str = "todos.txt";

fn main() -> Result<(), MainError> {
    let cli = Cli::parse();

    match cli.subcommand {
        Command::Log { todo } => {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(TODO_FILE_NAME)?;
            file.write_all(format!("{}\n", todo).as_bytes())?;
        }
        Command::List => {
            let todos = fs::read_to_string(TODO_FILE_NAME)?;
            let todos = todos.trim_end().split("\n");
            for (i, todo) in todos.into_iter().enumerate() {
                println!("{}: {}", i + 1, todo);
            }
        }
        Command::Delete { index } => {
            let todos = fs::read_to_string(TODO_FILE_NAME)?;
            let todos = todos.trim_end().split("\n");
            let todo = todos
                .clone()
                .into_iter()
                .nth(index - 1)
                .ok_or(MainError::UsageError)?;

            let updated_todos: Vec<&str> = todos
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| i != index - 1)
                .map(|(_, todo)| todo)
                .collect();

            println!("{}: {} is deleted.", index, todo);

            File::create(TODO_FILE_NAME)?.write_all(updated_todos.join("\n").as_bytes())?;
        }
    }

    Ok(())
}
