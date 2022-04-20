use chrono::ParseError;

use super::{constants::DATE_FORMAT, log::LogExpectResult};

pub fn get_backup_time(name: &str, filename: &str) -> Result<i64, ParseError> {
    let date_string = &filename[(name.len() + 1)..];
    let date = chrono::DateTime::parse_from_str(date_string, DATE_FORMAT)
        .log_expect(format!("Failed to read timestamp for {}", &filename))
        .timestamp();
    Ok(date)
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use chrono::NaiveDateTime;

    #[test]
    fn test_time() {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let other_time = NaiveDateTime::from_timestamp(time.try_into().unwrap(), 0);
        println!("{}", time);
        println!("{:?}", other_time);
    }
}
