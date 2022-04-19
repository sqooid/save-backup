use sysinfo::{ProcessExt, System, SystemExt};

use super::log::LogExpectResult;

pub fn replace_instance() {
    let s = System::new_all();
    for process in s.processes_by_exact_name("save-backup.exe") {
        let pid = sysinfo::get_current_pid().log_expect("Unsupported platform");
        if process.pid() != pid {
            process.kill();
        }
    }
}
