use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::{EST5EDT};
use std::ops::Add;

// today midnight auth_token expires
// convert to midnight
pub fn midnight_eastern(days: i64) -> DateTime<Utc> {
    let nows = Utc::now().naive_utc();
    let est = EST5EDT.from_utc_datetime(&nows).date().add(Duration::days(days)).and_hms(0, 0, 0);
    est.with_timezone(&Utc)
    //
    // // get us to EST time
    // let mut now = Utc::now() - Duration::hours(4);
    // // today midnight auth_token expires
    // // convert to midnight
    // now = (now + Duration::days(days)).date().and_hms(0, 0, 0);
}

pub fn now_eastern() -> DateTime<Utc> {
    let now = Utc::now().naive_utc();
    let est = EST5EDT.from_utc_datetime(&now);
    // TODO take into account ESD vs EST (fall winter)
    // let res = Utc::now() - Duration::hours(4);
    est.with_timezone(&Utc)
}

pub fn now_plus_hours(hours: i64) -> DateTime<Utc> {
    now_eastern().add(Duration::hours(hours))
}

// use chrono::{TimeZone, Duration};
// // use chrono_tz::{EST5EDT};
// use chrono_tz::America::New_York;
// use std::ops::Add;

// // today midnight auth_token expires
// // convert to midnight
// pub fn midnight_eastern(days: i64) -> DateTime<chrono_tz::Tz> {
//     let now = Utc::now().naive_utc() + Duration::days(days);//).date().and_hms(0, 0, 0);
//     // let mut est = EST5EDT.from_utc_datetime(&now).date().add(Duration::days(days)).and_hms(0, 0, 0);
//     // est = est.with_timezone(&Utc);
//     // let mut est = New_York.from_utc_datetime(&now).date().add(Duration::days(days)).and_hms(0, 0, 0);
//     est
// }

// pub fn now_eastern() -> DateTime<chrono_tz::Tz> {
//     // TODO take into account ESD vs EST (fall winter)
//     let now = Utc::now().naive_utc();//).date().and_hms(0, 0, 0);
//     // get us to EST/EDT time
//     let mut est = EST5EDT.from_utc_datetime(&now).date().add(Duration::days(days)).and_hms(0, 0, 0);
//     // est = est.with_timezone(&Utc);
//     est
// }
