use crate::commands::TempFile;
use std::fs;
use std::path::PathBuf;

pub async fn scan_temp_files() -> Result<Vec<TempFile>, String> {
    let mut temp_files = Vec::new();
    
    // Windows temp directories
    let temp_dirs = vec![
        std::env::var("TEMP").unwrap_or_else(|_| "C:\\Windows\\Temp".to_string()),
        std::env::var("TMP").unwrap_or_else(|_| "C:\\Windows\\Temp".to_string()),
        "C:\\Windows\\Temp".to_string(),
    ];
    
    for temp_dir in temp_dirs {
        if let Ok(entries) = fs::read_dir(&temp_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
                        if size_mb > 0.1 { // Only include files > 100KB
                            temp_files.push(TempFile {
                                path: entry.path().to_string_lossy().to_string(),
                                size_mb,
                                file_type: "Temporary File".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    
    // Sort by size descending
    temp_files.sort_by(|a, b| b.size_mb.partial_cmp(&a.size_mb).unwrap());
    
    // Limit to top 100 files
    temp_files.truncate(100);
    
    Ok(temp_files)
}

pub async fn clean_temp_files(paths: Vec<String>) -> Result<u64, String> {
    let mut total_freed: u64 = 0;
    
    for path in paths {
        if let Ok(metadata) = fs::metadata(&path) {
            let size = metadata.len();
            if fs::remove_file(&path).is_ok() {
                total_freed += size;
            }
        }
    }
    
    Ok(total_freed / 1024 / 1024) // Return MB freed
}

pub async fn scan_browser_cache() -> Result<Vec<TempFile>, String> {
    let mut cache_files = Vec::new();
    
    // Common browser cache locations
    let user_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    
    let cache_dirs = vec![
        format!("{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache", user_profile),
        format!("{}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache", user_profile),
        format!("{}\\AppData\\Local\\Mozilla\\Firefox\\Profiles", user_profile),
    ];
    
    for cache_dir in cache_dirs {
        if let Ok(entries) = fs::read_dir(&cache_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
                        if size_mb > 0.1 {
                            cache_files.push(TempFile {
                                path: entry.path().to_string_lossy().to_string(),
                                size_mb,
                                file_type: "Browser Cache".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    
    cache_files.sort_by(|a, b| b.size_mb.partial_cmp(&a.size_mb).unwrap());
    cache_files.truncate(100);
    
    Ok(cache_files)
}

pub async fn clean_browser_cache() -> Result<u64, String> {
    let cache_files = scan_browser_cache().await?;
    let paths: Vec<String> = cache_files.iter().map(|f| f.path.clone()).collect();
    clean_temp_files(paths).await
}
