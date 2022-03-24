use std::fs;
use std::time::SystemTime;

use crate::config::types::GameConfig;

fn get_modified_time(file: &str) -> SystemTime {
    fs::metadata(file)
        .expect("System not supported")
        .modified()
        .expect("System not supported")
}

fn get_created_time(file: &str) -> SystemTime {
    fs::metadata(file)
        .expect("System not supported")
        .created()
        .expect("System not supported")
}

// fn get_latest_backup_time(config: GameConfig) -> Option<SystemTime> {}
