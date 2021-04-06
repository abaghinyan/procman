use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Process {
    pid: u32,
    name: String,
    uid: u32,
    username: String
}

impl Process {
    pub fn new(pid:u32, name: String, uid: u32, username: String) -> Self {
        Process {
            pid,
            name,
            uid,
            username
        }
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid &&
        self.name == other.name &&
        self.uid == other.uid
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Processes {
    pub items: Vec<Process>
}

impl Processes {
    /// Create new array of process
    pub fn new() -> Self {
        Processes {
            items: vec![]
        }
    }

    /// Add a process to processes
    ///
    /// * `process` - Process
    pub fn add(&mut self, process: Process) {
        self.items.push(process)
    }

    /// Get process by pid and return processes
    ///
    /// * `pid` - the pid of process
    pub fn get(&self,pid: u32) -> Processes {
        let mut result: Processes = Processes::new();
        for process in &self.items {
            if process.pid == pid {
                result.add(process.clone())
            }
        }
        result
    }

    /// Get process by username
    ///
    /// * `username` - The username of process creator
    pub fn get_by_username(&self, username: String) -> Processes {
        let mut result: Processes = Processes::new();
        for process in &self.items {
            if process.username.eq(&username)  {
                result.add(process.clone())
            }
        }
        result
    }
}