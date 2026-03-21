use crate::models::{ActionResult, AfterEffectsInstall, PluginEntry};
use crate::util::{
    env_path, normalize, powershell, powershell_escape, powershell_json, sanitize_id, version_key,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn performance_mode_map() -> BTreeMap<String, String> {
    let script = r#"
$regPath = 'HKCU:\Software\Microsoft\DirectX\UserGpuPreferences'
if (-not (Test-Path $regPath)) { '{}' ; exit 0 }
$props = Get-ItemProperty -Path $regPath
$result = @{}
$props.PSObject.Properties | Where-Object { $_.Name -notlike 'PS*' } | ForEach-Object {
  $value = [string]$_.Value
  $mode = 'balanced'
  if ($value -match 'GpuPreference=2') { $mode = 'gpu' }
  elseif ($value -match 'GpuPreference=1') { $mode = 'cpu' }
  $result[$_.Name] = $mode
}
$result | ConvertTo-Json -Compress
"#;

    powershell_json(script).unwrap_or_default()
}

fn detect_afterfx_running() -> bool {
    Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq AfterFX.exe"])
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .to_ascii_lowercase()
                .contains("afterfx.exe")
        })
        .unwrap_or(false)
}

fn collect_version_dirs(base: &Path) -> BTreeMap<String, Vec<String>> {
    let mut map = BTreeMap::new();
    if !base.exists() {
        return map;
    }

    if let Ok(entries) = fs::read_dir(base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|value| value.to_str()) {
                    map.entry(version_key(name))
                        .or_insert_with(Vec::new)
                        .push(normalize(&path));
                }
            }
        }
    }

    map
}

fn dir_size_mb(path: &Path) -> f64 {
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let child = entry.path();
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    total += (dir_size_mb(&child) * 1024.0 * 1024.0) as u64;
                } else {
                    total += metadata.len();
                }
            }
        }
    }
    ((total as f64) / (1024.0 * 1024.0) * 10.0).round() / 10.0
}

fn check_signature(path: &Path) -> bool {
    let escaped = normalize(path);
    let script = format!(
        "(Get-AuthenticodeSignature -FilePath '{}').Status -eq 'Valid'",
        escaped.replace('\'', "''")
    );

    matches!(powershell(&script).ok(), Some(output) if output.trim().eq_ignore_ascii_case("True"))
}

fn push_plugin_file(target: &mut Vec<PluginEntry>, path: &Path, source: &str, kind: &str) {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return;
    };

    let is_enabled = !name.to_lowercase().ends_with(".disabled");
    let size_mb = if kind == "folder" {
        dir_size_mb(path)
    } else {
        fs::metadata(path)
            .map(|meta| ((meta.len() as f64) / (1024.0 * 1024.0) * 10.0).round() / 10.0)
            .unwrap_or(0.0)
    };

    target.push(PluginEntry {
        id: sanitize_id(&format!("{source}-{name}-{}", normalize(path))),
        name: name.replace(".disabled", ""),
        path: normalize(path),
        source: source.to_string(),
        kind: kind.to_string(),
        size_mb,
        has_signature: check_signature(path) && kind == "binary" && is_enabled,
        is_enabled,
        duplicate_count: 0,
    });
}

fn collect_plugins(plugin_roots: &[PathBuf]) -> Vec<PluginEntry> {
    let mut plugins = Vec::new();
    for root in plugin_roots {
        if !root.exists() {
            continue;
        }

        let root_text = normalize(root).to_ascii_lowercase();
        let source = if root_text.contains("mediacore") {
            "common"
        } else if root_text.contains("appdata") {
            "user"
        } else {
            "version"
        };

        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    push_plugin_file(&mut plugins, &path, source, "folder");
                    continue;
                }

                let ext = path
                    .extension()
                    .and_then(|value| value.to_str())
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                
                if ext == "aex" || ext == "dll" || path.to_string_lossy().to_lowercase().ends_with(".aex.disabled") {
                    push_plugin_file(&mut plugins, &path, source, "binary");
                }
            }
        }
    }

    plugins.sort_by(|left, right| left.name.cmp(&right.name));

    let mut counts = HashMap::new();
    for plugin in &plugins {
        *counts.entry(plugin.name.to_ascii_lowercase()).or_insert(0usize) += 1;
    }
    for plugin in &mut plugins {
        let key = plugin.name.to_ascii_lowercase();
        plugin.duplicate_count = *counts.get(&key).unwrap_or(&0);
    }
    plugins
}

fn plugin_roots_for_install(install_root: &Path) -> Vec<PathBuf> {
    let mut roots = vec![install_root.join("Support Files").join("Plug-ins")];

    if let Some(path) = env_path("ProgramFiles") {
        roots.push(
            path.join("Adobe")
                .join("Common")
                .join("Plug-ins")
                .join("7.0")
                .join("MediaCore"),
        );
    }

    if let Some(path) = env_path("ProgramFiles(x86)") {
        roots.push(
            path.join("Adobe")
                .join("Common")
                .join("Plug-ins")
                .join("7.0")
                .join("MediaCore"),
        );
    }

    if let Some(path) = env_path("APPDATA") {
        roots.push(
            path.join("Adobe")
                .join("Common")
                .join("Plug-ins")
                .join("7.0")
                .join("MediaCore"),
        );
    }

    roots
}

pub fn find_ae_installs() -> Vec<AfterEffectsInstall> {
    let running = detect_afterfx_running();
    let performance_map = performance_mode_map();
    let roaming_versions = env_path("APPDATA")
        .map(|path| path.join("Adobe").join("After Effects"))
        .map(|path| collect_version_dirs(&path))
        .unwrap_or_default();
    let local_versions = env_path("LOCALAPPDATA")
        .map(|path| path.join("Adobe").join("After Effects"))
        .map(|path| collect_version_dirs(&path))
        .unwrap_or_default();

    let mut installs = Vec::new();
    let adobe_roots = [
        env_path("ProgramFiles").map(|path| path.join("Adobe")),
        env_path("ProgramFiles(x86)").map(|path| path.join("Adobe")),
    ];

    for root in adobe_roots.into_iter().flatten() {
        let Ok(entries) = fs::read_dir(&root) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
                continue;
            };

            if !path.is_dir() || !name.to_ascii_lowercase().contains("after effects") {
                continue;
            }

            let exe = path.join("Support Files").join("AfterFX.exe");
            let aerender = path.join("Support Files").join("aerender.exe");
            let key = version_key(name);
            let plugin_roots = plugin_roots_for_install(&path);
            let plugin_paths: Vec<String> = plugin_roots
                .iter()
                .filter(|plugin_root| plugin_root.exists())
                .map(|plugin_root| normalize(plugin_root))
                .collect();

            let exe_normalized = exe.exists().then(|| normalize(&exe));
            let aerender_normalized = aerender.exists().then(|| normalize(&aerender));
            let performance_mode = exe_normalized
                .as_ref()
                .and_then(|value| performance_map.get(value))
                .cloned()
                .unwrap_or_else(|| "balanced".to_string());

            installs.push(AfterEffectsInstall {
                id: sanitize_id(name),
                display_name: name.to_string(),
                install_root: Some(normalize(&path)),
                exe_path: exe_normalized,
                aerender_path: aerender_normalized,
                version_hint: key.clone(),
                profile_paths: roaming_versions.get(&key).cloned().unwrap_or_default(),
                cache_paths: local_versions.get(&key).cloned().unwrap_or_default(),
                plugin_paths,
                plugins: collect_plugins(&plugin_roots),
                is_running: running,
                performance_mode,
            });
        }
    }

    if installs.is_empty() {
        let keys: BTreeSet<String> = roaming_versions
            .keys()
            .chain(local_versions.keys())
            .cloned()
            .collect();

        for key in keys {
            installs.push(AfterEffectsInstall {
                id: format!("profile-{key}"),
                display_name: format!("After Effects profile {key}"),
                install_root: None,
                exe_path: None,
                aerender_path: None,
                version_hint: key.clone(),
                profile_paths: roaming_versions.get(&key).cloned().unwrap_or_default(),
                cache_paths: local_versions.get(&key).cloned().unwrap_or_default(),
                plugin_paths: Vec::new(),
                plugins: Vec::new(),
                is_running: running,
                performance_mode: "balanced".to_string(),
            });
        }
    }

    installs.sort_by(|left, right| right.version_hint.cmp(&left.version_hint));
    installs
}

pub fn set_performance_mode(exe_path: String, mode: String) -> Result<ActionResult, String> {
    let escaped = powershell_escape(&exe_path);
    let script = match mode.as_str() {
        "gpu" => format!(
            "$path = '{escaped}'; if (-not (Test-Path $path)) {{ throw \"Executable not found: $path\" }}; $reg = 'HKCU:\\Software\\Microsoft\\DirectX\\UserGpuPreferences'; if (-not (Test-Path $reg)) {{ New-Item -Path $reg -Force | Out-Null }}; Set-ItemProperty -Path $reg -Name $path -Value 'GpuPreference=2;'; 'ok'"
        ),
        "cpu" => format!(
            "$path = '{escaped}'; if (-not (Test-Path $path)) {{ throw \"Executable not found: $path\" }}; $reg = 'HKCU:\\Software\\Microsoft\\DirectX\\UserGpuPreferences'; if (-not (Test-Path $reg)) {{ New-Item -Path $reg -Force | Out-Null }}; Set-ItemProperty -Path $reg -Name $path -Value 'GpuPreference=1;'; 'ok'"
        ),
        _ => format!(
            "$path = '{escaped}'; $reg = 'HKCU:\\Software\\Microsoft\\DirectX\\UserGpuPreferences'; if (Test-Path $reg) {{ Remove-ItemProperty -Path $reg -Name $path -ErrorAction SilentlyContinue }}; 'ok'"
        ),
    };

    powershell(&script)?;

    Ok(ActionResult {
        success: true,
        message: match mode.as_str() {
            "gpu" => "Set version to GPU-priority mode.".to_string(),
            "cpu" => "Set version to CPU-priority mode.".to_string(),
            _ => "Restored version to balanced mode.".to_string(),
        },
        details: vec![exe_path],
    })
}

pub fn discover_global_caches() -> Vec<String> {
    let mut paths = Vec::new();
    if let Some(appdata) = env_path("APPDATA") {
        let common = appdata.join("Adobe").join("Common");
        let candidates = [
            "Media Cache",
            "Media Cache Files",
            "Peak Files",
            "PTX",
            "Team Projects Cache",
        ];

        for candidate in candidates {
            let path = common.join(candidate);
            if path.exists() {
                paths.push(normalize(&path));
            }
        }
    }
    paths
}
