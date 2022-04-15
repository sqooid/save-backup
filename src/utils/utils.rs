use std::{
    error,
    time::{SystemTime, UNIX_EPOCH},
};

pub type GenericResult<T> = Result<T, Box<dyn error::Error>>;

pub fn time_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .try_into()
        .unwrap()
}

pub fn system_time_unix(time: &SystemTime) -> i64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .try_into()
        .unwrap()
}
