use chrono::{DateTime, Utc};

pub fn local_timestamp() -> DateTime<Utc> {
    Utc::now()
}
