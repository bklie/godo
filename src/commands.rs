use crate::config::{config_path, godo_dir, is_initialized, Config};
use crate::error::{GodoError, Result};
use crate::storage::{create_initial_tasks_file, load_tasks, save_tasks};

pub fn init() -> Result<()> {
    let godo_dir = godo_dir()?;
    let config_path = config_path()?;

    if config_path.exists() {
        return Err(GodoError::AlreadyInitialized(godo_dir));
    }

    // Create directory
    std::fs::create_dir_all(&godo_dir).map_err(|e| GodoError::CreateDir {
        path: godo_dir.clone(),
        source: e,
    })?;

    // Create config file
    let config = Config::default();
    config.save(&config_path)?;
    println!("Created: {}", config_path.display());

    // Create tasks file
    let tasks_path = config.data_file_path()?;
    create_initial_tasks_file(&tasks_path)?;
    println!("Created: {}", tasks_path.display());

    println!("godo initialized successfully!");

    Ok(())
}

pub fn add(title: String) -> Result<()> {
    ensure_initialized()?;

    let config = load_config()?;
    let tasks_path = config.data_file_path()?;
    let mut tasks = load_tasks(&tasks_path)?;

    let task = tasks.add(title);
    let id = task.id;
    let title = task.title.clone();

    save_tasks(&tasks_path, &tasks)?;

    println!("Added: [{}] {}", id, title);

    Ok(())
}

pub fn list(show_all: bool) -> Result<()> {
    ensure_initialized()?;

    let config = load_config()?;
    let tasks_path = config.data_file_path()?;
    let tasks = load_tasks(&tasks_path)?;

    let tasks_to_show: Vec<_> = if show_all {
        tasks.all_tasks().collect()
    } else {
        tasks.todo_tasks().collect()
    };

    if tasks_to_show.is_empty() {
        if show_all {
            println!("No tasks.");
        } else {
            println!("No pending tasks.");
        }
        return Ok(());
    }

    println!("  ID  Status  Task");
    println!("----  ------  ----");

    for task in tasks_to_show {
        println!(
            "{:>4}  {}     {}",
            task.id,
            task.status.marker(),
            task.title
        );
    }

    Ok(())
}

pub fn done(id: u32) -> Result<()> {
    ensure_initialized()?;

    let config = load_config()?;
    let tasks_path = config.data_file_path()?;
    let mut tasks = load_tasks(&tasks_path)?;

    let task = tasks.find_mut(id).ok_or(GodoError::TaskNotFound(id))?;
    task.mark_done();
    let title = task.title.clone();

    save_tasks(&tasks_path, &tasks)?;

    println!("Done: [{}] {}", id, title);

    Ok(())
}

pub fn rm(id: u32) -> Result<()> {
    ensure_initialized()?;

    let config = load_config()?;
    let tasks_path = config.data_file_path()?;
    let mut tasks = load_tasks(&tasks_path)?;

    let task = tasks.remove(id).ok_or(GodoError::TaskNotFound(id))?;

    save_tasks(&tasks_path, &tasks)?;

    println!("Removed: [{}] {}", id, task.title);

    Ok(())
}

pub fn edit(id: u32, new_title: String) -> Result<()> {
    ensure_initialized()?;

    let config = load_config()?;
    let tasks_path = config.data_file_path()?;
    let mut tasks = load_tasks(&tasks_path)?;

    let task = tasks.find_mut(id).ok_or(GodoError::TaskNotFound(id))?;
    task.title = new_title.clone();

    save_tasks(&tasks_path, &tasks)?;

    println!("Updated: [{}] {}", id, new_title);

    Ok(())
}

fn ensure_initialized() -> Result<()> {
    if !is_initialized() {
        return Err(GodoError::NotInitialized);
    }
    Ok(())
}

fn load_config() -> Result<Config> {
    let config_path = config_path()?;
    Config::load(&config_path)
}
