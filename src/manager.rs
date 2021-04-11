use serde::{Serialize, Deserialize};
use prettytable::{Attr, color, Cell, Row, row, cell};

use crate::file::FileAccess;
use crate::{Res, ResErr};
use crate::table::TableDisplay;
use crate::time;

fn default_group_name() -> String {
    time::today_local().format("%m-%d-%Y").to_string()
}

// --- DATA STRUCTS ---

/// Manages groups of tasks
#[derive(Debug, Deserialize, Serialize)]
pub struct Manager {
    next_task: usize,
    current_task: Option<usize>,
    next_group: usize,
    current_group: Option<usize>,
    groups: Vec<Group>
}

impl Manager {

    pub fn init() -> Res<Manager> {
        let file_access = FileAccess::new();

        // Ensure the file exists
        if !file_access.exists() {
            file_access.write(&Manager::new())?;
        }

        let mut manager: Manager = file_access.read()?;

        // Ensure that there is a default group
        let name = default_group_name();
        if manager.group_by_name(&name).is_none() {
            let new_group = Group::new(manager.next_group, name);
            manager.next_group += 1;
            manager.groups.push(new_group);

            file_access.write(&manager)?;
        }

        Ok(manager)
    }

    pub fn commit(&self) -> Res<()> {
        let file_access = FileAccess::new();
        file_access.write(self)?;

        Ok(())
    }
}

/// Private methods
impl Manager {
    /// Create a base manager. 
    /// This will likely only be called once, and then is on file creation
    fn new() -> Self {
        Manager {
            next_task: 1,
            next_group: 1,
            current_task: None,
            current_group: None,
            groups: Vec::new()
        }
    }
    
    fn group_by_id(&mut self, group_id: usize) -> Option<&mut Group> {
        for group in &mut self.groups {
            if group.id == group_id {
                return Some(group);
            }
        }

        None
    }

    fn group_by_name(&mut self, group_name: &String) -> Option<&mut Group> {
        for group in &mut self.groups {
            if group.name == *group_name {
                return Some(group);
            }
        }

        None
    }

    fn resolve_group(&mut self) -> Res<&mut Group> {
        match self.current_group {
            Some(curr) => {
                return self.group_by_id(curr)
                    .ok_or(ResErr::from("Could not resolve existing group!"))
            },
            _ => {
                let name = default_group_name();
                return self.group_by_name(&name)
                    .ok_or(ResErr::from("Could not resolve default group!"));
            }
        }
    }
}

/// Public methods
impl Manager {

    pub fn group(&mut self) -> Res<&Group> {
        Ok(self.resolve_group()?)
    }

    /// Add a task with the task_name to the group with group_name
    /// Return a Res<Task>. The returned Task is a clone, implying that it
    /// cannot be used to modify the existing data structure.
    pub fn add_task(&mut self, task_name: String) -> Res<Task> {
        let task = Task::new(self.next_task, task_name);
        let clone = task.clone();
        
        // Ensure the next_id is ready for the next addition
        self.next_task += 1;

        let group = self.resolve_group()?;
        group.tasks.push(task);

        Ok(clone)
    }

    /// Remove a task with id: task_id from the group with name: group_name
    /// Return a Res<Task>. The returned Task is a clone, implying that it
    /// cannot be used to modify the existing data structure.
    pub fn remove_task(&mut self, task_id: usize) -> Res<Task> {
        let group = self.resolve_group()?;
        let task = group.task_mut(task_id)
            .ok_or(ResErr::from("Could not find task in group"))?;

        let clone = task.clone();
        group.tasks.retain(|t| *t != clone);

        if self.current_task.filter(|curr| *curr == clone.id).is_some() {
            self.current_task = None;
        }

        Ok(clone)
    }

    pub fn use_group(&mut self, group_id: usize) -> Res<Group> {
        let group = self.group_by_id(group_id)
            .ok_or(ResErr::from("Could not find group!"))?;
        let clone = group.clone();

        self.current_group = Some(group.id);

        Ok(clone)
    }

    pub fn reset_group(&mut self) {
        // Just reset the group (none)
        self.current_group = None;
    }

    /// Start the task with the task_id
    /// Return a Res<Task>. The returned Task is a clone, implying that it
    /// cannot be used to modify the existing data structure.
    pub fn start_task(&mut self, task_id: usize) -> Res<Task> {
        let group = self.resolve_group()?;
        let task = group.task_mut(task_id)
            .ok_or(ResErr::from("Could not find task in group!"))?;

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
        let current = self.current_task;

        let group = self.resolve_group()?;
        let task = current
            .and_then(|curr| group.task_mut(curr))
            .ok_or(ResErr::from("Could not find current task in group"))?;

        // Stop the task
        task.stop();

        // Clone the task so we can be done with the mut ref
        let clone = task.clone();

        // Reset the current task
        self.current_task = None;

        Ok(clone)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    id: usize,
    name: String,
    tasks: Vec<Task>
}

impl Group {
    fn new(id: usize, name: String) -> Self {
        Group {
            id: id,
            name: name,
            tasks: Vec::new()
        }
    }

    /// Get the task with id: task_id as mutable from this group
    fn task_mut(&mut self, task_id: usize) -> Option<&mut Task> {
        for task in &mut self.tasks {
            if task.id == task_id {
                return Some(task);
            }
        }

        None
    }

    pub fn name(&self) -> &String {
        &self.name
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

impl TableDisplay for Manager {

    fn header(&self) -> Row {
        row!["ID", "Group"]
    }

    fn rows(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();

        let style = |cell: Cell, is_current: bool| -> Cell {
            if is_current {
                return cell
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BRIGHT_RED));
            }

            cell
        };

        for g in &self.groups {  
            let is_current =
            self.current_group.filter(|curr| *curr == g.id).is_some()
                || (g.name == default_group_name() && self.current_group.is_none());

            let v = vec![
                style(Cell::new(&g.id.to_string()), is_current),
                style(Cell::new(&g.name), is_current)
            ];

            rows.push(Row::new(v));
        }

        rows
    }
}

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