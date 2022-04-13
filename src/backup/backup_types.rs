use std::{path::PathBuf, time::SystemTime};

pub struct BackupState {
    last_modified_time: SystemTime,
    latest_backup_time: SystemTime,
    oldest_backup_path: Option<PathBuf>,
}
