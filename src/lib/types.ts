export type PerformanceMode = "balanced" | "gpu" | "cpu";

export interface PluginEntry {
  id: string;
  name: string;
  path: string;
  source: "version" | "common" | "user";
  kind: "binary" | "folder";
  sizeMb: number;
  hasSignature: boolean;
  isEnabled: boolean;
  duplicateCount: number;
}

export interface SystemOverview {
  computerName: string;
  os: string;
  biosVersion: string;
  biosDate: string;
  motherboard: string;
  cpu: string;
  gpu: string;
  ramGb: number;
  powerScheme: string;
}

export interface AfterEffectsInstall {
  id: string;
  displayName: string;
  installRoot: string | null;
  exePath: string | null;
  versionHint: string;
  profilePaths: string[];
  cachePaths: string[];
  pluginPaths: string[];
  plugins: PluginEntry[];
  isRunning: boolean;
  performanceMode: PerformanceMode;
}

export interface StartupItem {
  id: string;
  name: string;
  command: string;
  location: string;
  kind: string;
  scope: string;
  score: number;
}

export interface Recommendation {
  id: string;
  severity: "low" | "medium" | "high" | string;
  title: string;
  detail: string;
  actionLabel: string;
  actionKind: string;
}

export interface EverythingStatus {
  available: boolean;
  esPath: string;
}

export interface ScanSnapshot {
  system: SystemOverview;
  installs: AfterEffectsInstall[];
  startupItems: StartupItem[];
  recommendations: Recommendation[];
  warnings: string[];
  globalCaches: string[];
}

export interface ActionResult {
  success: boolean;
  message: string;
  details: string[];
}

export interface ProjectEntry {
  id: string;
  name: string;
  path: string;
  extension: string;
  modified: string;
  sizeMb: number;
  drive: string;
  autoSaveCount: number;
  autoSaveSizeMb: number;
  width: number | null;
  height: number | null;
  duration: number | null;
  fps: number | null;
  plugins: string[];
  compositions: string[];
  missingFootage: number;
}

export interface ProjectIndexSnapshot {
  roots: string[];
  projects: ProjectEntry[];
  totalFound: number;
  scannedMode: "quick" | "full";
  engine: string;
  skippedRoots: string[];
}

export interface SessionStatus {
  active: boolean;
  startTime: string | null;
  disabledItems: string[];
}

export interface RenderProcess {
  pid: number;
  name: string;
  cpuUsage: number;
  memoryMb: number;
  isRendering: boolean;
}

export interface RenderStatus {
  isActive: boolean;
  processes: RenderProcess[];
  totalCpu: number;
  totalMemoryMb: number;
}

export interface FontEntry {
  name: string;
  family: string;
  style: string | null;
  isInstalled: boolean;
}

export interface FontAuditResult {
  success: boolean;
  projectPath: string;
  fonts: FontEntry[];
  missingCount: number;
}

export interface ExpressionError {
  timestamp: string;
  project?: string;
  composition: string;
  layer: string;
  property: string;
  message: string;
  version: string;
}

export interface ExpressionAuditResult {
  success: boolean;
  projectPath: string;
  errors: ExpressionError[];
  riskyCount: number;
}
