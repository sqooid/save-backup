use std::path::Path;

mod config {
    pub mod config_types;
    pub mod parse;
}
mod backup {
    pub mod backup_types;
    pub mod file_data;
}

fn main() {
    let configs = config::parse::read_config_from_file("test-1.yaml");
    println!("{:?}", configs);
    let latest = backup::file_data::get_latest_backup_time(Path::new("./test/test_latest/file"));
}
