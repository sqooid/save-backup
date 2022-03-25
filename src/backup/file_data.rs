use std::fs;
use std::path::Path;
use std::time::SystemTime;

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
