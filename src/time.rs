use chrono::{offset::TimeZone, DateTime, Utc, Local, NaiveDateTime, Duration, Date};

pub fn today() -> Date<Utc> {
    Utc::now().date()
}

pub fn today_local() -> Date<Local> {
    today().with_timezone(&Local)
}

pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn to_datetime(stamp: i64) -> DateTime<Utc> {
    Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(stamp, 0))
}

pub fn to_local_datetime(stamp: i64) -> DateTime<Local> {
    to_datetime(stamp).with_timezone(&Local)
}

pub fn duration_str(stamp: i64) -> String {
    let duration = Duration::seconds(stamp);

    format!("{}h, {}m, {}s",
        duration.num_hours(),
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    )
}