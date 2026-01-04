use crate::commands::ProcessInfo;

pub fn get_processes() -> Result<Vec<ProcessInfo>, String> {
    Ok(vec![
        ProcessInfo {
            pid: 1234,
            name: "chrome.exe".to_string(),
            cpu_usage: 5.2,
            memory_usage: 500000000,
        },
        ProcessInfo {
            pid: 5678,
            name: "explorer.exe".to_string(),
            cpu_usage: 2.1,
            memory_usage: 200000000,
        },
    ])
}

pub fn kill_process(_pid: u32) -> Result<(), String> {
    Ok(())
}
