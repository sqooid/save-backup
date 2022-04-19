#![windows_subsystem = "windows"]
use std::thread;

use backup::run::start_backup_loop;
use utils::{process::replace_instance, utils::GenericResult};

mod config {
    pub mod config_types;
    pub mod parse;
}
mod backup {
    pub mod backup_types;
    pub mod file_data;
    pub mod run;
}
mod utils {
    pub mod constants;
    pub mod path;
    pub mod process;
    pub mod utils;
}

fn main() -> GenericResult<()> {
    replace_instance()?;

    let configs = config::parse::read_config_from_file("config.yaml");
    let mut threads = vec![];
    for config in configs {
        let thread = thread::spawn(move || {
            let _ = start_backup_loop(&config);
        });
        threads.push(thread);
    }
    for thread in threads {
        let _ = thread.join();
    }
    Ok(())
}
