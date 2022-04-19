use std::{fs::File, io::Write};

use super::utils::GenericResult;

static LOG_FILE: &str = "log.txt";

pub fn reset_log() -> GenericResult<()> {
    let mut file = File::options().create(true).write(true).open(LOG_FILE)?;
    file.write("".as_bytes())?;

    Ok(())
}

pub fn log(text: &str) {
    if let Ok(mut file) = File::options().create(true).append(true).open(LOG_FILE) {
        file.write(text.as_bytes()).unwrap();
    }
}
