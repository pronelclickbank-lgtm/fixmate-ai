use crate::commands::StartupProgram;

pub fn get_startup_programs() -> Result<Vec<StartupProgram>, String> {
    Ok(vec![
        StartupProgram {
            name: "Spotify".to_string(),
            path: "C:\\Program Files\\Spotify\\Spotify.exe".to_string(),
            enabled: true,
        },
    ])
}

pub fn toggle_startup_program(_name: &str, _enabled: bool) -> Result<(), String> {
    Ok(())
}
