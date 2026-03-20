import { invoke } from "@tauri-apps/api/core";
import type {
  ActionResult,
  PerformanceMode,
  ProjectIndexSnapshot,
  ScanSnapshot,
  StartupItem,
  EverythingStatus,
  SessionStatus,
  RenderStatus,
  FontAuditResult,
  ExpressionError,
  ExpressionAuditResult,
} from "./types";

export const getScanSnapshot = () => invoke<ScanSnapshot>("get_scan_snapshot");

export const getProjectIndex = (mode: "quick" | "full") =>
  invoke<ProjectIndexSnapshot>("get_project_index", { mode });

export const getEverythingStatus = () => invoke<EverythingStatus>("get_everything_status");

export const getSessionStatus = () => invoke<SessionStatus>("session_status");

export const startSessionMode = () => invoke<ActionResult>("start_session_mode");

export const stopSessionMode = () => invoke<ActionResult>("stop_session_mode");

export const clearDirectories = (paths: string[]) =>
  invoke<ActionResult>("clear_directories", { paths });

export const setGpuPreference = (exePath: string) =>
  invoke<ActionResult>("set_gpu_preference", { exePath });

export const setPerformanceMode = (exePath: string, mode: PerformanceMode) =>
  invoke<ActionResult>("set_performance_mode", { exePath, mode });

export const applyPowerProfile = (mode: "stable" | "performance") =>
  invoke<ActionResult>("apply_power_profile", { mode });

export const disableStartupItem = (item: StartupItem) =>
  invoke<ActionResult>("disable_startup_item", { item });

export const installEverything = (packageId: string) =>
  invoke<ActionResult>("install_everything", { package: packageId });

export const togglePlugin = (path: string, enable: boolean) =>
  invoke<ActionResult>("toggle_plugin", { path, enable });

export const getRenderStatus = () =>
  invoke<RenderStatus>("get_render_status");

export const downConvertAep = (path: string, targetVersion: string) =>
  invoke<ActionResult>("down_convert_aep", { path, targetVersion });

export const installAeScript = (installId: string, scriptPath: string) =>
  invoke<ActionResult>("install_ae_script", { installId, scriptPath });

export const purgeAutoSaves = (projectPath: string) =>
  invoke<ActionResult>("purge_auto_saves", { projectPath });

export const auditProjectFonts = (path: string) =>
  invoke<FontAuditResult>("audit_project_fonts", { path });

export const getExpressionLogs = () =>
  invoke<ExpressionError[]>("get_expression_logs");

export const auditProjectExpressions = (path: string) =>
  invoke<ExpressionAuditResult>("audit_project_expressions", { path });
