use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    Done,
}

impl TaskStatus {
    pub fn marker(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "[ ]",
            TaskStatus::Done => "[x]",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
    pub done_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            status: TaskStatus::Todo,
            created_at: Local::now(),
            done_at: None,
        }
    }

    pub fn mark_done(&mut self) {
        self.status = TaskStatus::Done;
        self.done_at = Some(Local::now());
    }

    pub fn is_done(&self) -> bool {
        self.status == TaskStatus::Done
    }
}

#[derive(Debug, Default)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    next_id: u32,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn with_tasks(tasks: Vec<Task>) -> Self {
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Self { tasks, next_id }
    }

    pub fn add(&mut self, title: String) -> &Task {
        let task = Task::new(self.next_id, title);
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn remove(&mut self, id: u32) -> Option<Task> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(pos))
        } else {
            None
        }
    }

    pub fn todo_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter().filter(|t| !t.is_done())
    }

    pub fn all_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }
}
