use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime, Utc};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    next_id: usize,
    days: Vec<Day>
}