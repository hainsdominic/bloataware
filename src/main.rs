use notify_rust::Notification;
use std::{thread, time};
use sysinfo::{ProcessExt, System, SystemExt};

// goal: identifies the processes that takes too much space in memory and notifies the user

#[derive(Debug)]
struct MemoyProcess {
    name: String,
    pid: String,
    total_written_bytes: u64,
}

fn main() {
    let sys = System::new_all();
    let mut memory_processes = vec![];

    for (pid, process) in sys.processes() {
        let new_process = MemoyProcess {
            name: process.name().to_string(),
            pid: pid.to_string(),
            total_written_bytes: process.disk_usage().total_written_bytes,
        };
        memory_processes.push(new_process);
    }

    loop {
        let max = max_memory_processes(&memory_processes);
        Notification::new()
            .summary("Bloat alert!")
            .body(&format!(
                "The suspect is {} with pid [{}], using {} bytes",
                max.name, max.pid, max.total_written_bytes
            ))
            .show()
            .unwrap();

        thread::sleep(time::Duration::from_secs(5));
    }
}

fn max_memory_processes(memory_processes: &Vec<MemoyProcess>) -> &MemoyProcess {
    let mut max_process = memory_processes.first().unwrap();
    for process in memory_processes.iter() {
        if process.total_written_bytes > max_process.total_written_bytes {
            max_process = process;
        }
    }

    return max_process;
}
