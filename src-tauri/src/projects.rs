use crate::models::{
    ActionResult, EverythingStatus, ProjectEntry, ProjectIndexSnapshot,
};
use crate::util::{
    env_path, normalize, powershell, powershell_escape, powershell_json, sanitize_id,
};
use std::{
    collections::BTreeSet,
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};


fn discover_auto_saves(project_path: &Path) -> (usize, f64) {
    let Some(parent) = project_path.parent() else {
        return (0, 0.0);
    };
    let auto_save_dir = parent.join("Adobe After Effects Auto-Save");
    if !auto_save_dir.exists() || !auto_save_dir.is_dir() {
        return (0, 0.0);
    }

    let mut count = 0;
    let mut total_size = 0u64;

    if let Ok(entries) = fs::read_dir(auto_save_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                    if ext.to_lowercase() == "aep" {
                        count += 1;
                        if let Ok(meta) = entry.metadata() {
                            total_size += meta.len();
                        }
                    }
                }
            }
        }
    }

    let size_mb = (total_size as f64 / (1024.0 * 1024.0) * 10.0).round() / 10.0;
    (count, size_mb)
}

pub fn purge_auto_saves(project_path: String) -> Result<ActionResult, String> {
    let path = PathBuf::from(&project_path);
    let Some(parent) = path.parent() else {
        return Err("Invalid project path".to_string());
    };
    let auto_save_dir = parent.join("Adobe After Effects Auto-Save");
    if !auto_save_dir.exists() {
        return Ok(ActionResult {
            success: true,
            message: "No auto-save folder found to purge.".to_string(),
            details: Vec::new(),
        });
    }

    let mut purged_count = 0;
    let mut total_freed = 0u64;
    let mut details = Vec::new();

    if let Ok(entries) = fs::read_dir(&auto_save_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                    if ext.to_lowercase() == "aep" {
                        if let Ok(meta) = entry.metadata() {
                            total_freed += meta.len();
                        }
                        if fs::remove_file(&path).is_ok() {
                            purged_count += 1;
                            details.push(normalize(&path));
                        }
                    }
                }
            }
        }
    }

    let freed_mb = (total_freed as f64 / (1024.0 * 1024.0) * 10.0).round() / 10.0;
    Ok(ActionResult {
        success: true,
        message: format!("Purged {} auto-save files, freeing {} MB.", purged_count, freed_mb),
        details,
    })
}

#[allow(dead_code)]
pub fn everything_status() -> EverythingStatus {
    if let Some(path) = locate_es() {
        EverythingStatus {
            available: true,
            es_path: Some(path.to_string_lossy().to_string()),
        }
    } else {
        EverythingStatus {
            available: false,
            es_path: None,
        }
    }
}

#[allow(dead_code)]
enum DownloadKind {
    Installer,
    Zip,
}

#[allow(dead_code)]
struct EverythingDownload {
    id: &'static str,
    url: &'static str,
    file_name: &'static str,
    label: &'static str,
    kind: DownloadKind,
}

#[allow(dead_code)]
const EVERYTHING_DOWNLOADS: &[EverythingDownload] = &[
    EverythingDownload {
        id: "setup-x86",
        url: "https://www.voidtools.com/Everything-1.4.1.1032.x86-Setup.exe",
        file_name: "Everything-1.4.1.1032.x86-Setup.exe",
        label: "Everything x86 installer",
        kind: DownloadKind::Installer,
    },
    EverythingDownload {
        id: "setup-x64",
        url: "https://www.voidtools.com/Everything-1.4.1.1032.x64-Setup.exe",
        file_name: "Everything-1.4.1.1032.x64-Setup.exe",
        label: "Everything x64 installer",
        kind: DownloadKind::Installer,
    },
    EverythingDownload {
        id: "zip-x86",
        url: "https://www.voidtools.com/Everything-1.4.1.1032.x86.zip",
        file_name: "Everything-1.4.1.1032.x86.zip",
        label: "Everything x86 archive",
        kind: DownloadKind::Zip,
    },
    EverythingDownload {
        id: "zip-x64",
        url: "https://www.voidtools.com/Everything-1.4.1.1032.x64.zip",
        file_name: "Everything-1.4.1.1032.x64.zip",
        label: "Everything x64 archive",
        kind: DownloadKind::Zip,
    },
];

#[allow(dead_code)]
fn download_everything_package(download: &EverythingDownload) -> Result<ActionResult, String> {
    let target = env::temp_dir().join(download.file_name);
    let normalized = normalize(&target);
    let url_literal = powershell_escape(download.url);
    let path_literal = powershell_escape(&normalized);

    powershell(&format!(
        "Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing -DisableKeepAlive;",
        url_literal, path_literal
    ))?;

    match download.kind {
        DownloadKind::Installer => {
            powershell(&format!(
                "Start-Process -FilePath '{}' -Verb RunAs -Wait; 'ok'",
                path_literal
            ))?;
            Ok(ActionResult {
                success: true,
                message: format!("Downloaded and launched {}.", download.label),
                details: vec![normalized],
            })
        }
        DownloadKind::Zip => {
            powershell(&format!(
                "Start-Process -FilePath 'explorer.exe' -ArgumentList '/select,{}'; 'ok'",
                path_literal
            ))?;
            Ok(ActionResult {
                success: true,
                message: format!("Downloaded {}. Explorer opened for extraction.", download.label),
                details: vec![normalized],
            })
        }
    }
}

pub fn install_everything(package_id: String) -> Result<ActionResult, String> {
    let download = EVERYTHING_DOWNLOADS
        .iter()
        .find(|option| option.id == package_id)
        .ok_or_else(|| "Invalid Everything package selected.".to_string())?;
    download_everything_package(download)
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EverythingCsvRow {
    name: String,
    path: String,
    size_mb: Option<String>,
    modified: Option<String>,
}

fn discover_project_roots(mode: &str) -> (Vec<PathBuf>, Vec<String>) {
    let mut roots = Vec::new();
    let skipped = Vec::new();

    if mode == "full" {
        for drive in ['C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'] {
            let root = PathBuf::from(format!("{drive}:\\"));
            if root.exists() {
                roots.push(root);
            }
        }
    } else {
        for env_name in ["USERPROFILE", "OneDrive", "PUBLIC"] {
            if let Some(path) = env_path(env_name) {
                roots.push(path);
            }
        }
    }

    let mut seen = BTreeSet::new();
    roots.retain(|root| {
        let key = normalize(root);
        if seen.contains(&key) {
            false
        } else {
            seen.insert(key);
            true
        }
    });

    (roots, skipped)
}

fn locate_es() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(path_value) = env::var("PATH") {
        for part in path_value.split(';').filter(|part| !part.is_empty()) {
            candidates.push(PathBuf::from(part).join("es.exe"));
        }
    }

    if let Some(program_files) = env_path("ProgramFiles") {
        candidates.push(program_files.join("Everything").join("es.exe"));
    }
    if let Some(program_files_x86) = env_path("ProgramFiles(x86)") {
        candidates.push(program_files_x86.join("Everything").join("es.exe"));
    }

    candidates.into_iter().find(|candidate| candidate.exists())
}

fn try_everything_index(limit: usize, mode: &str) -> Option<ProjectIndexSnapshot> {
    let es = locate_es()?;
    let temp_path = env::temp_dir().join(format!("ae-tools-project-index-{mode}.csv"));

    let status = Command::new(&es)
        .args([
            "-regex",
            "\\.(aep|aepx|aet)$",
            "-path-column",
            "-size",
            "-date-modified",
            "-csv",
            "-export-csv",
            &temp_path.to_string_lossy(),
            "-no-header",
            "-date-format",
            "1",
            "-size-format",
            "3",
            "-sort",
            "date-modified-descending",
            "-max-results",
            &limit.to_string(),
        ])
        .status()
        .ok()?;

    if !status.success() || !temp_path.exists() {
        return None;
    }

    let script = format!(
        "Import-Csv -Path '{}' -Header 'name','path','sizeMb','modified' | ConvertTo-Json -Compress",
        temp_path.to_string_lossy().replace('\'', "''")
    );
    let rows: Vec<EverythingCsvRow> = powershell_json(&script).ok()?;

    let mut projects = Vec::new();
    for row in rows {
        let full_path = PathBuf::from(&row.path).join(&row.name);
        let normalized = normalize(&full_path);
        let extension = full_path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        let (auto_save_count, auto_save_size_mb) = discover_auto_saves(&full_path);

        projects.push(ProjectEntry {
            id: sanitize_id(&normalized),
            name: row.name,
            path: normalized.clone(),
            extension,
            modified: row.modified.unwrap_or_default(),
            size_mb: row
                .size_mb
                .and_then(|value| value.parse::<f64>().ok())
                .unwrap_or(0.0),
            drive: normalized.chars().take(2).collect::<String>(),
            auto_save_count,
            auto_save_size_mb,
        });
    }

    let _ = fs::remove_file(&temp_path);

    let total_found = projects.len();

    Some(ProjectIndexSnapshot {
        roots: vec!["Everything index".to_string()],
        projects,
        total_found,
        scanned_mode: mode.to_string(),
        engine: "everything-es".to_string(),
        skipped_roots: Vec::new(),
    })
}

fn is_ignored_dir(path: &Path) -> bool {
    let lowered = normalize(path).to_ascii_lowercase();
    [
        "\\windows",
        "\\program files",
        "\\program files (x86)",
        "\\programdata",
        "\\appdata",
        "\\node_modules",
        "\\$recycle.bin",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

fn fmt_time(time: SystemTime) -> String {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

fn scan_projects_under(root: &Path, projects: &mut Vec<ProjectEntry>, limit: usize) {
    if projects.len() >= limit || is_ignored_dir(root) {
        return;
    }

    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        if projects.len() >= limit {
            break;
        }

        let path = entry.path();
        if path.is_dir() {
            scan_projects_under(&path, projects, limit);
            continue;
        }

        let ext = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !["aep", "aepx", "aet"].contains(&ext.as_str()) {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(value) => value,
            Err(_) => continue,
        };
        let path_text = normalize(&path);
        let (auto_save_count, auto_save_size_mb) = discover_auto_saves(&path);

        projects.push(ProjectEntry {
            id: sanitize_id(&path_text),
            name: path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("Unnamed")
                .to_string(),
            path: path_text.clone(),
            extension: ext,
            modified: metadata
                .modified()
                .map(fmt_time)
                .unwrap_or_else(|_| "0".to_string()),
            size_mb: ((metadata.len() as f64) / (1024.0 * 1024.0) * 10.0).round() / 10.0,
            drive: path_text.chars().take(2).collect::<String>(),
            auto_save_count,
            auto_save_size_mb,
        });
    }
}

pub fn get_project_index(mode: String) -> ProjectIndexSnapshot {
    let scanned_mode = if mode == "full" {
        "full".to_string()
    } else {
        "quick".to_string()
    };
    let limit = if scanned_mode == "full" { 1200 } else { 350 };

    if let Some(indexed) = try_everything_index(limit, &scanned_mode) {
        return indexed;
    }

    let (roots, skipped_roots) = discover_project_roots(&scanned_mode);
    let mut projects = Vec::new();

    for root in &roots {
        if root.exists() {
            scan_projects_under(root, &mut projects, limit);
        }
    }

    projects.sort_by(|left, right| right.modified.cmp(&left.modified));
    let total_found = projects.len();

    ProjectIndexSnapshot {
        roots: roots.iter().map(|root| normalize(root)).collect(),
        total_found,
        projects,
        scanned_mode,
        engine: "filesystem-fallback".to_string(),
        skipped_roots,
    }
}
