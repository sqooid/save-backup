use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct SharedConfig {
    pub save_root: PathBuf,
    pub zip: bool,
    pub count: i64,
    pub interval: i64,
}

impl SharedConfig {
    pub fn new(
        save_root: Option<&str>,
        zip: Option<bool>,
        count: Option<i64>,
        interval: Option<i64>,
    ) -> SharedConfig {
        SharedConfig {
            save_root: PathBuf::from(save_root.unwrap_or("~/Documents/save-backups")),
            zip: zip.unwrap_or(true),
            count: count.unwrap_or(5),
            interval: interval.unwrap_or(30),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameConfig {
    pub name: String,
    pub save_dir: PathBuf,
    pub zip: bool,
    pub root: PathBuf,
    pub files: Option<Vec<String>>,
    pub interval: i64,
    pub count: i64,
}

impl GameConfig {
    pub fn with_defaults(
        name: &str,
        save_dir: Option<&str>,
        zip: Option<bool>,
        root: &str,
        files: Option<Vec<String>>,
        interval: Option<i64>,
        count: Option<i64>,
        defaults: &SharedConfig,
    ) -> GameConfig {
        let mut save_dir = PathBuf::from(save_dir.unwrap_or(defaults.save_root.to_str().unwrap()));
        save_dir.push(name);

        GameConfig {
            name: name.to_owned(),
            save_dir,
            zip: zip.unwrap_or(defaults.zip),
            root: PathBuf::from(root),
            files,
            interval: interval.unwrap_or(defaults.interval),
            count: count.unwrap_or(defaults.count),
        }
    }
}
