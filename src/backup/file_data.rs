use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};

use crate::config::config_types::GameConfig;

use super::backup_types::BackupState;

// pub fn get_backup_state(config: &GameConfig) -> Result<BackupState, io::Error> {
//     // Check save files
//     let root_path = PathBuf::from(&config.root);

// }

pub fn get_latest_backup_time(dir: &Path) -> Option<SystemTime> {
    let dir_exists = dir.is_dir();
    if !dir_exists {
        return None;
    }

    let mut latest_time = None;
    for entry in dir.read_dir().expect("Failed to open save directory") {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            if file_name
                .to_str()
                .unwrap_or_default()
                .starts_with(&dir.file_name().unwrap().to_str().unwrap())
            {
                let metadata = entry.metadata();
                if let Ok(metadata) = metadata {
                    let modified = metadata.modified().expect("System not supported");
                    println!("{:?}", modified);
                    if latest_time == None || modified > latest_time.unwrap() {
                        latest_time = Some(modified);
                    }
                }
            }
        }
    }

    latest_time
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::get_latest_backup_time;

    #[test]
    fn test_latest_time() {
        let latest = get_latest_backup_time(Path::new("./test/test_latest/file"));
    }
}
