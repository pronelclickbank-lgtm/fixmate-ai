use crate::commands::RegistryIssue;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

pub async fn scan_registry_issues() -> Result<Vec<RegistryIssue>, String> {
    #[cfg(target_os = "windows")]
    {
        let mut issues = Vec::new();
        
        // Scan for invalid uninstall entries
        if let Ok(uninstall_key) = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
        {
            for subkey_name in uninstall_key.enum_keys().filter_map(|x| x.ok()) {
                if let Ok(subkey) = uninstall_key.open_subkey(&subkey_name) {
                    // Check if DisplayName exists
                    if subkey.get_value::<String, _>("DisplayName").is_err() {
                        issues.push(RegistryIssue {
                            key_path: format!("HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}", subkey_name),
                            issue_type: "Invalid Uninstall Entry".to_string(),
                            description: "Missing DisplayName value".to_string(),
                        });
                    }
                }
            }
        }
        
        // Scan for invalid file associations
        if let Ok(classes_root) = RegKey::predef(HKEY_CLASSES_ROOT).open_subkey("") {
            for ext_name in classes_root.enum_keys().filter_map(|x| x.ok()).take(50) {
                if ext_name.starts_with('.') {
                    if let Ok(ext_key) = classes_root.open_subkey(&ext_name) {
                        if let Ok(prog_id) = ext_key.get_value::<String, _>("") {
                            // Check if the ProgID exists
                            if classes_root.open_subkey(&prog_id).is_err() {
                                issues.push(RegistryIssue {
                                    key_path: format!("HKCR\\{}", ext_name),
                                    issue_type: "Invalid File Association".to_string(),
                                    description: format!("Points to non-existent ProgID: {}", prog_id),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        Ok(issues)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(vec![])
    }
}

pub async fn clean_registry_issues(keys: Vec<String>) -> Result<usize, String> {
    #[cfg(target_os = "windows")]
    {
        let mut cleaned = 0;
        
        for key_path in keys {
            // Parse the key path (e.g., "HKLM\\SOFTWARE\\...")
            let parts: Vec<&str> = key_path.split('\\').collect();
            if parts.len() < 2 {
                continue;
            }
            
            let hive = match parts[0] {
                "HKLM" => HKEY_LOCAL_MACHINE,
                "HKCU" => HKEY_CURRENT_USER,
                "HKCR" => HKEY_CLASSES_ROOT,
                _ => continue,
            };
            
            let subkey_path = parts[1..parts.len()-1].join("\\");
            let value_name = parts[parts.len()-1];
            
            if let Ok(key) = RegKey::predef(hive).open_subkey_with_flags(&subkey_path, KEY_ALL_ACCESS) {
                if key.delete_subkey(value_name).is_ok() {
                    cleaned += 1;
                }
            }
        }
        
        Ok(cleaned)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(0)
    }
}
