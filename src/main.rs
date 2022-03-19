use notify_rust::Notification;
use std::{thread, time};
use sysinfo::{ProcessExt, System, SystemExt};

// goal: identifies the processes that takes too much space in memory and notifies the user
fn main() {
    let sys = System::new_all();
    for (pid, process) in sys.processes() {
        println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }
    loop {
        let memory_percentage = sys.used_memory() as f64 / sys.total_memory() as f64;
        let status = format!("NB processors: {}", memory_percentage);
        Notification::new()
            .summary("CPU temp")
            .body(&status)
            .show()
            .unwrap();

        thread::sleep(time::Duration::from_secs(5));
    }
}
