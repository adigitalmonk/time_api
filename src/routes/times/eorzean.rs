static HOUR: i64 = 3600;
static MINUTE: i64 = 60;
static SECOND: i64 = 1;
static EORZEAN_CONSTANT: f64 = 20.571428571428573;

pub fn get_eorzean_time(current_timestamp: i64) -> (i64, i64, i64) {
    let eorzea_timestamp = ((current_timestamp as f64) * EORZEAN_CONSTANT) as i64;
    let eorzean_hour = eorzea_timestamp / HOUR % 24;
    let eorzean_minute = eorzea_timestamp / MINUTE % 60;
    let eorzean_second = eorzea_timestamp / SECOND % 60;

    (eorzean_hour, eorzean_minute, eorzean_second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_time_at_epoch() {
        assert_eq!(get_eorzean_time(0), (0, 0, 0));
    }

    #[test]
    fn test_get_time_at_some_date() {
        assert_eq!(get_eorzean_time(1674704475423), (19, 33, 36));
    }
}
