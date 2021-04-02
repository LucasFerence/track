use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::file::FileAccess;
use crate::Res;

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