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
