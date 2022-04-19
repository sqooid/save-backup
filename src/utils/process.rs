use sysinfo::{PidExt, ProcessExt, System, SystemExt};

pub fn replace_instance() {
    let s = System::new_all();
    for process in s.processes_by_exact_name("save-backup.exe") {
        let pid = std::process::id();
        if process.pid() != sysinfo::Pid::from_u32(pid) {
            process.kill();
        }
    }
}
