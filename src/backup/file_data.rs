use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};

use crate::config::config_types::GameConfig;

use super::backup_types::BackupState;

pub fn get_backup_state(config: &GameConfig) -> Result<BackupState, io::Error> {
    // Check save files
    let mut last_modified_time = SystemTime::UNIX_EPOCH;
    for file in config.file_list.into_iter() {
        let last_modified = file.metadata()?.modified()?;
        if last_modified > last_modified_time {
            last_modified_time = last_modified;
        }
    }

    let mut latest_backup_time = SystemTime::UNIX_EPOCH;
    let mut oldest_backup_path: Option<PathBuf> = None;
    let mut oldest_backup_time = SystemTime::now();
    let mut backup_count: i32 = 0;
    for file in config
        .save_dir
        .read_dir()?
        .filter_map(|x| x.ok())
        .filter(|x| {
            x.to_owned()
                .file_name()
                .as_os_str()
                .to_str()
                .unwrap()
                .to_owned()
                .contains(&config.name)
        })
    {
        let created_time = file.metadata()?.created()?;
        if created_time < oldest_backup_time {
            oldest_backup_path = Some(file.path());
            oldest_backup_time = created_time;
        }
        if created_time > latest_backup_time {
            latest_backup_time = created_time;
        }
        backup_count += 1;
    }
    Ok(BackupState::new(
        last_modified_time,
        latest_backup_time,
        oldest_backup_path,
        backup_count,
    ))
}

#[cfg(test)]
mod test {
    use std::{fs, io, path::PathBuf};

    use crate::{
        backup::backup_types::BackupState,
        config::config_types::{FileList, GameConfig},
    };

    use super::get_backup_state;

    #[test]
    fn test_backup_state() -> Result<(), io::Error> {
        let config = GameConfig {
            name: "file".to_owned(),
            save_dir: PathBuf::from("./test/test_state/save"),
            zip: false,
            file_list: FileList::new("./test/test_state/root", None, None),
            interval: 30,
            count: 10,
        };
        let state = get_backup_state(&config)?;
        let last_modified_time = fs::metadata("test/test_state/root/file2")?.modified()?;
        let latest_backup_time = fs::metadata("test/test_state/save/file2")?.created()?;
        let oldest_backup_path = Some(PathBuf::from(r"./test/test_state/save\file1"));
        assert_eq!(
            state,
            BackupState::new(
                last_modified_time,
                latest_backup_time,
                oldest_backup_path,
                2
            )
        );
        Ok(())
    }
}
