use crate::commands::FileInfo;

pub fn scan_temp_files() -> Result<Vec<FileInfo>, String> {
    Ok(vec![
        FileInfo {
            path: "C:\\Temp\\file1.tmp".to_string(),
            size: 1024000,
        },
    ])
}

pub fn clean_temp_files(_files: Vec<String>) -> Result<u64, String> {
    Ok(1024000)
}

pub fn scan_browser_cache() -> Result<Vec<FileInfo>, String> {
    Ok(vec![
        FileInfo {
            path: "C:\\Cache\\cache1.dat".to_string(),
            size: 2048000,
        },
    ])
}

pub fn clean_browser_cache(_files: Vec<String>) -> Result<u64, String> {
    Ok(2048000)
}
