// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod system;

use commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // System Information
            get_system_info,
            get_cpu_usage,
            get_memory_usage,
            get_disk_usage,
            
            // Process Management
            get_processes,
            kill_process,
            
            // Startup Programs
            get_startup_programs,
            toggle_startup_program,
            
            // Disk Cleanup
            scan_temp_files,
            clean_temp_files,
            scan_browser_cache,
            clean_browser_cache,
            
            // Registry Cleanup
            scan_registry_issues,
            clean_registry_issues,
            
            // Optimization
            optimize_system,
            defragment_disk,
            
            // Backup & Restore
            create_system_backup,
            restore_system_backup,
            list_backups,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
