use std::fs;
use std::time::SystemTime;

fn get_last_modified(file: &str) -> SystemTime {
    fs::metadata(file)
}
