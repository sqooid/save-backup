use std::error;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::config_types::GameConfig;
use crate::utils::path::get_backup_time;
use crate::utils::utils::time_now;

use super::backup_types::BackupState;

pub fn get_backup_state(config: &GameConfig) -> Result<BackupState, Box<dyn error::Error>> {
    // Check save files
    let mut last_modified_time = SystemTime::UNIX_EPOCH;
    for file in config.file_list.into_iter() {
        let last_modified = file.metadata()?.modified()?;
        if last_modified > last_modified_time {
            last_modified_time = last_modified;
        }
    }

    let mut latest_backup_time = 0;
    let mut oldest_backup_path: Option<PathBuf> = None;
    let mut oldest_backup_time: i64 = time_now();
    let mut backup_count: u64 = 0;
    for file in config
        .save_dir
        .read_dir()?
        .filter_map(|x| x.ok())
        .filter(|x| {
            x.to_owned()
                .file_name()
                .to_str()
                .unwrap()
                .to_owned()
                .contains(&config.name)
        })
    {
        let filename_ext = file.file_name();
        let mut filename = filename_ext.to_str().unwrap();
        if filename.ends_with(".zip") {
            filename = filename.strip_suffix(".zip").unwrap();
        }
        let created_time = get_backup_time(&config.name, &filename)?;
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
        last_modified_time
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .try_into()
            .unwrap(),
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
        utils::utils::{system_time_unix, GenericResult},
    };

    use super::get_backup_state;

    #[test]
    fn test_backup_state() -> GenericResult<()> {
        let config = GameConfig {
            name: "thing".to_owned(),
            save_dir: PathBuf::from("./test/test_backup/dst"),
            zip: false,
            file_list: FileList::new("./test/test_backup/src", None, None),
            interval: 30,
            count: 10,
        };
        let state = get_backup_state(&config)?;
        let last_modified_time = system_time_unix(
            &fs::metadata("test/test_backup/src/sub/another_file.txt")?.modified()?,
        );
        let latest_backup_time = system_time_unix(
            &fs::metadata("test/test_backup/dst/thing_2022-04-15_21-02-01+1000.zip")?.created()?,
        );
        let oldest_backup_path = Some(PathBuf::from(
            r"./test/test_backup/dst\thing_2022-04-15_21-01-41+1000.zip",
        ));
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
