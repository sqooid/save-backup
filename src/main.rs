use std::{io, path::Path, thread};

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
    pub mod utils;
}

fn main() -> Result<(), io::Error> {
    let configs = config::parse::read_config_from_file("test-1.yaml");
    // for config in configs {
    //     thread::spawn(|| {});
    // }
    Ok(())
}
