use std::fs::File;
use std::io::{self, BufReader};

use super::config_types::{GameConfig, SharedConfig};

const SAVE: &str = "save_root";
const ZIP: &str = "zip";
const COUNT: &str = "count";
const ROOT: &str = "root";
const FILES: &str = "files";
const INTERVAL: &str = "interval";

const SHARED_FIELDS: &[&str] = &[SAVE, ZIP, COUNT, INTERVAL];

pub fn read_config_from_file(file: &str) -> Vec<GameConfig> {
    let file = File::open(file).expect("Failed to open config file");
    let mut reader = BufReader::new(file);
    read_config(&mut reader)
}

pub fn read_config<R: io::Read>(reader: &mut R) -> Vec<GameConfig> {
    let config: serde_yaml::Value = serde_yaml::from_reader(reader).expect("Invalid config");

    let shared_config = SharedConfig::new(
        config[SAVE].as_str(),
        config[ZIP].as_bool(),
        config[COUNT].as_i64(),
        config[INTERVAL].as_i64(),
    );

    let mut configs: Vec<GameConfig> = Vec::new();

    for field in config.as_mapping().unwrap().iter() {
        if SHARED_FIELDS.contains(&field.0.as_str().expect("Invalid field in config file")) {
            continue;
        }

        let name = field.0.as_str().expect("Invalid name in config file");

        let save_dir = field.1[SAVE].as_str();

        let zip = field.1[ZIP].as_bool();

        let root = field.1[ROOT].as_str().expect("Invalid root in config file");

        let files_seq = field.1[FILES]
            .as_sequence()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|x| x.as_str().expect("Invalid file in config file").to_string())
            .collect::<Vec<String>>();
        let files = if files_seq.len() > 0 {
            Some(files_seq)
        } else {
            None
        };

        let count = field.1[COUNT].as_i64();

        let interval = field.1[INTERVAL].as_i64();

        let game_config = GameConfig::with_defaults(
            name,
            save_dir,
            zip,
            root,
            files,
            interval,
            count,
            &shared_config,
        );

        configs.push(game_config);
    }

    configs
}

/*
TESTS
*/

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::config_types::GameConfig;

    #[test]
    fn test_config_no_defaults() {
        let config_str = r#"
save_root: ~/save-game-backups
zip: true
interval: 60
count: 10

elden-ring:
  root: "%APPDATA%/EldenRing/12345"
  save_root: ~/Documents/elden-ring-backups/
  files:
    - ER0000.sl2
  interval: 30
  count: 6 
"#;
        let configs = crate::config::parse::read_config(&mut config_str.as_bytes());
        assert_eq!(
            configs,
            vec![GameConfig {
                name: "elden-ring".to_owned(),
                save_dir: PathBuf::from("~/Documents/elden-ring-backups/elden-ring"),
                zip: true,
                root: PathBuf::from("%APPDATA%/EldenRing/12345"),
                files: Some(vec!["ER0000.sl2".to_owned()]),
                interval: 30,
                count: 6
            }]
        );
    }

    #[test]
    fn test_config_all_defaults() {
        let config_str = r#"
save_root: ~/save-game-backups
zip: true
interval: 60
count: 10

elden-ring:
  root: "%APPDATA%/EldenRing/12345"
"#;
        let configs = crate::config::parse::read_config(&mut config_str.as_bytes());
        assert_eq!(
            configs,
            vec![GameConfig {
                name: "elden-ring".to_owned(),
                save_dir: PathBuf::from("~/save-game-backups/elden-ring"),
                zip: true,
                root: PathBuf::from("%APPDATA%/EldenRing/12345"),
                files: None,
                interval: 60,
                count: 10
            }]
        );
    }
}
