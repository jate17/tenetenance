use sysinfo::{self, System};
use chrono::{DateTime, Utc};


use crate::utils::{conv_bytes_to_mb, conv_bytes_ram};


#[derive(Debug, Clone)]
pub struct ProcessInfo {
    name: String,
    pid: String,
    total_read: f64,
    read: f64,
    total_write: f64,
    write: f64,
    cpu_usage: f32,
    path_process: String, 
    memory: f64,
    virtual_memory: f64,
    parent: String,
    status: String,
    start_time: DateTime<Utc>,
    user_id: String

}

pub fn process_info() -> Vec<ProcessInfo> {
    let mut output: Vec<ProcessInfo> = Vec::new();

    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(100));
    sys.refresh_all();

    for (pid, process) in sys.processes() {
        let disk_usage = process.disk_usage();
        let timestamp = process.start_time() as i64;
        let dt = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap();

        output.push(ProcessInfo {
            name: process.name().to_string_lossy().to_string(),
            pid: pid.to_string(),
            total_read: conv_bytes_to_mb(disk_usage.total_read_bytes),
            read: conv_bytes_to_mb(disk_usage.read_bytes),
            total_write: conv_bytes_to_mb(disk_usage.total_written_bytes),
            write: conv_bytes_to_mb(disk_usage.written_bytes),
            cpu_usage: process.cpu_usage(),
            path_process: process.exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "N/A".to_string()),  // Default se None
            memory: conv_bytes_to_mb(process.memory()),
            virtual_memory: conv_bytes_to_mb(process.virtual_memory()),
            parent: process.parent()
                .map(|p| p.to_string())
                .unwrap_or_else(|| "N/A".to_string()),  // Default se None
            status: process.status().to_string(),
            start_time: dt,
            user_id: process.user_id()
                .map(|u| u.to_string())
                .unwrap_or_else(|| "Unknown".to_string()),  // Default se None
        });
    }

    output
}
