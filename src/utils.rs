use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::{EST5EDT};
use rand::prelude::*;
use std::ops::Add;

// today midnight auth_token expires
// convert to midnight
pub fn midnight_eastern(days: i64) -> DateTime<Utc> {
    let nows = Utc::now().naive_utc();
    let est = EST5EDT.from_utc_datetime(&nows).date().add(Duration::days(days)).and_hms(0, 0, 0);
    est.with_timezone(&Utc)

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

pub fn gen_id(size: usize) -> String {
    let mask = ALPHA_NUMERIC.len().next_power_of_two() - 1;
    let step: usize = 8 * size / 5;

    // Assert that the masking does not truncate the ALPHA_NUMERIC. (See #9)
    debug_assert!(ALPHA_NUMERIC.len() <= mask + 1);

    let mut id = String::with_capacity(size);

    loop {
        let bytes = gen_rng(step);

        for &byte in &bytes {
            let byte = byte as usize & mask;

            if ALPHA_NUMERIC.len() > byte {
                id.push(ALPHA_NUMERIC[byte]);

                if id.len() == size {
                    return id;
                }
            }
        }
    }
}

fn gen_rng(size: usize) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    let mut result: Vec<u8> = vec![0; size];

    rng.fill(&mut result[..]);

    result
}

pub const ALPHA_NUMERIC: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn simple_id() -> String {
    gen_id(18)
}

// #[macro_export]
// macro_rules! simple_id {
//     () => {
//         gen_id(20)
//     };
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_simple_id() {
        assert_eq!(simple_id!(), "".to_string());;
    }
}


