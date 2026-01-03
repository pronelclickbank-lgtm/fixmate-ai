use serde::{Deserialize, Serialize};
use crate::system::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub total_memory_gb: f64,
    pub total_disk_gb: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartupProgram {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub impact: String, // "Low", "Medium", "High"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempFile {
    pub path: String,
    pub size_mb: f64,
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryIssue {
    pub key_path: String,
    pub issue_type: String,
    pub description: String,
}

// System Information Commands
#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    system_info::get_system_info().await
}

#[tauri::command]
pub async fn get_cpu_usage() -> Result<f32, String> {
    system_info::get_cpu_usage().await
}

#[tauri::command]
pub async fn get_memory_usage() -> Result<f32, String> {
    system_info::get_memory_usage().await
}

#[tauri::command]
pub async fn get_disk_usage() -> Result<f32, String> {
    system_info::get_disk_usage().await
}

// Process Management Commands
#[tauri::command]
pub async fn get_processes() -> Result<Vec<ProcessInfo>, String> {
    process::get_processes().await
}

#[tauri::command]
pub async fn kill_process(pid: u32) -> Result<bool, String> {
    process::kill_process(pid).await
}

// Startup Programs Commands
#[tauri::command]
pub async fn get_startup_programs() -> Result<Vec<StartupProgram>, String> {
    startup::get_startup_programs().await
}

#[tauri::command]
pub async fn toggle_startup_program(name: String, enabled: bool) -> Result<bool, String> {
    startup::toggle_startup_program(&name, enabled).await
}

// Disk Cleanup Commands
#[tauri::command]
pub async fn scan_temp_files() -> Result<Vec<TempFile>, String> {
    cleanup::scan_temp_files().await
}

#[tauri::command]
pub async fn clean_temp_files(paths: Vec<String>) -> Result<u64, String> {
    cleanup::clean_temp_files(paths).await
}

#[tauri::command]
pub async fn scan_browser_cache() -> Result<Vec<TempFile>, String> {
    cleanup::scan_browser_cache().await
}

#[tauri::command]
pub async fn clean_browser_cache() -> Result<u64, String> {
    cleanup::clean_browser_cache().await
}

// Registry Cleanup Commands
#[tauri::command]
pub async fn scan_registry_issues() -> Result<Vec<RegistryIssue>, String> {
    registry::scan_registry_issues().await
}

#[tauri::command]
pub async fn clean_registry_issues(keys: Vec<String>) -> Result<usize, String> {
    registry::clean_registry_issues(keys).await
}

// Optimization Commands
#[tauri::command]
pub async fn optimize_system() -> Result<String, String> {
    optimization::optimize_system().await
}

#[tauri::command]
pub async fn defragment_disk(drive: String) -> Result<bool, String> {
    optimization::defragment_disk(&drive).await
}

// Backup & Restore Commands
#[tauri::command]
pub async fn create_system_backup(name: String) -> Result<String, String> {
    backup::create_system_backup(&name).await
}

#[tauri::command]
pub async fn restore_system_backup(backup_id: String) -> Result<bool, String> {
    backup::restore_system_backup(&backup_id).await
}

#[tauri::command]
pub async fn list_backups() -> Result<Vec<String>, String> {
    backup::list_backups().await
}
