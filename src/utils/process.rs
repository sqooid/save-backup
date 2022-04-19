use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use sysinfo::{ProcessExt, System, SystemExt};
use winapi::um::{
    handleapi::CloseHandle,
    processthreadsapi::{OpenProcess, TerminateProcess},
    psapi::GetModuleFileNameExW,
    winnt::{PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE},
};

use super::utils::GenericResult;

pub fn replace_instance() {
    let s = System::new_all();
    for process in s.processes_by_exact_name("save-backup") {
        process.kill();
    }
}
