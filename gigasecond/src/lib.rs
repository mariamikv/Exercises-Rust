use time::{OffsetDateTime, PrimitiveDateTime as DateTime, UtcOffset};

pub fn after(start: DateTime) -> DateTime {
    seconds_to_primitive_date_time(crate::seconds(start) + 1000000000)
}

pub fn seconds(date: DateTime) -> i64 {
    date.assume_utc().unix_timestamp()
}

pub fn seconds_to_primitive_date_time(seconds: i64) -> DateTime {
    let offset_dt = OffsetDateTime::from_unix_timestamp(seconds).unwrap();
    let utc_dt = offset_dt.to_offset(UtcOffset::UTC);
    DateTime::new(utc_dt.date(), utc_dt.time())
}