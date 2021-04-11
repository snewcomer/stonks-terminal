use chrono::prelude::*;
use chrono::Duration;

// today midnight auth_token expires
// convert to midnight
pub fn midnight_eastern(days: i64) -> DateTime<Utc> {
    // get us to EST time
    let mut now = Utc::now() - Duration::hours(5);
    // today midnight auth_token expires
    // convert to midnight
    now = (now + Duration::days(days)).date().and_hms(0, 0, 0);
    now
}

pub fn now_eastern() -> DateTime<Utc> {
    Utc::now() - Duration::hours(5)
}

