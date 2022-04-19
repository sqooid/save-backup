use std::{
    error::Error,
    fs,
    io::{self, BufReader, Read, Write},
    path::Path,
    thread,
    time::Duration,
};

use zip::{write::FileOptions, ZipWriter};

use crate::{
    config::config_types::GameConfig,
    utils::{
        constants::DATE_FORMAT,
        log::{log, LogExpectResult},
        utils::{time_now, GenericResult},
    },
};

use super::{backup_types::BackupState, file_data::get_backup_state};

pub fn start_backup_loop(config: &GameConfig) -> Result<(), Box<dyn Error>> {
    // Initial check
    fs::create_dir_all(&config.save_dir).log_expect(format!(
        "Failed to create save directory at {}",
        &config.save_dir.to_str().unwrap()
    )); // Required to check for files
    let state = get_backup_state(&config)?;
    let elapsed_minutes = (time_now() - state.latest_backup_time) / 60;
    if elapsed_minutes < config.interval {
        thread::sleep(Duration::from_secs(
            ((config.interval - elapsed_minutes) * 60)
                .try_into()
                .unwrap(),
        ));
    }

    loop {
        let state = get_backup_state(&config)?;

        // Backup required
        if state.last_modified_time > state.latest_backup_time {
            create_backup(&config)?;
        }
        if state.backup_count >= config.count {
            remove_backup(&state)?;
        }
        thread::sleep(Duration::from_secs(
            (config.interval * 60).try_into().unwrap(),
        ));
    }
}

fn path_to_string(path: &Path) -> Result<String, io::Error> {
    Ok(path
        .as_os_str()
        .to_str()
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid file name",
        ))?
        .to_owned())
}

fn remove_backup(state: &BackupState) -> Result<(), io::Error> {
    let extension = state.oldest_backup_path.as_ref().unwrap().extension();
    if extension.is_none() {
        fs::remove_dir_all(state.oldest_backup_path.as_ref().unwrap())?;
    } else if extension.unwrap() == "zip" {
        fs::remove_file(state.oldest_backup_path.as_ref().unwrap())?;
    }

    Ok(())
}

fn create_backup(config: &GameConfig) -> Result<(), Box<dyn Error>> {
    let time_format = chrono::offset::Local::now().format(DATE_FORMAT);
    let backup_string = format!("{}_{}", &config.name, time_format);

    fs::create_dir_all(&config.save_dir)?;

    // Compression on
    if config.zip {
        create_zip_backup(config, &backup_string)?;

    // Compression off
    } else {
        create_folder_backup(config, &backup_string)?;
    }
    log(format!("Created backup for {}", &config.name));
    Ok(())
}

fn create_folder_backup(config: &GameConfig, backup_string: &str) -> GenericResult<()> {
    Ok(for file_path in config.file_list.into_iter() {
        let relative_path = file_path.strip_prefix(&config.file_list.root)?;
        let save_dir = config.save_dir.join(&backup_string);
        let save_path = save_dir.join(&relative_path);

        fs::create_dir_all(save_path.parent().unwrap())?;
        fs::File::create(&save_path)?;
        println!("{:?}", &save_path);
        fs::copy(&file_path, &save_path)?;
    })
}

fn create_zip_backup(config: &GameConfig, backup_string: &str) -> GenericResult<()> {
    let zip_file = fs::File::create(config.save_dir.join(format!("{}.zip", &backup_string)))?;
    let mut writer = ZipWriter::new(zip_file);
    Ok(for file_path in config.file_list.into_iter() {
        let options = FileOptions::default();
        let relative_path = file_path.strip_prefix(&config.file_list.root)?;
        let file_string = path_to_string(relative_path)?;

        writer.start_file(file_string, options)?;
        let file = fs::File::open(file_path)?;
        let buffer = BufReader::new(file);
        let content: Vec<u8> = buffer.bytes().map(|x| x.unwrap()).collect();

        writer.write_all(&content)?;
    })
}

#[cfg(test)]
mod tests {
    use std::{error, path::PathBuf};

    use crate::{
        backup::file_data::get_backup_state,
        config::config_types::{FileList, GameConfig},
    };

    use super::{create_backup, remove_backup};

    #[test]
    fn test_backup() -> Result<(), Box<dyn error::Error>> {
        let config = GameConfig {
            count: 3,
            file_list: FileList::new("test/test_backup/src", None, None),
            interval: 30,
            name: "thing".to_owned(),
            save_dir: PathBuf::from("test/test_backup/dst"),
            zip: true,
        };
        create_backup(&config)?;
        Ok(())
    }

    #[test]
    fn test_remove() -> Result<(), Box<dyn error::Error>> {
        let config = GameConfig {
            count: 3,
            file_list: FileList::new("test/test_backup/src", None, None),
            interval: 30,
            name: "thing".to_owned(),
            save_dir: PathBuf::from("test/test_backup/dst"),
            zip: true,
        };
        let state = get_backup_state(&config)?;
        remove_backup(&state)?;
        Ok(())
    }
}
