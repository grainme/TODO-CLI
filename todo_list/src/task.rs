use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

/* ----------------------------------------------------------*/

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // open the file!
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // consume the file as a vector of tasks
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Rewind the file after reading from it.
    // otherwise, we'd begin writing at the cursor's last positionwhich
    // which would cause a malformed JSON file
    file.seek(SeekFrom::Start(0))?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

/* ----------------------------------------------------------*/

pub fn complete(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // consume the file as a vector of tasks
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Remove the task at position : task_position
    if task_position == 0 || task_position > tasks.len() {
        // do some errors handling
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    // Rewind and truncate the file.
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    // Write the modified tasks list back to the file
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

/* ----------------------------------------------------------*/

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_tasks(&file)?;
    let mut order = 1;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        for task in tasks.iter() {
            println!("{}: {}", order, task);
            order += 1;
        }
    }
    Ok(())
}

/* ----------------------------------------------------------*/

// we've created this function jsut to avoid repitition
// as it actually called in multiple actions
fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(tasks)
}
