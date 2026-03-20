use crate::models::{StartupItem, SessionStatus};
use crate::util::{env_path, normalize, powershell, powershell_escape};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct SessionState {
    items: Vec<SessionItemData>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SessionItemData {
    kind: String,
    name: String,
    location: String,
    command: String,
    temp_path: Option<String>,
}


 #[allow(dead_code)]
 pub fn get_session_status() -> SessionStatus {
    let path = session_file();
    if let Some(path) = path {
        if let Ok(file) = fs::read_to_string(&path) {
            if let Ok(state) = serde_json::from_str::<SessionState>(&file) {
                return SessionStatus {
                    active: !state.items.is_empty(),
                    disabled_items: state.items.into_iter().map(|item| item.name).collect(),
                };
            }
        }
    }
    SessionStatus {
        active: false,
        disabled_items: Vec::new(),
    }
}

 #[allow(dead_code)]
 pub fn start_session_mode(startup_items: &[StartupItem]) -> Result<Vec<String>, String> {
    let to_disable: Vec<_> = startup_items
        .iter()
        .filter(|item| item.score >= 70)
        .cloned()
        .collect();

    let mut disabled = Vec::new();
    let mut state = SessionState { items: Vec::new() };

    for item in &to_disable {
        if let Ok(temp) = disable_item(item) {
            state.items.push(temp.clone());
            disabled.push(temp.name);
        }
    }

    if !state.items.is_empty() {
        if let Some(path) = session_file() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(content) = serde_json::to_string(&state) {
                let _ = fs::write(&path, content);
            }
        }
    }

    Ok(disabled)
}

 #[allow(dead_code)]
 pub fn stop_session_mode() -> Result<Vec<String>, String> {
    let path = session_file().ok_or_else(|| "Session file not configured".to_string())?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(&path).map_err(|err| err.to_string())?;
    let state: SessionState = serde_json::from_str(&data).map_err(|err| err.to_string())?;
    let mut restored = Vec::new();

    for item in state.items {
        if restore_item(&item).is_ok() {
            restored.push(item.name);
        }
    }

    let _ = fs::remove_file(&path);
    Ok(restored)
}

fn disable_item(item: &StartupItem) -> Result<SessionItemData, String> {
    match item.kind.as_str() {
        "registry" => {
            let path = powershell_escape(&item.location);
            let name = powershell_escape(&item.name);
            powershell(&format!(
                "Remove-ItemProperty -Path '{path}' -Name '{name}' -ErrorAction Stop; 'ok'"
            ))?;
            Ok(SessionItemData {
                kind: item.kind.clone(),
                name: item.name.clone(),
                location: item.location.clone(),
                command: item.command.clone(),
                temp_path: None,
            })
        }
        "startup-folder" => {
            let original = PathBuf::from(&item.location).join(&item.name);
            let temp = original.with_extension("ae-tools-disabled");
            if original.exists() {
                let _ = fs::rename(&original, &temp);
            }
            Ok(SessionItemData {
                kind: item.kind.clone(),
                name: item.name.clone(),
                location: item.location.clone(),
                command: item.command.clone(),
                temp_path: Some(normalize(&temp)),
            })
        }
        "scheduled-task" => {
            let parts: Vec<&str> = item.location.split('|').collect();
            if parts.len() != 2 {
                return Err("Invalid scheduled task location".to_string());
            }
            let task_path = powershell_escape(parts[0]);
            let task_name = powershell_escape(parts[1]);
            powershell(&format!(
                "Disable-ScheduledTask -TaskPath '{task_path}' -TaskName '{task_name}' -ErrorAction Stop | Out-Null; 'ok'"
            ))?;
            Ok(SessionItemData {
                kind: item.kind.clone(),
                name: item.name.clone(),
                location: item.location.clone(),
                command: item.command.clone(),
                temp_path: None,
            })
        }
        _ => Err("Unsupported startup item kind".to_string()),
    }
}

fn restore_item(item: &SessionItemData) -> Result<(), String> {
    match item.kind.as_str() {
        "registry" => {
            let path = powershell_escape(&item.location);
            let name = powershell_escape(&item.name);
            let command = powershell_escape(&item.command);
            powershell(&format!(
                "Set-ItemProperty -Path '{path}' -Name '{name}' -Value '{command}' -Force; 'ok'"
            ))?;
            Ok(())
        }
        "startup-folder" => {
            if let Some(temp) = &item.temp_path {
                let temp_path = PathBuf::from(temp);
                if temp_path.exists() {
                    let original = PathBuf::from(&item.location).join(&item.name);
                    let _ = fs::rename(&temp_path, &original);
                }
            }
            Ok(())
        }
        "scheduled-task" => {
            let parts: Vec<&str> = item.location.split('|').collect();
            if parts.len() != 2 {
                return Err("Invalid scheduled task location".to_string());
            }
            let task_path = powershell_escape(parts[0]);
            let task_name = powershell_escape(parts[1]);
            powershell(&format!(
                "Enable-ScheduledTask -TaskPath '{task_path}' -TaskName '{task_name}' -ErrorAction Stop | Out-Null; 'ok'"
            ))?;
            Ok(())
        }
        _ => Err("Unsupported startup item kind".to_string()),
    }
}

fn session_file() -> Option<PathBuf> {
    env_path("LOCALAPPDATA").map(|base| base.join("AetherFXOptimizer").join("session.json"))
}
