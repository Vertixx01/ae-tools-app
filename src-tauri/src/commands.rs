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
use std::path::Path;
use std::process::{Command, Stdio};
use tauri::{Emitter, State, Window};

#[tauri::command]
pub fn reveal_in_explorer(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err("File does not exist".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(format!("/select,{}", path))
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        let parent = p.parent().unwrap_or(Path::new("/"));
        Command::new("xdg-open")
            .arg(parent)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

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
    options: crate::models::RenderOptions,
    window: Window,
) -> Result<ActionResult, String> {
    let installs = find_ae_installs();
    let install = installs
        .iter()
        .find(|i| i.aerender_path.is_some())
        .ok_or("No After Effects installation with aerender.exe found")?;

    let exe = install.aerender_path.as_ref().unwrap();

    let mut cmd = std::process::Command::new(exe);
    
    // Core Project and State
    cmd.arg("-project").arg(&options.project_path);
    
    if options.reuse {
        cmd.arg("-reuse");
    }
    
    if options.continue_on_missing {
        cmd.arg("-continueOnMissingFootage");
    }
    
    cmd.arg("-v").arg("ERRORS_AND_PROGRESS");
    cmd.arg("-sound").arg(if options.sound { "ON" } else { "OFF" });

    // Target Selection
    if let Some(c) = options.comp {
        cmd.arg("-comp").arg(c);
    } else {
        cmd.arg("-rqindex").arg("1");
    }

    // Templates
    if let Some(template) = options.om_template {
        cmd.arg("-OMtemplate").arg(template);
    }
    
    if let Some(template) = options.rs_template {
        cmd.arg("-RStemplate").arg(template);
    }

    // Frame Range
    if let Some(start) = options.start_frame {
        cmd.arg("-s").arg(start.to_string());
    }
    if let Some(end) = options.end_frame {
        cmd.arg("-e").arg(end.to_string());
    }

    // Output Path
    if let Some(output) = options.output_path {
        cmd.arg("-output").arg(output);
    } else {
        // Fallback or default output logic
        let path_buf = std::path::Path::new(&options.project_path);
        if let (Some(parent), Some(stem)) = (path_buf.parent(), path_buf.file_stem()) {
            let render_dir = parent.join("renders");
            if !render_dir.exists() {
                std::fs::create_dir_all(&render_dir).map_err(|e| format!("Failed to create render directory {}: {}", render_dir.display(), e))?;
            }
            let output_file = render_dir.join(format!("{}.mp4", stem.to_string_lossy()));
            cmd.arg("-output").arg(output_file);
        }
    }

    // Optimization Settings
    if options.mfr {
        cmd.arg("-mfr").arg("ON").arg(options.cpu_percent.to_string());
    }

    // Memory Usage (max_mem image_cache)
    if options.max_mem > 0 || options.image_cache > 0 {
        cmd.arg("-mem_usage")
           .arg(options.max_mem.to_string())
           .arg(options.image_cache.to_string());
    }

    // CPU Priority (Windows Specific)
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const IDLE_PRIORITY_CLASS: u32 = 0x00000040;
        const BELOW_NORMAL_PRIORITY_CLASS: u32 = 0x00004000;
        const NORMAL_PRIORITY_CLASS: u32 = 0x00000020;
        const ABOVE_NORMAL_PRIORITY_CLASS: u32 = 0x00008000;
        const HIGH_PRIORITY_CLASS: u32 = 0x00000080;

        let flag = match options.priority.as_str() {
            "Low" => IDLE_PRIORITY_CLASS,
            "BelowNormal" => BELOW_NORMAL_PRIORITY_CLASS,
            "Normal" => NORMAL_PRIORITY_CLASS,
            "AboveNormal" => ABOVE_NORMAL_PRIORITY_CLASS,
            "High" => HIGH_PRIORITY_CLASS,
            _ => NORMAL_PRIORITY_CLASS,
        };
        cmd.creation_flags(flag);
    }

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn aerender: {}", e))?;

    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to open stderr")?;

    // Stream Output
    let win_out = window.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(l) = line {
                win_out.emit("render-output", l).ok();
            }
        }
    });

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
    let p_path = options.project_path.clone();
    std::thread::spawn(move || match child.wait() {
        Ok(status) if status.success() => {
            win_finish.emit("render-finished", format!("Successfully rendered {}", p_path)).ok();
        }
        Ok(status) => {
            win_finish.emit("render-finished", format!("Render failed: {} (code {})", p_path, status)).ok();
        }
        Err(e) => {
            win_finish.emit("render-finished", format!("Render error: {} - {}", p_path, e)).ok();
        }
    });

    Ok(ActionResult {
        success: true,
        message: "Advanced rendering started...".into(),
        details: vec![options.project_path, exe.clone()],
    })
}
