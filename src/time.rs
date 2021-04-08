use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime, Duration};

pub fn today_date() -> String {
    Local::now().date().to_string()
}

pub fn timestamp() -> i64 {
    Local::now().timestamp()
}

pub fn to_datetime(stamp: i64) -> Option<DateTime<Local>> {
    Local.from_local_datetime(
        &NaiveDateTime::from_timestamp(stamp, 0)
    ).latest()
}

pub fn duration_str(stamp: i64) -> String {
    let duration = Duration::seconds(stamp);

    format!("{}h, {}m, {}s",
        duration.num_hours(),
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    )
}