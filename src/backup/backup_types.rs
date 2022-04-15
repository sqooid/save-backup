use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct BackupState {
    pub last_modified_time: i64,
    pub latest_backup_time: i64,
    pub oldest_backup_path: Option<PathBuf>,
    pub backup_count: u64,
}

impl BackupState {
    pub fn new(
        last_modified_time: i64,
        latest_backup_time: i64,
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
