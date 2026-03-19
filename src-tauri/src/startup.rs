use crate::models::{ActionResult, StartupDiscovery, StartupItem, StartupItemRequest};
use crate::util::{normalize, powershell, powershell_escape, powershell_json, sanitize_id};
use std::{fs, path::PathBuf};

fn startup_score(name: &str, command: &str, kind: &str) -> u8 {
    let blob = format!("{name} {command}").to_ascii_lowercase();
    let mut score = match kind {
        "scheduled-task" => 55,
        "startup-folder" => 45,
        _ => 35,
    };

    for (needle, weight) in [
        ("wallpaper", 35),
        ("discord", 24),
        ("steam", 28),
        ("riot", 30),
        ("epic", 20),
        ("spotify", 16),
        ("battlenet", 22),
        ("battle.net", 22),
        ("bluestacks", 32),
        ("mumu", 32),
        ("nox", 32),
        ("medal", 18),
        ("notion", 14),
        ("claude", 14),
    ] {
        if blob.contains(needle) {
            score += weight;
        }
    }

    score.min(100)
}

pub fn discover_startup_items() -> Vec<StartupItem> {
    let script = r#"
$items = @()
$regPaths = @(
  @{ Path = 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Run'; Scope = 'machine' },
  @{ Path = 'HKLM:\Software\Microsoft\Windows\CurrentVersion\RunOnce'; Scope = 'machine' },
  @{ Path = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'; Scope = 'user' },
  @{ Path = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\RunOnce'; Scope = 'user' }
)
foreach ($entry in $regPaths) {
  if (Test-Path $entry.Path) {
    $props = Get-ItemProperty -Path $entry.Path
    $props.PSObject.Properties | Where-Object { $_.Name -notlike 'PS*' } | ForEach-Object {
      $items += [pscustomobject]@{
        name = $_.Name; command = [string]$_.Value; location = $entry.Path; kind = 'registry'; scope = $entry.Scope
      }
    }
  }
}
$startupFolders = @(
  @{ Path = "$env:ProgramData\Microsoft\Windows\Start Menu\Programs\Startup"; Scope = 'machine' },
  @{ Path = "$env:AppData\Microsoft\Windows\Start Menu\Programs\Startup"; Scope = 'user' }
)
foreach ($folder in $startupFolders) {
  if (Test-Path $folder.Path) {
    Get-ChildItem -Path $folder.Path -ErrorAction SilentlyContinue | ForEach-Object {
      $items += [pscustomobject]@{
        name = $_.BaseName; command = $_.FullName; location = $folder.Path; kind = 'startup-folder'; scope = $folder.Scope
      }
    }
  }
}
Get-ScheduledTask -ErrorAction SilentlyContinue |
  Where-Object { $_.State -ne 'Disabled' -and $_.TaskPath -notlike '\Microsoft*' } |
  ForEach-Object {
    $action = ''
    if ($_.Actions -and $_.Actions.Count -gt 0) { $action = [string]$_.Actions[0].Execute }
    $items += [pscustomobject]@{
      name = $_.TaskName; command = $action; location = "$($_.TaskPath)|$($_.TaskName)"; kind = 'scheduled-task'; scope = 'machine'
    }
  }
@($items) | ConvertTo-Json -Compress -Depth 4
"#;

    let raw_items: Vec<StartupDiscovery> = powershell_json(script).unwrap_or_default();
    let mut items: Vec<StartupItem> = raw_items
        .into_iter()
        .map(|item| StartupItem {
            id: sanitize_id(&format!("{}-{}-{}", item.kind, item.location, item.name)),
            score: startup_score(&item.name, &item.command, &item.kind),
            name: item.name,
            command: item.command,
            location: item.location,
            kind: item.kind,
            scope: item.scope,
        })
        .collect();

    items.sort_by(|left, right| right.score.cmp(&left.score).then(left.name.cmp(&right.name)));
    items
}

pub fn disable_startup_item(item: StartupItemRequest) -> Result<ActionResult, String> {
    match item.kind.as_str() {
        "registry" => {
            let path = powershell_escape(&item.location);
            let name = powershell_escape(&item.name);
            powershell(&format!(
                "Remove-ItemProperty -Path '{path}' -Name '{name}' -ErrorAction Stop; 'ok'"
            ))?;
        }
        "startup-folder" => {
            let lnk_path = PathBuf::from(&item.location).join(format!("{}.lnk", item.name));
            if lnk_path.exists() {
                fs::remove_file(&lnk_path)
                    .map_err(|err| format!("Failed to remove {}: {err}", normalize(&lnk_path)))?;
            } else {
                let file_path = PathBuf::from(&item.location).join(&item.name);
                if file_path.exists() {
                    fs::remove_file(&file_path).map_err(|err| {
                        format!("Failed to remove {}: {err}", normalize(&file_path))
                    })?;
                } else {
                    return Err(format!("Startup file not found for {}", item.name));
                }
            }
        }
        "scheduled-task" => {
            let parts: Vec<&str> = item.location.split('|').collect();
            if parts.len() != 2 {
                return Err("Scheduled task location payload was invalid.".to_string());
            }
            let task_path = powershell_escape(parts[0]);
            let task_name = powershell_escape(parts[1]);
            powershell(&format!(
                "Disable-ScheduledTask -TaskPath '{task_path}' -TaskName '{task_name}' -ErrorAction Stop | Out-Null; 'ok'"
            ))?;
        }
        _ => return Err(format!("Unsupported startup item kind: {}", item.kind)),
    }

    Ok(ActionResult {
        success: true,
        message: format!("Disabled startup item: {}", item.name),
        details: vec![item.id],
    })
}
