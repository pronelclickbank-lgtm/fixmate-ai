use crate::commands::StartupProgram;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

pub async fn get_startup_programs() -> Result<Vec<StartupProgram>, String> {
    #[cfg(target_os = "windows")]
    {
        let mut programs = Vec::new();
        
        // Check HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
        if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER).open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
            for (name, value) in hkcu.enum_values().filter_map(|x| x.ok()) {
                if let Ok(path) = value.to_string() {
                    programs.push(StartupProgram {
                        name: name.clone(),
                        path: path.clone(),
                        enabled: true,
                        impact: estimate_impact(&path),
                    });
                }
            }
        }
        
        // Check HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Run
        if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
            for (name, value) in hklm.enum_values().filter_map(|x| x.ok()) {
                if let Ok(path) = value.to_string() {
                    // Avoid duplicates
                    if !programs.iter().any(|p| p.name == name) {
                        programs.push(StartupProgram {
                            name: name.clone(),
                            path: path.clone(),
                            enabled: true,
                            impact: estimate_impact(&path),
                        });
                    }
                }
            }
        }
        
        Ok(programs)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Return mock data for non-Windows platforms
        Ok(vec![
            StartupProgram {
                name: "Example Program".to_string(),
                path: "C:\\Program Files\\Example\\app.exe".to_string(),
                enabled: true,
                impact: "Low".to_string(),
            }
        ])
    }
}

pub async fn toggle_startup_program(name: &str, enabled: bool) -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        let run_key = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_ALL_ACCESS)
            .map_err(|e| format!("Failed to open registry key: {}", e))?;
        
        if enabled {
            // Re-enable would require storing the original path, which we don't have
            // In a real implementation, you'd store disabled entries separately
            Err("Re-enabling not implemented - requires storing original path".to_string())
        } else {
            // Disable by deleting the registry value
            run_key.delete_value(name)
                .map_err(|e| format!("Failed to delete registry value: {}", e))?;
            Ok(true)
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(true)
    }
}

fn estimate_impact(path: &str) -> String {
    let path_lower = path.to_lowercase();
    
    // Simple heuristic based on known programs
    if path_lower.contains("steam") || path_lower.contains("discord") || path_lower.contains("spotify") {
        "High".to_string()
    } else if path_lower.contains("microsoft") || path_lower.contains("windows") {
        "Low".to_string()
    } else {
        "Medium".to_string()
    }
}
