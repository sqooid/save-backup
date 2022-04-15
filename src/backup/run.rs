use std::{
    error::Error,
    fs,
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime},
};

use zip::{write::FileOptions, ZipWriter};

use crate::config::config_types::GameConfig;

use super::{backup_types::BackupState, file_data::get_backup_state};

pub fn start_backup_loop(config: &GameConfig) -> Result<(), Box<dyn Error>> {
    // Initial check
    let state = get_backup_state(config)?;
    let elapsed_minutes = state.latest_backup_time.elapsed()?.as_secs() / 60;
    if elapsed_minutes < config.interval {
        thread::sleep(Duration::from_secs(
            (config.interval - elapsed_minutes) * 60,
        ));
    }

    loop {
        let state = get_backup_state(config)?;

        // Backup required
        if state.last_modified_time > state.latest_backup_time {
            create_backup(config)?;
        }
        if state.backup_count >= config.count {
            remove_backup(&state)?;
        }
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
    let time_format = chrono::offset::Local::now().format(r"%Y-%m-%d_%H-%M-%S");
    let backup_string = format!("{}_{}", &config.name, time_format);

    fs::create_dir_all(&config.save_dir)?;

    // Compression on
    if config.zip {
        let zip_file = fs::File::create(config.save_dir.join(format!("{}.zip", &backup_string)))?;
        let mut writer = ZipWriter::new(zip_file);

        for file_path in config.file_list.into_iter() {
            let options = FileOptions::default();
            let relative_path = file_path.strip_prefix(&config.file_list.root)?;
            let file_string = path_to_string(relative_path)?;

            writer.start_file(file_string, options)?;
            let file = fs::File::open(file_path)?;
            let buffer = BufReader::new(file);
            let content: Vec<u8> = buffer.bytes().map(|x| x.unwrap()).collect();

            writer.write_all(&content)?;
        }

    // Compression off
    } else {
        for file_path in config.file_list.into_iter() {
            let relative_path = file_path.strip_prefix(&config.file_list.root)?;
            let save_dir = config.save_dir.join(&backup_string);
            let save_path = save_dir.join(&relative_path);

            fs::create_dir_all(save_path.parent().unwrap())?;
            fs::File::create(&save_path)?;
            println!("{:?}", &save_path);
            fs::copy(&file_path, &save_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error, path::PathBuf};

    use crate::config::config_types::{FileList, GameConfig};

    use super::create_backup;

    #[test]
    fn test_backup() -> Result<(), Box<dyn error::Error>> {
        let config = GameConfig {
            count: 10,
            file_list: FileList::new("test/test_backup/src", None, None),
            interval: 30,
            name: "thing".to_owned(),
            save_dir: PathBuf::from("test/test_backup/dst"),
            zip: false,
        };
        create_backup(&config)?;
        Ok(())
    }
}
