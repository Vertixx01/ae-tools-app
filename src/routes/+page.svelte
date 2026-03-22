<script lang="ts">
  import { onMount } from "svelte";
  import { openPath as revealPath } from "@tauri-apps/plugin-opener";
  import {
    applyPowerProfile,
    clearDirectories,
    disableStartupItem,
    getEverythingStatus,
    getProjectIndex,
    getScanSnapshot,
    getSessionStatus,
    setPerformanceMode,
    startSessionMode,
    stopSessionMode,
    installEverything,
    togglePlugin,
    getRenderStatus,
    downConvertAep,
    installAeScript,
    purgeAutoSaves,
    auditProjectFonts,
    getExpressionLogs,
    auditProjectExpressions,
    runAerender,
  } from "$lib/api";
  import type {
    ActionResult,
    AfterEffectsInstall,
    EverythingStatus,
    PerformanceMode,
    PluginEntry,
    ProjectIndexSnapshot,
    RenderStatus,
    ScanSnapshot,
    SessionStatus,
    StartupItem,
    FontAuditResult,
    ExpressionError,
    ExpressionAuditResult,
    ProjectEntry,
    RenderOptions,
  } from "$lib/types";
  import HeroSection from "$lib/components/HeroSection.svelte";
  import HealthScoreCard from "$lib/components/HealthScoreCard.svelte";
  import InstallSection from "$lib/components/InstallSection.svelte";
  import ProjectIndexPanel from "$lib/components/ProjectIndexPanel.svelte";
  import FontAuditDialog from "$lib/components/FontAuditDialog.svelte";
  import ExpressionAuditDialog from "$lib/components/ExpressionAuditDialog.svelte";
  import RecommendationsPanel from "$lib/components/RecommendationsPanel.svelte";
  import EverythingStatusPanel from "$lib/components/EverythingStatusPanel.svelte";
  import SessionModePanel from "$lib/components/SessionModePanel.svelte";
  import RenderMonitor from "$lib/components/RenderMonitor.svelte";
  import StartupNoisePanel from "$lib/components/StartupNoisePanel.svelte";
  import SystemProfileCard from "$lib/components/SystemProfileCard.svelte";
  import ToastBanner from "$lib/components/ToastBanner.svelte";
  import RenderLogDialog from "$lib/components/RenderLogDialog.svelte";
  import RenderSettingsDialog from "$lib/components/RenderSettingsDialog.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getAllCachePaths, getHealthScore } from "$lib/dashboard";

  type Toast = { tone: "success" | "error"; message: string };

  let snapshot = $state<ScanSnapshot | null>(null);
  let loading = $state(true);
  let busy = $state<string | null>(null);
  let toast = $state<Toast | null>(null);

  let projectIndex = $state<ProjectIndexSnapshot | null>(null);
  let sessionStatus = $state<SessionStatus | null>(null);
  let renderStatus = $state<RenderStatus | null>(null);
  let everythingStatus = $state<EverythingStatus | null>(null);
  let everythingChecked = $state(false);
  
  let fontAuditResult = $state<FontAuditResult | null>(null);
  let fontAuditLoading = $state(false);

  let expressionAuditResult = $state<ExpressionAuditResult | null>(null);
  let expressionLogs = $state<ExpressionError[]>([]);
  let expressionLoading = $state(false);

  let renderLogs = $state<{ message: string; timestamp: Date }[]>([]);
  let renderLogVisible = $state(false);
  let renderSettingsVisible = $state(false);
  let currentProjectName = $state("");
  let projectForSettings = $state<ProjectEntry | null>(null);

  const healthScore = $derived.by(() => getHealthScore(snapshot));
  const noisyStartupItems = $derived.by(
    () => snapshot?.startupItems.filter((item: StartupItem) => item.score > 50).sort((a: StartupItem, b: StartupItem) => b.score - a.score).slice(0, 3) ?? [],
  );
  const allCachePaths = $derived.by(() => getAllCachePaths(snapshot?.installs ?? []));

  async function refresh() {
    loading = true;
    try {
      snapshot = await getScanSnapshot();
      await refreshProjectIndex("quick");
    } catch (e) {
      flash({ success: false, message: String(e), details: [] }, "error");
    } finally {
      loading = false;
    }
  }

  async function refreshProjectIndex(mode: "quick" | "full") {
    const key = `projects-${mode}`;
    busy = key;
    try {
      projectIndex = await getProjectIndex(mode);
    } catch (e) {
      flash({ success: false, message: String(e), details: [] }, "error");
    } finally {
      busy = null;
    }
  }

  async function refreshRenderStatus() {
    try {
      renderStatus = await getRenderStatus();
    } catch (e) {
      console.error("Failed to refresh render status", e);
    }
  }

  async function refreshSessionStatus() {
    try {
      sessionStatus = await getSessionStatus();
    } catch (e) {
      console.error("Failed to refresh session status", e);
    }
  }

  async function checkEverything() {
    busy = "everything-status";
    try {
      everythingStatus = await getEverythingStatus();
      everythingChecked = true;
    } catch (e) {
      flash({ success: false, message: String(e), details: [] }, "error");
    } finally {
      busy = null;
    }
  }

  async function handleInstallEverything(packageId: string) {
    busy = `everything-install-${packageId}`;
    try {
      const result = await installEverything(packageId);
      flash(result, result.success ? "success" : "error");
      await checkEverything();
    } catch (e) {
      flash({ success: false, message: String(e), details: [] }, "error");
    } finally {
      busy = null;
    }
  }

  function flash(res: ActionResult, tone: "success" | "error") {
    toast = { tone, message: res.message };
    setTimeout(() => {
      if (toast?.message === res.message) toast = null;
    }, 4000);
  }

  async function act<T>(key: string, fn: () => Promise<T>): Promise<T | null> {
    busy = key;
    try {
      const result = await fn();
      return result;
    } catch (e) {
      flash({ success: false, message: String(e), details: [] }, "error");
      return null;
    } finally {
      busy = null;
    }
  }

  async function clearAllLocalCaches() {
    const result = await act<ActionResult>("cache-all", () =>
      clearDirectories(allCachePaths),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refresh();
    }
  }

  async function handleApplyPower(mode: "stable" | "performance") {
    const result = await act<ActionResult>(`power-${mode}`, () =>
      applyPowerProfile(mode),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refresh();
    }
  }

  async function purgeGlobalMediaCache() {
    const result = await act<ActionResult>("purge-global", () =>
        clearDirectories(snapshot?.globalCaches ?? []),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refresh();
    }
  }

  async function handleSetPerformanceMode(install: AfterEffectsInstall, mode: PerformanceMode) {
    if (!install.exePath) return;
    const result = await act<ActionResult>(`perf-${install.id}`, () =>
      setPerformanceMode(install.exePath!, mode),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refresh();
    }
  }

  async function handleClearInstall(kind: "cache" | "profile", install: AfterEffectsInstall) {
    const paths = kind === "cache" ? install.cachePaths : install.profilePaths;
    const result = await act<ActionResult>(`clear-${kind}-${install.id}`, () =>
      clearDirectories(paths),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refresh();
    }
  }

  async function handleDisableStartup(item: StartupItem) {
    const result = await act<ActionResult>(`startup-${item.id}`, () =>
      disableStartupItem(item),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refresh();
    }
  }

  async function handleTogglePlugin(install: AfterEffectsInstall, plugin: PluginEntry, enable: boolean) {
    const result = await act<ActionResult>(`toggle-${plugin.id}`, () =>
      togglePlugin(plugin.path, enable),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refresh();
    }
  }

  async function handleInstallScript(install: AfterEffectsInstall, scriptPath: string) {
    const result = await act<ActionResult>(`install-script-${install.id}`, () =>
      installAeScript(install.id, scriptPath),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refresh();
    }
  }

  function openPath(path: string | null) {
    if (path) revealPath(path);
  }

  async function downConvertProject(path: string, version: string) {
    const result = await act<ActionResult>(`convert-${path}`, () =>
      downConvertAep(path, version),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
    }
  }

  async function purgeProjectAutoSaves(path: string) {
    const result = await act<ActionResult>(`purge-as-${path}`, () =>
      purgeAutoSaves(path),
    );
    if (result) {
      flash(result, result.success ? "success" : "error");
      await refreshProjectIndex(
        projectIndex?.scannedMode === "full" ? "full" : "quick",
      );
    }
  }

  async function performFontAudit(path: string) {
    fontAuditLoading = true;
    try {
      fontAuditResult = await auditProjectFonts(path);
    } catch (e) {
      flash({ success: false, message: String(e), details: [] }, "error");
    } finally {
      fontAuditLoading = false;
    }
  }

  async function performExpressionAudit(path: string) {
    expressionLoading = true;
    try {
      expressionLogs = await getExpressionLogs();
      expressionAuditResult = await auditProjectExpressions(path);
    } catch (e) {
      flash({ success: false, message: String(e), details: [] }, "error");
    } finally {
      expressionLoading = false;
    }
  }

  function performAerender(path: string, name: string) {
    const project = projectIndex?.projects.find(p => p.path === path);
    if (project) {
        projectForSettings = project;
        renderSettingsVisible = true;
    } else {
        flash({ success: false, message: "Project context not found in index.", details: [] }, "error");
    }
  }

  async function handleStartRender(options: RenderOptions) {
    renderSettingsVisible = false;
    renderLogs = [];
    currentProjectName = projectForSettings?.name || "Rendering Project";
    renderLogVisible = true;
    
    try {
      const result = await runAerender(options);
      flash(result, result.success ? "success" : "error");
    } catch (e) {
      flash({ success: false, message: String(e), details: [] }, "error");
    }
  }

  onMount(() => {
    refresh();
    const interval = setInterval(refreshRenderStatus, 3000);
    const sessionInterval = setInterval(refreshSessionStatus, 5000);

    const unlistenOutput = listen<string>("render-output", (event) => {
      renderLogs = [...renderLogs, { message: event.payload, timestamp: new Date() }];
    });

    const unlistenFinished = listen<string>("render-finished", (event) => {
      const isError = event.payload.toLowerCase().includes("failed") || event.payload.toLowerCase().includes("error");
      flash({ success: !isError, message: event.payload, details: [] }, isError ? "error" : "success");
    });

    return () => {
      clearInterval(interval);
      clearInterval(sessionInterval);
      unlistenOutput.then(u => u());
      unlistenFinished.then(u => u());
    };
  });

  function handleProjectPlugins(_path: string, plugins: string[]) {
    const hasPlugins = plugins && plugins.length > 0;
    flash({
      success: true,
      message: hasPlugins ? `Project Plugins: ${plugins.join(", ")}` : "No external plugins detected in this project.",
      details: hasPlugins ? plugins : ["Clean project"]
    }, "success");
  }
</script>

<main class="relative mx-auto max-w-[1700px] overflow-hidden p-6 md:p-8">
  <HeroSection 
    {loading}
    {busy}
    warningMessages={snapshot?.warnings ?? []}
    errorMessage={""}
    canClearCaches={allCachePaths.length > 0}
    globalCacheCount={snapshot?.globalCaches.length ?? 0}
    onRefresh={refresh}
    onClearAllCaches={clearAllLocalCaches}
    onPurgeGlobalCaches={purgeGlobalMediaCache}
    onApplyPower={handleApplyPower}
  />

  <div class="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 items-start">
    <HealthScoreCard 
        {healthScore} 
        recommendationCount={snapshot?.recommendations.length ?? 0} 
    />
    <SystemProfileCard system={snapshot?.system ?? null} />
    <StartupNoisePanel 
      items={noisyStartupItems} 
      totalCount={snapshot?.startupItems.length ?? 0}
      {busy}
      isAdmin={snapshot?.system.isAdmin ?? false}
      onDisable={handleDisableStartup}
    />
    <div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8 flex flex-col items-center justify-center text-center group">
      <div class="absolute inset-0 bg-emerald-500/5 opacity-0 group-hover:opacity-100 transition duration-500 pointer-events-none mix-blend-screen"></div>
      <p class="text-[10px] font-bold uppercase tracking-[0.2em] text-(--muted) mb-3">System Diagnostics</p>
      {#if loading}
        <div class="h-10 w-10 animate-spin rounded-full border-2 border-emerald-500/20 border-t-emerald-500 shadow-[0_0_15px_rgba(16,185,129,0.5)]"></div>
        <p class="mt-4 text-[11px] font-bold uppercase tracking-widest text-white animate-pulse">Running Scans...</p>
      {:else}
        <div class="relative flex items-center justify-center">
          <div class="absolute inset-0 bg-emerald-500 blur-[20px] opacity-20 rounded-full"></div>
          <p class="relative z-10 text-4xl font-black tracking-tighter text-emerald-400 drop-shadow-lg">Optimal</p>
        </div>
        <p class="mt-3 text-[10px] font-medium text-(--muted) max-w-[150px] leading-relaxed">All core subsystems verify nominal operations.</p>
      {/if}
    </div>
  </div>

  <div class="mt-8 grid grid-cols-1 lg:grid-cols-3 gap-6 items-start">
    <!-- Main Column (Actions & Projects) -->
    <div class="lg:col-span-2 space-y-6">
      <InstallSection
        installs={snapshot?.installs ?? []}
        loading={loading}
        {busy}
        onSetPerformanceMode={handleSetPerformanceMode}
        onClearInstall={handleClearInstall}
        onOpenPath={openPath}
        onTogglePlugin={handleTogglePlugin}
        onInstallScript={handleInstallScript}
      />

      {#if projectIndex || loading}
        <ProjectIndexPanel
          {projectIndex}
          {busy}
          onRefresh={refreshProjectIndex}
          onOpenPath={openPath}
          onDownConvert={downConvertProject}
          onPurgeAutoSaves={purgeProjectAutoSaves}
          onAuditFonts={performFontAudit}
          onAuditExpressions={performExpressionAudit}
          onRunAerender={performAerender}
          onOpenPlugins={handleProjectPlugins}
        />
      {/if}
    </div>

    <!-- Side Column (Status & Feedback) -->
    <div class="space-y-6">
      <RenderMonitor status={renderStatus} busy={false} />
      
      <SessionModePanel 
        status={sessionStatus ?? { active: false, startTime: null, disabledItems: [] }}
        {busy}
        onStart={startSessionMode}
        onStop={stopSessionMode}
      />

      <EverythingStatusPanel
        status={everythingStatus ?? { available: false, esPath: "" }}
        {busy}
        checked={everythingChecked}
        onCheck={checkEverything}
        onInstall={handleInstallEverything}
      />

      <RecommendationsPanel
        recommendations={snapshot?.recommendations ?? []}
      />
    </div>
  </div>
</main>

<FontAuditDialog 
  result={fontAuditResult} 
  loading={fontAuditLoading} 
  onClose={() => (fontAuditResult = null)} 
/>

<ExpressionAuditDialog
  result={expressionAuditResult}
  logs={expressionLogs}
  loading={expressionLoading}
  onClose={() => (expressionAuditResult = null)}
/>

<RenderLogDialog
  projectName={currentProjectName}
  logs={renderLogs}
  visible={renderLogVisible}
  onClose={() => (renderLogVisible = false)}
/>

{#if projectForSettings}
  <RenderSettingsDialog
    project={projectForSettings}
    visible={renderSettingsVisible}
    onClose={() => (renderSettingsVisible = false)}
    onStartRender={handleStartRender}
  />
{/if}

<ToastBanner {toast} />
