use mongodb::bson::DateTime as BsonDateTime;
use chrono::{Local, TimeZone, Datelike};

// Converts BsonDateTime to i64 timestamp in milliseconds
pub fn get_long_timestamp(dt: BsonDateTime) -> i64 {
    dt.timestamp_millis()
}

pub fn get_option_long_timestamp(datetime: Option<BsonDateTime>) -> Option<i64> {
    datetime.map(|dt| dt.timestamp_millis())
}

// Returns true if the BsonDateTime is today or later
pub fn is_today_or_later(dt: BsonDateTime) -> bool {
    let now = Local::now();

    // Get the start of today (midnight)
    let start_of_today = Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
        .unwrap();

    // Compare timestamps using millis
    let input_millis = dt.timestamp_millis();
    let today_millis = start_of_today.timestamp_millis();

    input_millis >= today_millis
}
