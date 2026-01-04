use crate::commands::BackupInfo;

pub fn create_system_backup(_name: &str) -> Result<BackupInfo, String> {
    Ok(BackupInfo {
        id: "backup_001".to_string(),
        date: "2026-01-03".to_string(),
        size: 5000000000,
    })
}

pub fn restore_system_backup(_backup_id: &str) -> Result<(), String> {
    Ok(())
}

pub fn list_backups() -> Result<Vec<BackupInfo>, String> {
    Ok(vec![
        BackupInfo {
            id: "backup_001".to_string(),
            date: "2026-01-03".to_string(),
            size: 5000000000,
        },
    ])
}
