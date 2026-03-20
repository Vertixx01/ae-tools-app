use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanSnapshot {
    pub system: SystemOverview,
    pub installs: Vec<AfterEffectsInstall>,
    pub startup_items: Vec<StartupItem>,
    pub recommendations: Vec<Recommendation>,
    pub warnings: Vec<String>,
    pub global_caches: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub source: String,
    pub kind: String,
    pub size_mb: f64,
    pub has_signature: bool,
    pub is_enabled: bool,
    pub duplicate_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    pub id: String,
    pub severity: String,
    pub title: String,
    pub detail: String,
    pub action_label: String,
    pub action_kind: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub success: bool,
    pub message: String,
    pub details: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupItemRequest {
    pub id: String,
    pub name: String,
    pub location: String,
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupDiscovery {
    pub name: String,
    pub command: String,
    pub location: String,
    pub kind: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub extension: String,
    pub modified: String,
    pub size_mb: f64,
    pub drive: String,
    pub auto_save_count: usize,
    pub auto_save_size_mb: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIndexSnapshot {
    pub roots: Vec<String>,
    pub projects: Vec<ProjectEntry>,
    pub total_found: usize,
    pub scanned_mode: String,
    pub engine: String,
    pub skipped_roots: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RenderProcess {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: u64,
    pub is_rendering: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderStatus {
    pub is_active: bool,
    pub processes: Vec<RenderProcess>,
    pub total_cpu: f32,
    pub total_memory_mb: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FontEntry {
    pub name: String,
    pub family: String,
    pub style: Option<String>,
    pub is_installed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontAuditResult {
    pub success: bool,
    pub project_path: String,
    pub fonts: Vec<FontEntry>,
    pub missing_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionError {
    pub timestamp: String,
    pub project: Option<String>,
    pub composition: String,
    pub layer: String,
    pub property: String,
    pub message: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionAuditResult {
    pub success: bool,
    pub project_path: String,
    pub errors: Vec<ExpressionError>,
    pub risky_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EverythingStatus {
    pub available: bool,
    pub es_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStatus {
    pub active: bool,
    pub disabled_items: Vec<String>,
}
