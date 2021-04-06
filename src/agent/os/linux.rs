use sysinfo::{SystemExt, ProcessExt, UserExt};
use serde::Serialize;

use crate::agent::models::process::{Process, Processes};
use crate::agent::models::user::{Users, User};

#[derive(Debug, Serialize)]
pub struct Linux {
    pub processes: Processes
}

impl Linux {
    /// Create a linux os
    pub fn new() -> Self {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        let mut users = Users::new();
        for user in system.get_users() {
            users.add(User {
                uid: *user.get_uid(),
                name: user.get_name().to_string()
            });
        }
        let mut processes: Processes = Processes::new();

        for (_pid, proc_) in system.get_processes() {
            let user = users.get(proc_.uid).unwrap();
            processes.add(Process::new(
                proc_.pid() as u32,
                proc_.name().to_string(),
                proc_.uid,
                user.name.to_string()
            ));//serde_json::from_str(proc_)?;
        }

        Linux {
            processes
        }
    }
}
