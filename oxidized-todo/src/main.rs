use std::collections::HashMap;
use std::io;

use oxidized_todo::{new_task, Todo};
use prettytable::{row, Table};

fn main() {
    let mut tasks: HashMap<usize, Todo> = HashMap::new();
    let mut task: String = String::from("");
    println!("Enter the name of the task:");
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");
    tasks.insert(
        tasks.len() + 1,
        new_task(task, tasks.len().try_into().unwrap()),
    );

    let mut table = Table::new();
    for (k, v) in tasks.iter() {
        table.add_row(row![k, v.task]);
    }

    table.printstd();
}
