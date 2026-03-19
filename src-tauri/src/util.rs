use serde::de::DeserializeOwned;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn powershell(script: &str) -> Result<String, String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .output()
        .map_err(|err| format!("Failed to launch PowerShell: {err}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

pub fn powershell_json<T: DeserializeOwned>(script: &str) -> Result<T, String> {
    let stdout = powershell(script)?;
    serde_json::from_str(&stdout).map_err(|err| format!("JSON parse failed: {err}\n{stdout}"))
}

pub fn env_path(name: &str) -> Option<PathBuf> {
    env::var_os(name).map(PathBuf::from)
}

pub fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('/', "\\")
}

pub fn sanitize_id(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

pub fn extract_digits(input: &str) -> Option<String> {
    let digits: String = input.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}

pub fn version_key(label: &str) -> String {
    if let Some(digits) = extract_digits(label) {
        if digits.len() >= 4 {
            let year = &digits[digits.len() - 4..];
            if let Ok(year_num) = year.parse::<u16>() {
                if year_num >= 2010 {
                    return (year_num - 2000).to_string();
                }
            }
        }
        digits
    } else {
        label.to_string()
    }
}

pub fn powershell_escape(input: &str) -> String {
    input.replace('\'', "''")
}
