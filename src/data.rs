use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use prettytable::{Cell, Row, row, cell};

use crate::file::FileAccess;
use crate::Res;
use crate::table::TableDisplay;

// Ensure the file exists
pub fn ensure_file() -> Res<()> {
    let file_access = FileAccess::new();

    // Create it if it doesn't exist
    if !file_access.exists() {
        file_access.write(&Root::new())?;
    }

    Ok(())
}

// Created when `track new {description}`
#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    id: usize,
    description: String,
    date: i64
}

impl Entry {
    fn new(id: usize, desc: String) -> Self {
        Entry {
            id: id,
            description: desc,
            date: Utc::now().timestamp()
        }
    }

    pub fn date(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(self.date, 0),
            Utc
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Day {
    val: String,
    entries: Vec<Entry>
}

impl Day {
    pub fn new() -> Self {
        Day {
            val: get_today_string(),
            entries: Vec::new()
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}

impl TableDisplay for Day {
    
    fn header(&self) -> Row {
        row!["ID", "Task"]
    }

    fn rows(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();

        for e in self.entries() {
            let v = vec![
                Cell::new(&e.id.to_string()),
                Cell::new(&e.description)
            ];

            rows.push(Row::new(v));
        }

        rows
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    next_id: usize,
    days: Vec<Day>
}

impl Root {
    fn new() -> Self {
        Root {
            next_id: 1,
            days: Vec::new()
        }
    }

    pub fn create_entry(&mut self, desc: String) -> Entry {
        let new_entry = Entry::new(self.next_id, desc);
        self.next_id += 1;

        new_entry
    }

    pub fn today(&mut self) -> Option<&mut Day> {
        let today_string = get_today_string();
        for day in &mut self.days {
            if day.val == today_string {
                return Some(day)
            }
        }

        None
    }

    pub fn add_day(&mut self, day: Day) {
        self.days.push(day);
    }
}

fn get_today_string() -> String {
    Utc::now().date().to_string()
}