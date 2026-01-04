pub fn scan_registry_issues() -> Result<Vec<String>, String> {
    Ok(vec![
        "HKEY_LOCAL_MACHINE\\Software\\InvalidKey".to_string(),
    ])
}

pub fn clean_registry_issues(_issues: Vec<String>) -> Result<u32, String> {
    Ok(1)
}
