use std::path::PathBuf;

use chrono::{format::Parsed, NaiveDateTime, ParseError};

use super::constants::DATE_FORMAT;

pub fn normalize_paths(paths: Vec<String>) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|x| PathBuf::from(str::replace(x, r"\", "/")))
        .collect()
}

pub fn get_backup_time(name: &str, filename: &str) -> Result<i64, ParseError> {
    let date_string = &filename[(name.len() + 1)..];
    let date = chrono::DateTime::parse_from_str(date_string, DATE_FORMAT)?.timestamp();
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
