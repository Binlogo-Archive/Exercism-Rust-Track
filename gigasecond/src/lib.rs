use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // unimplemented!("What time is a gigasecond later than {}", start);
    const GIGASECOND: i64 = 1_000_000_000;
    start + Duration::seconds(GIGASECOND)
}
