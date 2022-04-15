use std::{path::PathBuf, time::SystemTime};

#[derive(Debug, PartialEq)]
pub struct BackupState {
    pub last_modified_time: SystemTime,
    pub latest_backup_time: SystemTime,
    pub oldest_backup_path: Option<PathBuf>,
    pub backup_count: u64,
}

impl BackupState {
    pub fn new(
        last_modified_time: SystemTime,
        latest_backup_time: SystemTime,
        oldest_backup_path: Option<PathBuf>,
        backup_count: u64,
    ) -> Self {
        Self {
            last_modified_time,
            latest_backup_time,
            oldest_backup_path,
            backup_count,
        }
    }
}
