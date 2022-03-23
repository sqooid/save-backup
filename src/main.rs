pub mod config;

fn main() {
    let configs = config::read_config("test.yaml");
}
