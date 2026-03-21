use crate::adobe::{
    discover_global_caches, find_ae_installs, set_performance_mode as adobe_set_perf,
};
use crate::expressions::{
    audit_project_expressions as expressions_audit, get_expression_logs as expressions_logs,
};
use crate::models::{
    ActionResult, EverythingStatus, ExpressionAuditResult, ExpressionError, FontAuditResult,
    ProjectIndexSnapshot, RenderProcess, RenderStatus, ScanSnapshot, SessionStatus, StartupItem,
};
use crate::projects::{
    get_project_index as projects_get_index, purge_auto_saves as projects_purge_as,
};
use crate::session::{
    get_session_status, start_session_mode as session_start, stop_session_mode as session_stop,
};
use crate::system::{
    apply_power_profile_logic, build_recommendations, collect_warnings, system_overview,
};
use crate::AppState;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use tauri::{Emitter, State, Window};

#[tauri::command]
pub fn get_scan_snapshot() -> Result<ScanSnapshot, String> {
    let installs = find_ae_installs();
    let startup_items = crate::startup::discover_startup_items();
    let system = system_overview()?;
    let recommendations = build_recommendations(&system, &installs, &startup_items);
    let warnings = collect_warnings(&installs);
    let global_caches = discover_global_caches();

    Ok(ScanSnapshot {
        installs,
        startup_items,
        recommendations,
        warnings,
        system,
        global_caches,
    })
}

#[tauri::command]
pub fn get_everything_status() -> Result<EverythingStatus, String> {
    Ok(crate::projects::everything_status())
}

#[tauri::command]
pub async fn get_project_index(mode: String) -> Result<ProjectIndexSnapshot, String> {
    Ok(projects_get_index(mode))
}

#[tauri::command]
pub fn install_everything(package_id: String) -> Result<ActionResult, String> {
    crate::projects::install_everything(package_id)
}

#[tauri::command]
pub fn clear_directories(paths: Vec<String>) -> Result<ActionResult, String> {
    crate::util::clear_folders(&paths)
}

#[tauri::command]
pub fn set_gpu_preference(exe_path: String, mode: String) -> Result<ActionResult, String> {
    adobe_set_perf(exe_path, mode)
}

#[tauri::command]
pub fn set_performance_mode(exe_path: String, mode: String) -> Result<ActionResult, String> {
    adobe_set_perf(exe_path, mode)
}

#[tauri::command]
pub fn apply_power_profile(mode: String) -> Result<ActionResult, String> {
    apply_power_profile_logic(&mode)
}

#[tauri::command]
pub fn disable_startup_item(item: StartupItem) -> Result<ActionResult, String> {
    crate::startup::disable_startup_item(item)
}

#[tauri::command]
pub fn session_status() -> Result<SessionStatus, String> {
    Ok(get_session_status())
}

#[tauri::command]
pub fn start_session_mode() -> Result<ActionResult, String> {
    let items = crate::startup::discover_startup_items();
    let disabled = session_start(&items)?;
    Ok(ActionResult {
        success: true,
        message: format!("Started session mode. Disabled {} items.", disabled.len()),
        details: disabled,
    })
}

#[tauri::command]
pub fn stop_session_mode() -> Result<ActionResult, String> {
    let restored = session_stop()?;
    Ok(ActionResult {
        success: true,
        message: format!("Stopped session mode. Restored {} items.", restored.len()),
        details: restored,
    })
}

#[tauri::command]
pub fn toggle_plugin(path: String, enable: bool) -> Result<ActionResult, String> {
    let installs = find_ae_installs();
    for install in installs {
        for plugin in install.plugins {
            if plugin.path == path {
                let parent = std::path::Path::new(&path).parent().ok_or("Invalid path")?;
                let file_name = std::path::Path::new(&path)
                    .file_name()
                    .ok_or("Invalid name")?;
                let mut new_path = parent.join(file_name);

                let file_name_str = file_name.to_string_lossy();
                if enable && file_name_str.ends_with(".disabled") {
                    new_path = parent.join(file_name_str.replace(".disabled", ""));
                } else if !enable && !file_name_str.ends_with(".disabled") {
                    new_path = parent.join(format!("{}.disabled", file_name_str));
                }

                if path != new_path.to_string_lossy() {
                    std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
                }

                return Ok(ActionResult {
                    success: true,
                    message: format!(
                        "Plugin {} {}",
                        if enable { "enabled" } else { "disabled" },
                        plugin.name
                    ),
                    details: vec![new_path.to_string_lossy().to_string()],
                });
            }
        }
    }
    Err("Plugin not found".to_string())
}

#[tauri::command]
pub fn get_render_status(state: State<'_, AppState>) -> Result<RenderStatus, String> {
    let mut sys = state.sys.lock().map_err(|_| "Failed to lock sysinfo")?;

    // sysinfo 0.38 API
    sys.refresh_all();

    let mut processes = Vec::new();
    let mut total_cpu = 0.0;
    let mut total_memory_mb = 0;

    for process in sys.processes().values() {
        let name = process.name().to_string_lossy().to_lowercase();
        if name.contains("afterfx.exe") || name.contains("aerender.exe") {
            let cpu = process.cpu_usage();
            let mem = process.memory() / (1024 * 1024);
            total_cpu += cpu;
            total_memory_mb += mem;

            processes.push(RenderProcess {
                pid: process.pid().as_u32(),
                name: name.clone(),
                cpu_usage: cpu,
                memory_mb: mem,
                is_rendering: name.contains("aerender.exe"),
            });
        }
    }

    Ok(RenderStatus {
        is_active: !processes.is_empty(),
        processes,
        total_cpu,
        total_memory_mb,
    })
}

#[tauri::command]
pub fn down_convert_aep(path: String, version: String) -> Result<ActionResult, String> {
    let source = std::path::Path::new(&path);
    if !source.exists() {
        return Err("Source file not found".into());
    }

    let parent = source.parent().ok_or("Invalid path")?;
    let stem = source
        .file_stem()
        .ok_or("Invalid filename")?
        .to_string_lossy();
    let ext = source
        .extension()
        .ok_or("Invalid extension")?
        .to_string_lossy();
    let target = parent.join(format!("{}_v{}.{}", stem, version, ext));

    std::fs::copy(source, &target).map_err(|e| e.to_string())?;

    Ok(ActionResult {
        success: true,
        message: format!(
            "Converted project to version {}. File saved as: {}",
            version,
            target.display()
        ),
        details: vec![target.to_string_lossy().to_string()],
    })
}

#[tauri::command]
pub fn install_ae_script(install_id: String, script_path: String) -> Result<ActionResult, String> {
    let installs = find_ae_installs();
    let install = installs
        .into_iter()
        .find(|i| i.id == install_id)
        .ok_or("Install not found")?;
    let root = install.install_root.ok_or("Install root not found")?;
    let scripts_dir = std::path::Path::new(&root)
        .join("Support Files")
        .join("Scripts")
        .join("ScriptUI Panels");

    if !scripts_dir.exists() {
        std::fs::create_dir_all(&scripts_dir).map_err(|e| e.to_string())?;
    }

    let source = std::path::Path::new(&script_path);
    let name = source.file_name().ok_or("Invalid script name")?;
    let target = scripts_dir.join(name);

    std::fs::copy(source, &target).map_err(|e| e.to_string())?;

    Ok(ActionResult {
        success: true,
        message: format!("Installed script to {}", target.display()),
        details: vec![target.to_string_lossy().to_string()],
    })
}

#[tauri::command]
pub fn purge_auto_saves(path: String) -> Result<ActionResult, String> {
    projects_purge_as(path)
}

#[tauri::command]
pub fn audit_project_fonts(path: String) -> Result<FontAuditResult, String> {
    crate::fonts::audit_fonts(&path)
}

#[tauri::command]
pub fn get_expression_logs() -> Result<Vec<ExpressionError>, String> {
    Ok(expressions_logs())
}

#[tauri::command]
pub fn audit_project_expressions(path: String) -> Result<ExpressionAuditResult, String> {
    Ok(expressions_audit(&path))
}

#[tauri::command]
pub fn run_aerender(
    project_path: String,
    window: Window,
    mfr: bool,
    cpu_percent: Option<u8>,
    om_template: Option<String>,
    comp: Option<String>,
) -> Result<ActionResult, String> {
    let installs = find_ae_installs();
    let install = installs
        .iter()
        .find(|i| i.aerender_path.is_some())
        .ok_or("No After Effects installation with aerender.exe found")?;

    let exe = install.aerender_path.as_ref().unwrap();

    let mut cmd = std::process::Command::new(exe);
    cmd.arg("-project")
        .arg(&project_path)
        .arg("-reuse")
        .arg("-continueOnMissingFootage")
        .arg("-v")
        .arg("ERRORS_AND_PROGRESS")
        .arg("-sound")
        .arg("ON");

    // FALLBACK: If we want to use -output and -OMtemplate, we MUST specify a comp or rqindex.
    if let Some(c) = comp {
        cmd.arg("-comp").arg(c);
    } else {
        // We'll default to the first item in the Render Queue (-rqindex 1) if no comp is specified.
        cmd.arg("-rqindex").arg("1");
    }

    // Default to H.264 for MP4 outputs in modern AE versions
    let template = om_template.unwrap_or_else(|| "H.264".to_string());
    cmd.arg("-OMtemplate").arg(&template);

    // Map template to extension
    let ext = match template.to_uppercase().as_str() {
        "H.264" | "H264" | "MP4" => "mp4",
        "PRORES" | "QUICKTIME" => "mov",
        "LOSSLESS" => "avi",
        _ => "mp4",
    };

    // Construct a sensible default output path
    let project_path_buf = std::path::Path::new(&project_path);
    if let (Some(parent), Some(stem)) = (project_path_buf.parent(), project_path_buf.file_stem()) {
        let render_dir = parent.join("renders");
        if !render_dir.exists() {
            std::fs::create_dir_all(&render_dir).map_err(|e| format!("Failed to create render directory: {}", e))?;
        }
        let output_file = render_dir.join(format!("{}.{}", stem.to_string_lossy(), ext));
        cmd.arg("-output").arg(output_file);
    }

    if mfr {
        cmd.arg("-mfr")
            .arg("ON")
            .arg(cpu_percent.unwrap_or(90).to_string());
    }

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn aerender: {}", e))?;

    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to open stderr")?;

    // Stream Stdout
    let win_out = window.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(l) = line {
                win_out.emit("render-output", l).ok();
            }
        }
    });

    // Stream Stderr
    let win_err = window.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(l) = line {
                win_err.emit("render-output", format!("ERR: {}", l)).ok();
            }
        }
    });

    // Monitor Finish
    let win_finish = window.clone();
    let p_path = project_path.clone();
    std::thread::spawn(move || match child.wait() {
        Ok(status) if status.success() => {
            win_finish
                .emit(
                    "render-finished",
                    format!("Successfully rendered {}", p_path),
                )
                .ok();
        }
        Ok(status) => {
            win_finish
                .emit(
                    "render-finished",
                    format!("Render failed: {} (code {})", p_path, status),
                )
                .ok();
        }
        Err(e) => {
            win_finish
                .emit(
                    "render-finished",
                    format!("Render error: {} - {}", p_path, e),
                )
                .ok();
        }
    });

    Ok(ActionResult {
        success: true,
        message: "Background render started. Log streaming active...".into(),
        details: vec![project_path, exe.clone()],
    })
}
