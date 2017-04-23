pub struct Process {
    pub pid: i64,
    pub ppid: i64,
    pub executable: &'static str,
}

pub trait ProcessHandler {
    fn find_by_pid(pid: i64) -> Option<Process>;
    // fn find_by_name(name: str) -> Process;
    // fn list_processes() -> Vec<Process>;
}
