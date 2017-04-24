use std::path::Path;
use std::io;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use ps::Process;
use ps::ProcessHandler;
use std::process::Command;

pub struct Linux;

trait LinuxProcess {
    fn from_stat_file(file_path: &str) -> Result<Process, io::Error>;
}

impl LinuxProcess for Process {
    fn from_stat_file(file_path: &str) -> Result<Process, io::Error> {

        let mut file = try!(File::open(file_path));
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();

        try!(reader.read_to_string(&mut buffer));
        
        //  Stat files are column-based, separated by whitespace

        let columns: Vec<_> = buffer.split_whitespace().collect();

        let pid: i64 = columns[0].parse().unwrap_or(-1);
        let ppid: i64 = columns[3].parse().unwrap_or(-1);
        let executable = columns[1];

        let new_proc: Process = Process { pid: pid, ppid: ppid, executable: executable };
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
#[test]
fn find_by_pid_test() {
    // let mut child_process =
    //     Command::new("man")
    //             .arg("ls")
    //             .spawn();
    let sample_proc = Process {
        pid: 123,
        ppid: 1234,
        executable: "adasd",
    };
    assert_eq!(Linux::find_by_pid(4114).unwrap(), sample_proc);
}
