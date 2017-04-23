use std::path::Path;
use std::io;
use ps::Process;
use ps::ProcessHandler;

struct Linux;

trait LinuxProcess {
    fn from_stat_file(file_path: &str) -> Result<Process, io::Error>;
}

impl LinuxProcess for Process {
    fn from_stat_file(file_path: &str) -> Result<Process, io::Error> {
        let new_proc: Process = Process { pid: 123, ppid: 1234, executable: "adasd" };
        Ok(new_proc)
    }
}

impl ProcessHandler for Linux {
    fn find_by_pid(pid: i64) -> Option<Process> {
        let proc_dir = format!("/proc/{}", pid);
        let proc_path = Path::new(&proc_dir);

        if proc_path.exists() {
            let stat_file = format!("/proc/{}/stat", pid);

            match Process::from_stat_file(stat_file.as_str()) {
                Ok(process) => Some(process),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    // fn find_by_name(name: str) -> Process{

    // }

    // fn list_processes() -> Vec<Process> {

    // }
}
