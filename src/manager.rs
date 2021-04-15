///
/// Manager should be used to interfact directly with the data file
/// to perform all core project actions.
/// 
use std::cmp;

use serde::{Serialize, Deserialize};
use prettytable::{Attr, color, Cell, Row, row, cell};

use crate::file::FileAccess;
use crate::{Res, ResErr};
use crate::table::TableDisplay;
use crate::time;

pub const DATE_FORMAT: &str = "%m-%d-%Y";

/// Get the name of the default group, being the local date of today
/// 
/// The value returned from this method should be unique. The uniqueness
/// of this value will NOT be enforced elsewhere.
/// A non-unique value will likely cause unexpected behavior
fn default_group_name() -> String {
    time::today_local().format(DATE_FORMAT).to_string()
}

// --- DATA STRUCTS ---

/// Manages groups of tasks
#[derive(Debug, Deserialize, Serialize)]
pub struct Manager {
    next_group: usize,
    current_group: Option<usize>,
    groups: Vec<Group>
}

/// INIT
impl Manager {

    pub fn init() -> Res<Manager> {
        let file_access = FileAccess::new();

        // Ensure the file exists
        if !file_access.exists() {
            file_access.write(&Manager::new())?;
        }

        let mut manager: Manager = file_access.read()?;

        // Ensure that there is a default group
        let res = manager.add_group(default_group_name());
        if res.is_ok() {
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

/// PUBLIC
impl Manager {

    pub fn group(&mut self) -> Res<&Group> {
        Ok(self.resolve_group()?)
    }

    pub fn use_group(&mut self, group_id: usize) -> Res<Group> {
        let group = self.group_by_id(group_id)
            .ok_or(ResErr::from("Could not find group!"))?;
        let clone = group.clone();

        self.current_group = Some(group.id);

        Ok(clone)
    }

    pub fn reset_group(&mut self) {
        self.current_group = None;
    }

    pub fn add_group(&mut self, name: String) -> Res<Group> {
        // Avoid duplicate named groups
        // Technically they could be supported (since groups have an ID)
        // but it would likely break the default group behavior
        // and would be generally confusing
        if self.group_by_name(&name).is_some() {
            return Err(ResErr::from("Group already exists"));
        }

        let new_group = Group::new(self.next_group, name);
        let clone = new_group.clone();

        self.next_group += 1;
        self.groups.push(new_group);

        Ok(clone)
    }

    pub fn extract_groups(
        &mut self, retain: bool, group_ids: Vec<usize>
    ) -> Res<Vec<Group>> {

        let mut extracted_groups: Vec<Group> = Vec::new();
        let mut extracted_ids: Vec<usize> = Vec::new();

        for g in self.groups.iter() {
            if (retain && !group_ids.contains(&g.id))
                || (!retain && group_ids.contains(&g.id))  {

                extracted_ids.push(g.id);
                extracted_groups.push(g.clone());
            }
        }
        
        // Can't remove the current selected
        if extracted_ids.contains(&self.resolve_group()?.id) {
            return Err(ResErr::from("Cannot archive the current group"));
        }

        // Only retain the ones that weren't extracted
        self.groups.retain(|g| !extracted_ids.contains(&g.id));
        
        Ok(extracted_groups)
    }

    pub fn minimize_ids(&mut self) {
        // Should just be able to process in order
        // We can assume the IDs always get larger as we go
        // This assumes the groups/tasks are sorted by ID
        
        // Always start at 1 (the absolute min)
        let mut next_min = 1;

        for group in &mut self.groups {
            // Get the ID we are processing
            let process_id = group.id;

            // Resolve the current here if necessary
            let is_current = self.current_group
                .map(|c| c == process_id)
                .unwrap_or(false);

            group.id = cmp::min(process_id, next_min);

            if is_current {
                self.current_group = Some(group.id);
            }

            next_min = group.id + 1;
        }

        self.next_group = next_min;
    }

    // GROUP DELEGATES

    pub fn add_task(&mut self, task_name: String) -> Res<Task> {
        self.resolve_group()?.add_task(task_name)
    }

    pub fn remove_task(&mut self, task_id: usize) -> Res<Task> {
        self.resolve_group()?.remove_task(task_id)
    }

    pub fn start_task(&mut self, task_id: usize) -> Res<Task> {
        self.resolve_group()?.start_task(task_id)      
    }

    pub fn stop_current(&mut self) -> Res<Task> {
        self.resolve_group()?.stop_current()
    }

    pub fn complete_task(&mut self, task_id: Option<usize>) -> Res<Task> {
        self.resolve_group()?.complete_task(task_id)
    }
}

/// PRIVATE
impl Manager {
    /// Create a base manager. 
    /// This will likely only be called once, and then is on file creation
    fn new() -> Self {
        Manager {
            next_group: 1,
            current_group: None,
            groups: Vec::new()
        }
    }
    
    /// Get a mut group by searching by ID
    fn group_by_id(&mut self, group_id: usize) -> Option<&mut Group> {
        for group in &mut self.groups {
            if group.id == group_id {
                return Some(group);
            }
        }

        None
    }

    /// Get a mut group by searching by name
    fn group_by_name(&mut self, group_name: &String) -> Option<&mut Group> {
        for group in &mut self.groups {
            if group.name == *group_name {
                return Some(group);
            }
        }

        None
    }

    /// Resolve the current group.
    /// This method assumes the default group already exists,
    /// and it will NOT create it on the fly.
    fn resolve_group(&mut self) -> Res<&mut Group> {
        match self.current_group {
            Some(curr) => {
                // Find the current group
                return self.group_by_id(curr)
                    .ok_or(ResErr::from("Could not resolve existing group!"))
            },
            _ => {
                // Find the default group using the group_name()
                let name = default_group_name();
                return self.group_by_name(&name)
                    .ok_or(ResErr::from("Could not resolve default group!"));
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    id: usize,
    next_task: usize,
    current_task: Option<usize>,
    name: String,
    tasks: Vec<Task>
}

impl Group {
    fn new(id: usize, name: String) -> Self {
        Group {
            id: id,
            next_task: 1,
            current_task: None,
            name: name,
            tasks: Vec::new()
        }
    }

    fn add_task(&mut self, task_name: String) -> Res<Task> {
        let task = Task::new(self.next_task, task_name);
        let clone = task.clone();
        
        self.next_task += 1;
        self.tasks.push(task);

        Ok(clone)
    }

    fn remove_task(&mut self, task_id: usize) -> Res<Task> {
        let task = self.task_mut(task_id)
            .ok_or(ResErr::from("Could not find task in group"))?;
        let clone = task.clone();

        self.tasks.retain(|t| *t != clone);

        if self.current_task.filter(|curr| *curr == clone.id).is_some() {
            self.current_task = None;
        }

        Ok(clone)
    }

    fn start_task(&mut self, task_id: usize) -> Res<Task> {
        let task = self.task_mut(task_id)
            .ok_or(ResErr::from("Could not find task in group!"))?;
        let clone = task.clone();

        task.start();

        // Stop current if there is a current
        if self.current_task.is_some() {
            self.stop_current()?;
        }
    
        // Set the current task
        self.current_task = Some(task_id);

        Ok(clone)        
    }

    fn stop_current(&mut self) -> Res<Task> {
        let task = self.current_task
            .and_then(|curr| self.task_mut(curr))
            .ok_or(ResErr::from("Could not find current task in group"))?;
        let clone = task.clone();

        // Stop the task
        task.stop();        

        // Reset the current task
        self.current_task = None;

        Ok(clone)
    }

    fn complete_task(&mut self, task_id: Option<usize>) -> Res<Task> {
        let id = task_id
            .or_else(|| self.current_task)
            .ok_or(ResErr::from("No task or current task!"))?;
        
        let task = self.task_mut(id)
            .ok_or(ResErr::from("Could not find task in group!"))?;

        task.complete();
        let clone = task.clone();
    
        Ok(clone)
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

    // GETTERS

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Represents an individual task to complete.
/// Holds data necessary in computing time tracked
/// for a task
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    id: usize,
    name: String,
    started_date: Option<i64>,
    tracked: Option<i64>,
    is_complete: bool
}

impl Task {
    fn new(id: usize, name: String) -> Self {
        Task {
            id: id,
            name: name,
            started_date: None,
            tracked: None,
            is_complete: false
        }
    }

    /// Give the Task a timestamp as started_date
    /// This timestamp represents the time a task started in its current run
    fn start(&mut self) {
        self.started_date = Some(time::timestamp());

        // Un-complete the task if it is started
        self.is_complete = false;
    }

    /// Stop the task.
    /// This will erase the started_date timestamp and
    /// append tracked time to the tracked field
    fn stop(&mut self) {
        let tracked = self.tracked.unwrap_or(0);

        if let Some(started) = self.started_date {
            let now = time::timestamp();
            self.tracked = Some(tracked + (now - started));
        }

        self.started_date = None
    }

    /// Complete the task
    /// Will the stop the current task, and mark as cimplete
    fn complete(&mut self) {
        // Stop the task (it could be currently running)
        self.stop();
        self.is_complete = true;
    }
}

/// Compare tasks by their ID
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
        let is_complete = self.is_complete;

        let style = |cell: Cell| -> Cell {
            if is_complete {
                return cell
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN));
            } else if is_started {
                return cell
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BRIGHT_RED));
            }

            cell
        };

        // Display complete/stopped depending on the complete status
        // of the task
        let started_display = || -> String {
            return if is_complete { String::from("COMPLETE") } else { String::from("STOPPED") }
        };

        let v = vec![
            style(Cell::new(&self.id.to_string())),
            style(Cell::new(&self.name)),
            style(Cell::new(
                &self.started_date
                    .map(|sd| time::to_local_datetime(sd)
                        .format("%B %e %r %Y")
                        .to_string())
                    .unwrap_or(started_display())
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