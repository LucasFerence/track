use serde::{Serialize, Deserialize};
use prettytable::{Attr, color, Cell, Row, row, cell};

use crate::file::FileAccess;
use crate::Res;
use crate::table::TableDisplay;
use crate::time;

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

    pub fn task(&self, task_id: usize) -> Option<&Task>{
        for group in &self.groups {
            for task in &group.tasks {
                if task.id == task_id {
                    return Some(task);
                }
            }
        }

        None
    }

    // Return the ID of the started task
    pub fn start_task(&mut self, task_id: usize) -> Res<usize> {
        if let Some(task) = self.task_mut(task_id) {
            task.start();
            self.current_task = Some(task.id);
            return Ok(task_id);
        }

        Err(Box::from("Could not find task"))
    }

    // Return the ID of the stopped task
    pub fn stop_current(&mut self) -> Res<usize> { 
        if let Some(curr) = self.current_task {
            if let Some(task) = self.task_mut(curr) {
                task.stop();
                let id = task.id;
                self.current_task = None;

                return Ok(id)
            }
        }

        Err(Box::from("Could not find current task"))
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
        self.started_date = Some(time::timestamp());
    }

    fn stop(&mut self) {
        let tracked = self.tracked.unwrap_or(0);
        if let Some(started) = self.started_date {
            let now = time::timestamp();
            self.tracked = Some(tracked + (now - started));
        }

        self.started_date = None
    }
}

// --- Table Display ---

// TODO: I would like to move this TableDisplay stuff out of this file
impl TableDisplay for Group {
    
    fn header(&self) -> Row {
        row!["ID", "Task", "Started", "Time Tracked"]
    }

    fn rows(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();

        // Will apply a style if `should` is true
        let style = |cell: Cell, should: bool| -> Cell {
            if should {
                return cell
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BRIGHT_RED));
            }

            cell
        };

        for e in &self.tasks {  
            let is_started = e.started_date.is_some();

            let v = vec![
                style(Cell::new(&e.id.to_string()), is_started),
                style(Cell::new(&e.name), is_started),
                style(Cell::new(
                    &e.started_date
                        .map(|sd| {
                            // This definitely needs to be refactored
                            if let Some(dt) = time::to_datetime(sd) {
                                return dt.to_string();
                            }

                            return String::new();
                        })
                        .unwrap_or(String::from("STOPPED"))
                ), is_started),
                style(Cell::new(
                    // Use started time (its running) or else tracked (if its stopped)
                    &e.started_date
                        .map(|sd| {
                            
                            let tracked = e.tracked.unwrap_or(0);
                            let now = time::timestamp();
                            // now minus sd plus tracked
                            
                            time::duration_str(tracked + (now - sd))
                        })
                        .or_else(|| e.tracked.map(|sd| time::duration_str(sd)))
                        .unwrap_or(String::from("NONE"))
                ), is_started)
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
                    .map(|sd| {
                        // This definitely needs to be refactored
                        if let Some(dt) = time::to_datetime(sd) {
                            return dt.to_string();
                        }

                        return String::new();
                    })
                    .unwrap_or(String::from("STOPPED"))
            ),
            Cell::new(
                &self.started_date
                        .map(|sd| {
                            
                            let tracked = self.tracked.unwrap_or(0);
                            let now = time::timestamp();
                            // now minus sd plus tracked
                            
                            time::duration_str(tracked + (now - sd))
                        })
                        .or_else(|| self.tracked.map(|sd| time::duration_str(sd)))
                        .unwrap_or(String::from("NONE"))
            )
        ];

        rows.push(Row::new(v));

        rows
    }
}