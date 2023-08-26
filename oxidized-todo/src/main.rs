use std::collections::BTreeMap;
use std::io;

use oxidized_todo::{change_status, new_task, unwrap_status, Todo};
use prettytable::{row, Table};

fn main() {
    let mut tasks: BTreeMap<usize, Todo> = BTreeMap::new();
    let mut exit: bool = false;
    while !exit {
        println!("What do you want to do?");
        println!("Type \"help\" if you don't know anything.");
        let mut input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();

        if input == "help" {
            println!("add: to add new task");
            println!("change: to change of a particular task");
            println!("remove: to remove a particular task");
            println!("show: to show all tasks");
            println!("exit: to exit");
        } else if input == "add" {
            let mut task: String = String::from("");
            println!("Enter the name of the task:");
            io::stdin()
                .read_line(&mut task)
                .expect("Failed to read line");
            task = task.trim().to_string();
            tasks.insert(
                tasks.len() + 1,
                new_task(task, tasks.len().try_into().unwrap()),
            );
        } else if input == "show" {
            let mut table = Table::new();
            for (k, v) in tasks.iter() {
                table.add_row(row![k, v.task, unwrap_status(&v.task_status)]);
            }
            table.printstd();
        } else if input == "exit" {
            exit = true
        } else if input == "change" {
            println!("Which task status do you want to change?");
            for (k, v) in tasks.iter() {
                println!("{}: {}", k, v.task);
            }
            let mut input: String = String::from("");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input = input.trim().to_string();

            let mut choice: usize = 0;
            if let Ok(value) = input.parse::<usize>() {
                choice = value;
            }

            if let Some(task) = tasks.get_mut(&choice) {
                if let Ok(status) = change_status() {
                    task.task_status = status;
                    println!("Task changed Successfully.")
                }
            } else {
                println!("Invalid Choice.");
            }
        }
    }
}
