use crate::commands::SystemInfo;

pub fn get_system_info( ) -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        os: std::env::consts::OS.to_string(),
        cpu: "Unknown CPU".to_string(),
        total_memory: 16000000000,
        available_memory: 8000000000,
    })
}

pub fn get_cpu_usage() -> Result<f32, String> {
    Ok(25.5)
}

pub fn get_memory_usage() -> Result<f32, String> {
    Ok(50.0)
}

pub fn get_disk_usage() -> Result<f32, String> {
    Ok(60.0)
}
