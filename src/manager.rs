use serde::{Serialize, Deserialize};
use prettytable::{Attr, color, Cell, Row, row, cell};

use crate::file::FileAccess;
use crate::{Res, ResErr};
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

    /// Create a base manager. 
    /// This will likely only be called once, and then is on file creation
    fn new() -> Self {
        Manager {
            next_id: 1,
            current_task: None,
            groups: Vec::new()
        }
    }
    
    /// Get the group with name: group_name as mutable
    fn group_mut(&mut self, group_name: &String) -> Option<&mut Group> {
        for group in &mut self.groups {
            if group.name == *group_name {
                return Some(group);
            }
        }

        None
    }

    /// Get the task with id: task_id as mutable
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

    // -- External access methods --

    /// Get the group with name: group_name
    pub fn group(&mut self, group_name: &String) -> Option<&Group> {
        Some(self.group_mut(group_name)?)
    }

    /// Get the task with id: task_id
    pub fn task(&mut self, task_id: usize) -> Option<&Task> {
        Some(self.task_mut(task_id)?)
    }

    // -- Mutator methods --

    /// Add a task with the task_name to the group with group_name
    /// Return a Res<Task>. The returned Task is a clone, implying that it
    /// cannot be used to modify the existing data structure.
    pub fn add_task(&mut self, group_name: String, task_name: String) -> Res<Task> {
        let task = Task::new(self.next_id, task_name);
        let clone = task.clone();
        
        // Ensure the next_id is ready for the next addition
        self.next_id += 1;

        // Find a group if you can, and add the task to it
        // Otherwise, just create a new one
        match self.group_mut(&group_name) {
            Some(group) => group.tasks.push(task),
            _ => self.groups.push(Group::new(group_name, task))
        }

        Ok(clone)
    }

    /// Remove a task with id: task_id from the group with name: group_name
    /// Return a Res<Task>. The returned Task is a clone, implying that it
    /// cannot be used to modify the existing data structure.
    pub fn remove_task(&mut self, group_name: String, task_id: usize) -> Res<Task> {
        let task = self.task_mut(task_id)
            .ok_or(ResErr::from("Could not find task"))?;
        let clone = task.clone();

        let group = self.group_mut(&group_name)
            .ok_or(ResErr::from("Could not find group"))?;

        group.tasks.retain(|t| *t != clone);

        if self.current_task.filter(|curr| *curr == clone.id).is_some() {
            self.current_task = None;
        }

        Ok(clone)
    }

    /// Start the task with the task_id
    /// Return a Res<Task>. The returned Task is a clone, implying that it
    /// cannot be used to modify the existing data structure.
    pub fn start_task(&mut self, task_id: usize) -> Res<Task> {
        // Get the task from the task_id
        let task = self.task_mut(task_id)
            .ok_or(ResErr::from("Could not find task"))?;

        // Begin the current task
        task.start();

        // Clone the task so we can be done with the mut ref
        let clone = task.clone();

        // Stop current if there is a current
        if self.current_task.is_some() {
            self.stop_current()?;
        }
    
        // Set the current task
        self.current_task = Some(task_id);

        Ok(clone)        
    }

    /// Stop the current running task
    /// Return a Res<Task>. The returned Task is a clone, implying that it
    /// cannot be used to modify the existing data structure.
    pub fn stop_current(&mut self) -> Res<Task> {
        // Get the task based on the current running task
        let task = self.current_task
            .and_then(|curr| self.task_mut(curr))
            .ok_or(ResErr::from("Could not find current task"))?;

        // Stop the task
        task.stop();

        // Clone the task so we can be done with the mut ref
        let clone = task.clone();

        // Reset the current task
        self.current_task = None;

        Ok(clone)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    name: String,
    tasks: Vec<Task>
}

impl Group {
    fn new(name: String, initial_task: Task) -> Self {
        Group {
            name: name,
            tasks: vec![initial_task]
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl PartialEq for Task {
    
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
            rows.append(&mut e.rows());
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

        let is_started = self.started_date.is_some();

        let style = |cell: Cell| -> Cell {
            if is_started {
                return cell
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BRIGHT_RED));
            }

            cell
        };

        let v = vec![
            style(Cell::new(&self.id.to_string())),
            style(Cell::new(&self.name)),
            style(Cell::new(
                &self.started_date
                    .map(|sd| time::to_local_datetime(sd)
                        .format("%B %e %r %Y")
                        .to_string())
                    .unwrap_or(String::from("STOPPED"))
            )),
            style(Cell::new(
                &self.started_date
                        .map(|sd| {
                            
                            let tracked = self.tracked.unwrap_or(0);
                            let now = time::timestamp();
                            // now minus sd plus tracked
                            
                            time::duration_str(tracked + (now - sd))
                        })
                        .or_else(|| self.tracked.map(|sd| time::duration_str(sd)))
                        .unwrap_or(String::from("NONE"))
            ))
        ];

        rows.push(Row::new(v));

        rows
    }
}