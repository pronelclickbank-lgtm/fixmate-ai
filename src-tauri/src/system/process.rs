use crate::commands::ProcessInfo;
use sysinfo::{System, ProcessesToUpdate, Pid, Signal};

pub async fn get_processes() -> Result<Vec<ProcessInfo>, String> {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    
    let mut processes: Vec<ProcessInfo> = sys.processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_mb: process.memory() / 1024 / 1024,
        })
        .collect();
    
    // Sort by CPU usage descending
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    
    // Return top 100 processes
    processes.truncate(100);
    
    Ok(processes)
}

pub async fn kill_process(pid: u32) -> Result<bool, String> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    
    let pid = Pid::from_u32(pid);
    
    if let Some(process) = sys.process(pid) {
        if process.kill_with(Signal::Kill).is_some() {
            Ok(true)
        } else {
            Err("Failed to kill process".to_string())
        }
    } else {
        Err("Process not found".to_string())
    }
}
