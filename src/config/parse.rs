use std::fs::File;
use std::io::{self, BufReader};

use serde_yaml::Value;

use crate::utils::log::{log, LogExpectOption, LogExpectResult};

use super::config_types::{GameConfig, SharedConfig};

const SAVE: &str = "save_root";
const ZIP: &str = "zip";
const COUNT: &str = "count";
const ROOT: &str = "root";
const INCLUDE: &str = "include";
const EXCLUDE: &str = "exclude";
const INTERVAL: &str = "interval";

const SHARED_FIELDS: &[&str] = &[SAVE, ZIP, COUNT, INTERVAL];

pub fn read_config_from_file(file: &str) -> Vec<GameConfig> {
    let file =
        File::open(file).log_expect("Failed to find config.yaml file in application directory");
    let mut reader = BufReader::new(file);
    read_config(&mut reader)
}

fn collect_string_sequence(sequence: &Value) -> Option<Vec<String>> {
    if let Some(sequence) = sequence.as_sequence() {
        return Some(
            sequence
                .iter()
                .map(|x| {
                    x.as_str()
                        .log_expect("Invalid file in config file")
                        .to_string()
                })
                .collect::<Vec<String>>(),
        );
    } else {
        return None;
    }
}

pub fn read_config<R: io::Read>(reader: &mut R) -> Vec<GameConfig> {
    log("Reading shared config");
    let config: serde_yaml::Value = serde_yaml::from_reader(reader).log_expect("Invalid config");

    let shared_config = SharedConfig::new(
        config[SAVE].as_str(),
        config[ZIP].as_bool(),
        config[COUNT].as_u64(),
        config[INTERVAL].as_i64(),
    );
    log(&shared_config);

    let mut configs: Vec<GameConfig> = Vec::new();

    log("Reading individual configs");
    for field in config.as_mapping().unwrap().iter() {
        if SHARED_FIELDS.contains(&field.0.as_str().log_expect("Invalid field in config file")) {
            continue;
        }

        let name = field.0.as_str().log_expect("Invalid name in config file");

        let save_dir = field.1[SAVE].as_str();

        let zip = field.1[ZIP].as_bool();

        let root = field.1[ROOT].as_str().expect("Invalid root in config file");

        let include = collect_string_sequence(&field.1[INCLUDE]);

        let exclude = collect_string_sequence(&field.1[EXCLUDE]);

        let count = field.1[COUNT].as_u64();

        let interval = field.1[INTERVAL].as_i64();

        let game_config = GameConfig::with_defaults(
            name,
            save_dir,
            zip,
            root,
            include,
            exclude,
            interval,
            count,
            &shared_config,
        );
        log(&game_config);
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

    use crate::config::config_types::{FileList, GameConfig};

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
  include:
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
                file_list: FileList::new(
                    "%APPDATA%/EldenRing/12345",
                    Some(vec!["ER0000.sl2".to_owned()]),
                    None
                ),
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
                file_list: FileList::new("%APPDATA%/EldenRing/12345", None, None),
                interval: 60,
                count: 10
            }]
        );
    }
}
