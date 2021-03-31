use std::time::Instant;

// Created when `track new {description}`
#[derive(Serialize, Deserialize)]
pub struct Entry {
    id: u8,
    description: String,
    began: Instant
}

#[derive(Serialize, Deserialize)]
pub struct Day {
    day: String,
    entries: Vec<Entry>
}

#[derive(Serialize, Deserialize)]
pub struct Week {
    days: Vec<Day>
}