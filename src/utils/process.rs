use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use winapi::um::{
    handleapi::CloseHandle,
    processthreadsapi::{OpenProcess, TerminateProcess},
    winnt::PROCESS_TERMINATE,
};

use super::utils::GenericResult;

pub fn replace_instance() -> GenericResult<()> {
    let existing_pid = read_existing_pid();
    if let Ok(Some(pid)) = existing_pid {
        unsafe {
            let process = OpenProcess(PROCESS_TERMINATE, 0, pid);
            TerminateProcess(process, 1);
            CloseHandle(process);
        }
    }
    write_current_pid()?;
    Ok(())
}

fn write_current_pid() -> GenericResult<()> {
    let mut file = File::options().write(true).open("pid")?;
    let pid_string = std::process::id().to_string();
    let pid = pid_string.as_bytes();
    file.write(&pid)?;
    Ok(())
}

fn read_existing_pid() -> GenericResult<Option<u32>> {
    let file = File::open("pid");
    if let Ok(file) = file {
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        let pid: u32 = contents.parse()?;
        return Ok(Some(pid));
    } else {
        return Ok(None);
    }
}
