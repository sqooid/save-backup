use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedConfig {
    save_dir: String,
    zip: bool,
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    name: String,
    save_dir: String,
    zip: bool,
    root: String,
    files: Vec<String>,
    interval: i64,
}

const SAVE: &str = "save_dir";
const ZIP: &str = "zip";
const ROOT: &str = "root";
const FILES: &str = "files";
const INTERVAL: &str = "interval";

pub fn read_config(file: &str) -> Vec<GameConfig> {
    let f = File::open(file).unwrap();
    let mut reader = BufReader::new(f);
    let config: serde_yaml::Value = serde_yaml::from_reader(&mut reader).expect("Invalid config");

    let shared_config = SharedConfig {
        save_dir: config[SAVE].as_str().unwrap().to_string(),
        zip: config[ZIP].as_bool().unwrap_or(true),
    };

    let mut configs: Vec<GameConfig> = Vec::new();

    for field in config.as_mapping().unwrap().iter() {
        if field.0 == SAVE || field.0 == ZIP {
            continue;
        }

        let name = field
            .0
            .as_str()
            .expect("Invalid name in config file")
            .to_string();
        let save_dir = field.1[SAVE]
            .as_str()
            .unwrap_or(&shared_config.save_dir)
            .to_string();
        let zip = field.1[ZIP].as_bool().unwrap_or(shared_config.zip);
        let root = field.1[ROOT]
            .as_str()
            .expect("Invalid root in config file")
            .to_string();
        let files = field.1[FILES]
            .as_sequence()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|x| x.as_str().expect("Invalid file in config file").to_string())
            .collect::<Vec<String>>();
        let interval = field.1[INTERVAL].as_i64().unwrap_or(60);

        let game_config = GameConfig {
            name: name,
            save_dir: save_dir,
            zip: zip,
            root: root,
            files: files,
            interval: interval,
        };

        configs.push(game_config);
    }

    configs
}
