use crate::system::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub cpu: String,
    pub total_memory: u64,
    pub available_memory: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartupProgram {
    pub name: String,
    pub path: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub date: String,
    pub size: u64,
}

// System Information Commands
#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    system_info::get_system_info()
}

#[tauri::command]
pub fn get_cpu_usage() -> Result<f32, String> {
    system_info::get_cpu_usage()
}

#[tauri::command]
pub fn get_memory_usage() -> Result<f32, String> {
    system_info::get_memory_usage()
}

#[tauri::command]
pub fn get_disk_usage() -> Result<f32, String> {
    system_info::get_disk_usage()
}

// Process Management Commands
#[tauri::command]
pub fn get_processes() -> Result<Vec<ProcessInfo>, String> {
    process::get_processes()
}

#[tauri::command]
pub fn kill_process(pid: u32) -> Result<(), String> {
    process::kill_process(pid)
}

// Startup Programs Commands
#[tauri::command]
pub fn get_startup_programs() -> Result<Vec<StartupProgram>, String> {
    startup::get_startup_programs()
}

#[tauri::command]
pub fn toggle_startup_program(name: String, enabled: bool) -> Result<(), String> {
    startup::toggle_startup_program(&name, enabled)
}

// Disk Cleanup Commands
#[tauri::command]
pub fn scan_temp_files() -> Result<Vec<FileInfo>, String> {
    cleanup::scan_temp_files()
}

#[tauri::command]
pub fn clean_temp_files(files: Vec<String>) -> Result<u64, String> {
    cleanup::clean_temp_files(files)
}

#[tauri::command]
pub fn scan_browser_cache() -> Result<Vec<FileInfo>, String> {
    cleanup::scan_browser_cache()
}

#[tauri::command]
pub fn clean_browser_cache(files: Vec<String>) -> Result<u64, String> {
    cleanup::clean_browser_cache(files)
}

// Registry Cleanup Commands
#[tauri::command]
pub fn scan_registry_issues() -> Result<Vec<String>, String> {
    registry::scan_registry_issues()
}

#[tauri::command]
pub fn clean_registry_issues(issues: Vec<String>) -> Result<u32, String> {
    registry::clean_registry_issues(issues)
}

// Optimization Commands
#[tauri::command]
pub fn optimize_system() -> Result<String, String> {
    optimization::optimize_system()
}

#[tauri::command]
pub fn defragment_disk(drive: String) -> Result<String, String> {
    optimization::defragment_disk(&drive)
}

// Backup & Restore Commands
#[tauri::command]
pub fn create_system_backup(name: String) -> Result<BackupInfo, String> {
    backup::create_system_backup(&name)
}

#[tauri::command]
pub fn restore_system_backup(backup_id: String) -> Result<(), String> {
    backup::restore_system_backup(&backup_id)
}

#[tauri::command]
pub fn list_backups() -> Result<Vec<BackupInfo>, String> {
    backup::list_backups()
}
