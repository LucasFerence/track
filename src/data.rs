use serde::{Serialize, Deserialize};
use chrono::{DateTime, Date, NaiveDateTime, Utc};

// Created when `track new {description}`
#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    id: usize,
    description: String,
    date: i64
}

impl Entry {
    pub fn new(desc: String) -> Self {
        Entry {
            id: 1,
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
    day: String,
    entries: Vec<Entry>
}

impl Day {
    pub fn new(first_entry: Entry) -> Self {
        Day {
            day: Utc::now().date().to_string(),
            entries: vec!(first_entry)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    next_id: usize,
    days: Vec<Day>
}

impl Root {
    pub fn new() -> Self {
        Root {
            next_id: 1,
            days: Vec::new()
        }
    }
}