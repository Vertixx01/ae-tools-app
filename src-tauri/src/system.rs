use crate::models::{AfterEffectsInstall, Recommendation, StartupItem, SystemOverview};
use crate::util::powershell_json;

pub fn system_overview() -> Result<SystemOverview, String> {
    let script = r#"
$bios = Get-CimInstance Win32_BIOS | Select-Object -First 1
$board = Get-CimInstance Win32_BaseBoard | Select-Object -First 1
$cpu = Get-CimInstance Win32_Processor | Select-Object -First 1
$gpu = Get-CimInstance Win32_VideoController | Sort-Object AdapterRAM -Descending | Select-Object -First 1
$os = Get-CimInstance Win32_OperatingSystem | Select-Object -First 1
$computer = Get-CimInstance Win32_ComputerSystem | Select-Object -First 1
$power = (powercfg /getactivescheme | Out-String).Trim()
$biosDate = [string]$bios.ReleaseDate
try {
  if ($biosDate -match '^\d{14}\.\d{6}[\+\-]\d{3}$') { $biosDate = ([Management.ManagementDateTimeConverter]::ToDateTime($bios.ReleaseDate)).ToString('yyyy-MM-dd') }
  else { $biosDate = ([datetime]$bios.ReleaseDate).ToString('yyyy-MM-dd') }
} catch { $biosDate = [string]$bios.ReleaseDate }
[pscustomobject]@{
  computerName = $env:COMPUTERNAME
  os = "$($os.Caption) ($($os.Version))"
  biosVersion = [string]$bios.SMBIOSBIOSVersion
  biosDate = $biosDate
  motherboard = "$($board.Manufacturer) $($board.Product)"
  cpu = [string]$cpu.Name
  gpu = [string]$gpu.Name
  ramGb = [math]::Round(($computer.TotalPhysicalMemory / 1GB), 1)
  powerScheme = $power
  isAdmin = [bool](([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator"))
} | ConvertTo-Json -Compress
"#;

    powershell_json(script)
}

pub fn build_recommendations(
    system: &SystemOverview,
    installs: &[AfterEffectsInstall],
    startup_items: &[StartupItem],
) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();
    let cpu = system.cpu.to_ascii_lowercase();
    let plugin_count: usize = installs.iter().map(|install| install.plugins.len()).sum();

    if (cpu.contains("i9-13900") || cpu.contains("i7-13700") || cpu.contains("14th gen"))
        && system.bios_date.as_str() < "2024-08-01"
    {
        recommendations.push(Recommendation {
            id: "bios-update".to_string(),
            severity: "high".to_string(),
            title: "Update BIOS before chasing app-level fixes".to_string(),
            detail: format!(
                "{} on {} is old enough to miss later Intel stability microcode.",
                system.bios_version, system.motherboard
            ),
            action_label: "Open board support page manually".to_string(),
            action_kind: "manual".to_string(),
        });
    }

    if system.ram_gb < 32.0 {
        recommendations.push(Recommendation {
            id: "ram-pressure".to_string(),
            severity: "medium".to_string(),
            title: "RAM headroom is tight for modern AE comps".to_string(),
            detail: format!(
                "Detected {:.1} GB. 32 GB+ is a more realistic floor for heavier AE work.",
                system.ram_gb
            ),
            action_label: "Trim load or upgrade RAM".to_string(),
            action_kind: "system".to_string(),
        });
    }

    if installs.is_empty() {
        recommendations.push(Recommendation {
            id: "no-ae".to_string(),
            severity: "medium".to_string(),
            title: "No After Effects install was auto-detected".to_string(),
            detail: "Profiles may still exist, but the app could not find an AfterFX.exe path."
                .to_string(),
            action_label: "Review install folders".to_string(),
            action_kind: "manual".to_string(),
        });
    }

    if let Some(noisy) = startup_items.first() {
        if noisy.score >= 70 {
            recommendations.push(Recommendation {
                id: "startup-noise".to_string(),
                severity: "medium".to_string(),
                title: "Trim the heaviest startup items before long AE sessions".to_string(),
                detail: format!(
                    "{} is enabled at startup and scored {} for likely background churn.",
                    noisy.name, noisy.score
                ),
                action_label: "Disable startup noise".to_string(),
                action_kind: "startup".to_string(),
            });
        }
    }

    if plugin_count > 25 {
        recommendations.push(Recommendation {
            id: "plugin-load".to_string(),
            severity: "medium".to_string(),
            title: "Plugin loadout is large enough to affect stability".to_string(),
            detail: format!(
                "{plugin_count} plugin entries were found across detected installs and shared plugin roots."
            ),
            action_label: "Audit third-party plugins".to_string(),
            action_kind: "plugins".to_string(),
        });
    }

    if installs.iter().any(|install| !install.cache_paths.is_empty()) {
        recommendations.push(Recommendation {
            id: "clear-cache".to_string(),
            severity: "low".to_string(),
            title: "Versioned cache cleanup is available".to_string(),
            detail:
                "Use cache clears first when a specific AE version becomes unstable after crashes or updates."
                    .to_string(),
            action_label: "Clear cache paths".to_string(),
            action_kind: "cleanup".to_string(),
        });
    }

    recommendations
}

pub fn collect_warnings(installs: &[AfterEffectsInstall]) -> Vec<String> {
    let mut warnings = Vec::new();
    if installs.iter().any(|install| install.is_running) {
        warnings.push(
            "After Effects is currently running. Close it before clearing caches, resetting profiles, or changing version-level preferences."
                .to_string(),
        );
    }
    warnings
}

pub fn apply_power_profile_logic(mode: &str) -> Result<crate::models::ActionResult, String> {
    let script = match mode {
        "stable" => "powercfg /setactive SCHEME_BALANCED; powercfg /setacvalueindex SCHEME_BALANCED SUB_PROCESSOR PROCTHROTTLEMAX 99; powercfg /setactive SCHEME_BALANCED; 'ok'",
        "performance" => "powercfg /setactive SCHEME_MIN; powercfg /setacvalueindex SCHEME_MIN SUB_PROCESSOR PROCTHROTTLEMAX 100; powercfg /setactive SCHEME_MIN; 'ok'",
        _ => return Err("Invalid power mode".to_string()),
    };

    crate::util::powershell(script)?;

    Ok(crate::models::ActionResult {
        success: true,
        message: format!("Applied {mode} power profile."),
        details: Vec::new(),
    })
}
