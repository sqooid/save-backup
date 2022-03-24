pub mod backup;
mod config {
    pub mod parse;
    pub mod types;
}
pub mod file_data;

fn main() {
    let configs = config::parse::read_config_from_file("test-1.yaml");
    println!("{:?}", configs);
}
