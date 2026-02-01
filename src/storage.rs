use crate::error::{GodoError, Result};
use crate::task::{Task, TaskList, TaskStatus};
use chrono::{DateTime, Local};
use regex::Regex;
use std::path::PathBuf;

const HEADER: &str = "# godo tasks\n\n";

pub fn load_tasks(path: &PathBuf) -> Result<TaskList> {
    if !path.exists() {
        return Ok(TaskList::new());
    }

    let content = std::fs::read_to_string(path).map_err(|e| GodoError::FileRead {
        path: path.clone(),
        source: e,
    })?;

    parse_tasks(&content)
}

pub fn save_tasks(path: &PathBuf, tasks: &TaskList) -> Result<()> {
    let content = format_tasks(tasks);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| GodoError::CreateDir {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
    }

    std::fs::write(path, content).map_err(|e| GodoError::FileWrite {
        path: path.clone(),
        source: e,
    })?;

    Ok(())
}

fn parse_tasks(content: &str) -> Result<TaskList> {
    let mut tasks = Vec::new();

    let task_re =
        Regex::new(r"^- \[([ x])\] (.+?) <!-- id:(\d+) created:([^\s]+)(?: done:([^\s]+))? -->$")
            .unwrap();

    for line in content.lines() {
        let line = line.trim();
        if let Some(caps) = task_re.captures(line) {
            let status = if &caps[1] == "x" {
                TaskStatus::Done
            } else {
                TaskStatus::Todo
            };
            let title = caps[2].to_string();
            let id: u32 = caps[3].parse().unwrap_or(0);
            let created_at: DateTime<Local> = caps[4]
                .parse::<DateTime<Local>>()
                .unwrap_or_else(|_| Local::now());
            let done_at: Option<DateTime<Local>> = caps.get(5).and_then(|m| m.as_str().parse().ok());

            tasks.push(Task {
                id,
                title,
                status,
                created_at,
                done_at,
            });
        }
    }

    Ok(TaskList::with_tasks(tasks))
}

fn format_tasks(task_list: &TaskList) -> String {
    let mut output = String::from(HEADER);

    let todo_tasks: Vec<_> = task_list.tasks.iter().filter(|t| !t.is_done()).collect();
    let done_tasks: Vec<_> = task_list.tasks.iter().filter(|t| t.is_done()).collect();

    output.push_str("## Todo\n\n");
    for task in &todo_tasks {
        output.push_str(&format_task(task));
        output.push('\n');
    }

    output.push_str("\n## Done\n\n");
    for task in &done_tasks {
        output.push_str(&format_task(task));
        output.push('\n');
    }

    output
}

fn format_task(task: &Task) -> String {
    let marker = task.status.marker();
    let created = task.created_at.format("%Y-%m-%dT%H:%M:%S%:z");

    let metadata = if let Some(done_at) = task.done_at {
        let done = done_at.format("%Y-%m-%dT%H:%M:%S%:z");
        format!("id:{} created:{} done:{}", task.id, created, done)
    } else {
        format!("id:{} created:{}", task.id, created)
    };

    format!("- {} {} <!-- {} -->", marker, task.title, metadata)
}

pub fn create_initial_tasks_file(path: &PathBuf) -> Result<()> {
    let task_list = TaskList::new();
    save_tasks(path, &task_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_todo_task() {
        let content = r#"# godo tasks

## Todo

- [ ] Test task <!-- id:1 created:2025-01-07T10:00:00+09:00 -->

## Done

"#;
        let tasks = parse_tasks(content).unwrap();
        assert_eq!(tasks.tasks.len(), 1);
        assert_eq!(tasks.tasks[0].title, "Test task");
        assert_eq!(tasks.tasks[0].status, TaskStatus::Todo);
    }

    #[test]
    fn test_parse_done_task() {
        let content = r#"# godo tasks

## Todo

## Done

- [x] Done task <!-- id:1 created:2025-01-07T10:00:00+09:00 done:2025-01-07T12:00:00+09:00 -->
"#;
        let tasks = parse_tasks(content).unwrap();
        assert_eq!(tasks.tasks.len(), 1);
        assert_eq!(tasks.tasks[0].title, "Done task");
        assert_eq!(tasks.tasks[0].status, TaskStatus::Done);
        assert!(tasks.tasks[0].done_at.is_some());
    }
}
