use serde::{Serialize, Deserialize};
use chrono::Local;
use prettytable::{Cell, Row, row, cell};

use crate::file::FileAccess;
use crate::Res;
use crate::table::TableDisplay;

// Ensure the file exists
pub fn ensure_file() -> Res<()> {
    let file_access = FileAccess::new();

    // Create it if it doesn't exist
    if !file_access.exists() {
        file_access.write(&Manager::new())?;
    }

    Ok(())
}

// --- DATA STRUCTS ---

/// Manages groups of tasks
#[derive(Debug, Deserialize, Serialize)]
pub struct Manager {
    next_id: usize,
    current_task: Option<usize>,
    groups: Vec<Group>
}

/// Private methods
impl Manager {
    fn new() -> Self {
        Manager {
            next_id: 1,
            current_task: None,
            groups: Vec::new()
        }
    }

    fn group_mut(&mut self, group_name: &String) -> Option<&mut Group> {
        for group in &mut self.groups {
            if group.name == *group_name {
                return Some(group);
            }
        }

        None
    }

    fn task_mut(&mut self, task_id: usize) -> Option<&mut Task> {
        for group in &mut self.groups {
            for task in &mut group.tasks {
                if task.id == task_id {
                    return Some(task);
                }
            }
        }

        None
    }
}

/// Public methods
impl Manager {
    pub fn add_task(&mut self, group_name: String, task_name: String) {
        let task = Task::new(self.next_id, task_name);
        self.next_id += 1;

        // Find a group if you can, and add the task to it
        // Otherwise, just create a new one
        match self.group_mut(&group_name) {
            Some(group) => {
                group.tasks.push(task);
            },
            None => {
                let mut group = Group::new(group_name);
                group.tasks.push(task);
                self.groups.push(group);
            }
        }
    }

    pub fn group(&self, group_name: &String) -> Option<&Group> {
        for group in &self.groups {
            if group.name == *group_name {
                return Some(group)
            }
        }

        None
    }

    pub fn start_task(&mut self, task_id: usize) {
        if let Some(task) = self.task_mut(task_id) {
            task.start();
            self.current_task = Some(task.id)
        }
    }

    pub fn stop_current(&mut self) {
        if let Some(curr) = self.current_task {
            if let Some(task) = self.task_mut(curr) {
                task.stop();
                self.current_task = None
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    name: String,
    tasks: Vec<Task>
}

impl Group {
    fn new(name: String) -> Self {
        Group {
            name: name,
            tasks: Vec::new()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: usize,
    name: String,
    started_date: Option<i64>,
    tracked: Option<i64>
}

impl Task {
    fn new(id: usize, name: String) -> Self {
        Task {
            id: id,
            name: name,
            started_date: None,
            tracked: None
        }
    }

    fn start(&mut self) {
        self.started_date = Some(Local::now().timestamp());
    }

    fn stop(&mut self) {
        let tracked = self.tracked.unwrap_or(0);
        if let Some(started) = self.started_date {
            let now = Local::now().timestamp();
            self.tracked = Some(tracked + (now - started));
        }

        self.started_date = None
    }
}

// --- Table Display ---

impl TableDisplay for Group {
    
    fn header(&self) -> Row {
        row!["ID", "Task", "Started", "Time Tracked"]
    }

    fn rows(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();

        for e in &self.tasks {
            let v = vec![
                Cell::new(&e.id.to_string()),
                Cell::new(&e.name),
                Cell::new(
                    &e.started_date
                        .map(|sd| sd.to_string())
                        .unwrap_or(String::from("STOPPED"))
                ),
                Cell::new(
                    &e.tracked
                        .map(|sd| sd.to_string())
                        .unwrap_or(String::from("NONE"))
                )
            ];

            rows.push(Row::new(v));
        }

        rows
    }
}

impl TableDisplay for Task {
    fn header(&self) -> Row {
        row!["ID", "Task", "Started", "Time Tracked"]
    }

    fn rows(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();

        let v = vec![
            Cell::new(&self.id.to_string()),
            Cell::new(&self.name),
            Cell::new(
                &self.started_date
                    .map(|sd| sd.to_string())
                    .unwrap_or(String::from("STOPPED"))
            ),
            Cell::new(
                &self.tracked
                    .map(|sd| sd.to_string())
                    .unwrap_or(String::from("NONE"))
            )
        ];

        rows.push(Row::new(v));

        rows
    }
}