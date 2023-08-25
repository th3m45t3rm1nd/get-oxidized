use colored::{ColoredString, Colorize};
use std::error::Error;
use std::io;
#[warn(dead_code)]
#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub task: String,
    pub task_status: Status,
}

#[derive(Debug)]
pub enum Status {
    InProgress,
    Completed,
    Todo,
    OnHold,
}

pub fn new_task(task_name: String, task_len: usize) -> Todo {
    Todo {
        id: task_len + 1,
        task: task_name,
        task_status: Status::Todo,
    }
}

pub fn unwrap_status(status: &Status) -> ColoredString {
    match status {
        Status::Todo => "Todo".bright_red().bold(),
        Status::OnHold => "On Hold".blue().italic(),
        Status::InProgress => "In Progress".yellow().bold(),
        Status::Completed => "Completed".green().bold(),
    }
}

pub fn change_status() -> Result<Status, Box<dyn Error>> {
    println!("Change Status to:");
    println!("1. Todo");
    println!("2. On Hold");
    println!("3. In Progress");
    println!("4. Completed");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input == "1" {
        Ok(Status::Todo)
    } else if input == "2" {
        Ok(Status::OnHold)
    } else if input == "3" {
        Ok(Status::InProgress)
    } else if input == "4" {
        Ok(Status::Completed)
    } else {
        Err("Wrong Input.".into())
    }
}
