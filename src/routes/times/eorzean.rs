use chrono::{DateTime, Utc};

static HOUR: i64 = 3600;
static MINUTE: i64 = 60;
static SECOND: i64 = 1;

fn get_eorzean_time(current_timestamp: i64) -> (i64, i64, i64) {
    // 60 * 24 Eorzean minutes (one day) per 70 real-world minutes.
    let eorzea_timestamp = current_timestamp * 60 * 24 / 70;

    let eorzean_hour = eorzea_timestamp / HOUR % 24;
    let eorzean_minute = eorzea_timestamp / MINUTE % 60;
    let eorzean_second = eorzea_timestamp / SECOND % 60;

    (eorzean_hour, eorzean_minute, eorzean_second)
}

pub fn from_datetime(current_time: DateTime<Utc>) -> String {
    let (ez_hour, ez_minute, ez_second) = get_eorzean_time(current_time.timestamp());
    format!("{ez_hour:02}:{ez_minute:02}:{ez_second:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_eorzean_format() {
        let epoch = DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let formatted_result = from_datetime(epoch);

        assert_eq!(formatted_result, format!("00:00:00"));
    }

    #[test]
    fn test_get_time_at_epoch() {
        assert_eq!(get_eorzean_time(0), (0, 0, 0));
    }

    #[test]
    fn test_get_time_at_some_date() {
        assert_eq!(get_eorzean_time(1_674_704_475_423), (19, 33, 36));
    }
}
