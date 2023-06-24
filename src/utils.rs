use chrono::*;
pub fn unix_timestamp_to_datetime(timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 1).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let time_string = format!("{}", datetime.format("%d/%m/%Y %H:%M"));

    time_string
}
