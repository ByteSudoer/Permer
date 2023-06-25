use chrono::*;
pub fn unix_timestamp_to_datetime(timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 1).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let time_string = format!("{}", datetime.format("%d/%m/%Y %H:%M"));

    time_string
}

pub fn convert_byte_human(bytes: u64) -> String {
    let mut size = bytes as f64;
    let units = ["B", "KB", "MB", "GB", "TB"];

    let mut unit_index = 0;
    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    format!("{:.2} {}", size, units[unit_index])
}
