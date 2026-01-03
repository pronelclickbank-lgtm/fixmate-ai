use crate::commands::SystemInfo;
use sysinfo::{System, Disks};

pub async fn get_system_info() -> Result<SystemInfo, String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let disks = Disks::new_with_refreshed_list();
    let total_disk: u64 = disks.iter().map(|d| d.total_space()).sum();
    
    Ok(SystemInfo {
        os: System::name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        cpu_name: sys.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_else(|| "Unknown".to_string()),
        cpu_cores: sys.cpus().len(),
        total_memory_gb: sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        total_disk_gb: total_disk as f64 / 1024.0 / 1024.0 / 1024.0,
    })
}

pub async fn get_cpu_usage() -> Result<f32, String> {
    let mut sys = System::new();
    sys.refresh_cpu_all();
    
    // Wait a bit to get accurate CPU usage
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();
    
    let usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    Ok(usage)
}

pub async fn get_memory_usage() -> Result<f32, String> {
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    let used = sys.used_memory() as f64;
    let total = sys.total_memory() as f64;
    let usage = (used / total * 100.0) as f32;
    
    Ok(usage)
}

pub async fn get_disk_usage() -> Result<f32, String> {
    let disks = Disks::new_with_refreshed_list();
    
    let mut total_space: u64 = 0;
    let mut available_space: u64 = 0;
    
    for disk in disks.iter() {
        total_space += disk.total_space();
        available_space += disk.available_space();
    }
    
    if total_space == 0 {
        return Ok(0.0);
    }
    
    let used_space = total_space - available_space;
    let usage = (used_space as f64 / total_space as f64 * 100.0) as f32;
    
    Ok(usage)
}
