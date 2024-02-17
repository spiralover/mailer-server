use chrono::{Duration, Local, NaiveDateTime};

pub fn now_plus_seconds(sec: i64) -> NaiveDateTime {
    (Local::now() + Duration::seconds(sec)).naive_local()
}

pub fn now_plus_minutes(min: i64) -> NaiveDateTime {
    now_plus_seconds(min * 60)
}

pub fn current_timestamp() -> NaiveDateTime {
    Local::now().naive_local()
}
