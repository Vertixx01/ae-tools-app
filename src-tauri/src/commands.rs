use crate::{
    adobe, projects, session, startup, system,
    models::{ActionResult, ScanSnapshot, StartupItemRequest},
};
use std::{fs, path::PathBuf, process::Command};

fn build_snapshot() -> Result<ScanSnapshot, String> {
    let system = system::system_overview()?;
    let installs = adobe::discover_after_effects();
    let startup_items = startup::discover_startup_items();
    let recommendations = system::build_recommendations(&system, &installs, &startup_items);
    let warnings = system::collect_warnings(&installs);

    Ok(ScanSnapshot {
        system,
        installs,
        startup_items,
        recommendations,
        warnings,
    })
}

fn remove_dirs(paths: &[String]) -> ActionResult {
    let mut details = Vec::new();
    let mut removed = 0usize;

    for path in paths {
        let candidate = PathBuf::from(path);
        if !candidate.exists() {
            details.push(format!("Skipped missing path: {path}"));
            continue;
        }

        match fs::remove_dir_all(&candidate) {
            Ok(_) => {
                removed += 1;
                details.push(format!("Removed {path}"));
            }
            Err(err) => details.push(format!("Failed to remove {path}: {err}")),
        }
    }

    ActionResult {
        success: removed > 0,
        message: if removed > 0 {
            format!("Removed {removed} folder(s).")
        } else {
            "No folders were removed.".to_string()
        },
        details,
    }
}

fn active_power_scheme_guid() -> Result<String, String> {
    let output = Command::new("powercfg")
        .arg("/getactivescheme")
        .output()
        .map_err(|err| format!("powercfg failed: {err}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .split_whitespace()
        .find(|token| token.len() == 36 && token.matches('-').count() == 4)
        .map(|token| token.to_string())
        .ok_or_else(|| "Unable to read active power scheme GUID.".to_string())
}

#[tauri::command]
pub fn get_scan_snapshot() -> Result<ScanSnapshot, String> {
    build_snapshot()
}

#[tauri::command]
#[allow(dead_code)]
pub fn get_everything_status() -> projects::EverythingStatus {
    projects::everything_status()
}

#[tauri::command]
#[allow(dead_code)]
pub fn session_status() -> Result<session::SessionStatus, String> {
    Ok(session::get_session_status())
}

#[tauri::command]
#[allow(dead_code)]
pub fn start_session_mode() -> Result<Vec<String>, String> {
    let items = startup::discover_startup_items();
    session::start_session_mode(&items)
}

#[tauri::command]
#[allow(dead_code)]
pub fn stop_session_mode() -> Result<Vec<String>, String> {
    session::stop_session_mode()
}

#[tauri::command]
pub fn get_project_index(mode: String) -> crate::models::ProjectIndexSnapshot {
    projects::get_project_index(mode)
}

#[tauri::command]
pub fn install_everything(package: String) -> Result<ActionResult, String> {
    projects::install_everything(package)
}

#[tauri::command]
pub fn clear_directories(paths: Vec<String>) -> ActionResult {
    remove_dirs(&paths)
}

#[tauri::command]
pub fn set_gpu_preference(exe_path: String) -> Result<ActionResult, String> {
    adobe::set_performance_mode(exe_path, "gpu".to_string())
}

#[tauri::command]
pub fn set_performance_mode(exe_path: String, mode: String) -> Result<ActionResult, String> {
    adobe::set_performance_mode(exe_path, mode)
}

#[tauri::command]
pub fn apply_power_profile(mode: String) -> Result<ActionResult, String> {
    let guid = active_power_scheme_guid()?;
    let max_percent = if mode == "stable" { "99" } else { "100" };

    let output = Command::new("powercfg")
        .args([
            "/setacvalueindex",
            &guid,
            "SUB_PROCESSOR",
            "bc5038f7-23e0-4960-96da-33abaf5935ec",
            max_percent,
        ])
        .output()
        .map_err(|err| format!("Failed to update processor cap: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let activate = Command::new("powercfg")
        .args(["/setactive", &guid])
        .output()
        .map_err(|err| format!("Failed to activate scheme: {err}"))?;

    if !activate.status.success() {
        return Err(String::from_utf8_lossy(&activate.stderr).trim().to_string());
    }

    Ok(ActionResult {
        success: true,
        message: if mode == "stable" {
            "Applied a stability-focused 99% processor cap on AC power.".to_string()
        } else {
            "Restored processor cap to 100% on AC power.".to_string()
        },
        details: vec![format!("Scheme GUID: {guid}")],
    })
}

#[tauri::command]
pub fn disable_startup_item(item: StartupItemRequest) -> Result<ActionResult, String> {
    startup::disable_startup_item(item)
}
