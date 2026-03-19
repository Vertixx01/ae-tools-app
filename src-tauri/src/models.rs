use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanSnapshot {
    pub system: SystemOverview,
    pub installs: Vec<AfterEffectsInstall>,
    pub startup_items: Vec<StartupItem>,
    pub recommendations: Vec<Recommendation>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemOverview {
    pub computer_name: String,
    pub os: String,
    pub bios_version: String,
    pub bios_date: String,
    pub motherboard: String,
    pub cpu: String,
    pub gpu: String,
    pub ram_gb: f64,
    pub power_scheme: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub source: String,
    pub kind: String,
    pub size_mb: f64,
    pub has_signature: bool,
    pub duplicate_count: usize,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AfterEffectsInstall {
    pub id: String,
    pub display_name: String,
    pub install_root: Option<String>,
    pub exe_path: Option<String>,
    pub version_hint: String,
    pub profile_paths: Vec<String>,
    pub cache_paths: Vec<String>,
    pub plugin_paths: Vec<String>,
    pub plugins: Vec<PluginEntry>,
    pub is_running: bool,
    pub performance_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StartupItem {
    pub id: String,
    pub name: String,
    pub command: String,
    pub location: String,
    pub kind: String,
    pub scope: String,
    pub score: u8,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    pub id: String,
    pub severity: String,
    pub title: String,
    pub detail: String,
    pub action_label: String,
    pub action_kind: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
    pub details: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupItemRequest {
    pub id: String,
    pub name: String,
    pub location: String,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupDiscovery {
    pub name: String,
    pub command: String,
    pub location: String,
    pub kind: String,
    pub scope: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub extension: String,
    pub modified: String,
    pub size_mb: f64,
    pub drive: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIndexSnapshot {
    pub roots: Vec<String>,
    pub projects: Vec<ProjectEntry>,
    pub total_found: usize,
    pub scanned_mode: String,
    pub engine: String,
    pub skipped_roots: Vec<String>,
}
