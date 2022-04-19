use chrono::Local;

use super::utils::GenericResult;
use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Write,
};

static LOG_FILE: &str = "log.txt";

pub fn reset_log() -> GenericResult<()> {
    let file = File::options().create(true).write(true).open(LOG_FILE)?;
    file.set_len(0)?;

    Ok(())
}

pub fn log<T: ToString>(text: T) {
    if let Ok(mut file) = File::options().create(true).append(true).open(LOG_FILE) {
        let string = text.to_string();
        let time_string = Local::now().to_string();
        file.write(time_string.as_bytes()).unwrap();
        file.write(": ".as_bytes()).unwrap();
        file.write(string.as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
}

impl<T, E: Debug> LogExpectResult<T, E> for Result<T, E> {
    fn log_expect<M: Display>(self, message: M) -> T {
        self.map_err(|x| {
            log(message);
            x
        })
        .unwrap()
    }
}

impl<T> LogExpectOption<T> for Option<T> {
    fn log_expect<M: Display>(self, message: M) -> T {
        match self {
            Some(x) => x,
            None => {
                log(message);
                self.unwrap()
            }
        }
    }
}

pub trait LogExpectResult<T, E: Debug> {
    fn log_expect<M: Display>(self, message: M) -> T;
}

pub trait LogExpectOption<T> {
    fn log_expect<M: Display>(self, message: M) -> T;
}
