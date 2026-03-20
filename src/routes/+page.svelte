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

  async function act<T>(key: string, fn: () => Promise<T>): Promise<T> {
    busy = key;
    try {
      return await fn();
    } finally {
      busy = null;
    }
  }

  async function clearAllLocalCaches() {
    const result = await act<ActionResult>("cache-all", () =>
      clearDirectories(allCachePaths),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function handleApplyPower(mode: "stable" | "performance") {
    const result = await act<ActionResult>(`power-${mode}`, () =>
      applyPowerProfile(mode),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function purgeGlobalMediaCache() {
    const result = await act<ActionResult>("purge-global", () =>
        clearDirectories(snapshot?.globalCaches ?? []),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function handleSetPerformanceMode(install: AfterEffectsInstall, mode: PerformanceMode) {
    if (!install.exePath) return;
    const result = await act<ActionResult>(`perf-${install.id}`, () =>
      setPerformanceMode(install.exePath!, mode),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function handleClearInstall(kind: "cache" | "profile", install: AfterEffectsInstall) {
    const paths = kind === "cache" ? install.cachePaths : install.profilePaths;
    const result = await act<ActionResult>(`clear-${kind}-${install.id}`, () =>
      clearDirectories(paths),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function handleDisableStartup(item: StartupItem) {
    const result = await act<ActionResult>(`startup-${item.id}`, () =>
      disableStartupItem(item),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function handleTogglePlugin(install: AfterEffectsInstall, plugin: PluginEntry, enable: boolean) {
    const result = await act<ActionResult>(`toggle-${plugin.id}`, () =>
      togglePlugin(plugin.path, enable),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function handleInstallScript(install: AfterEffectsInstall, scriptPath: string) {
    const result = await act<ActionResult>(`install-script-${install.id}`, () =>
      installAeScript(install.id, scriptPath),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  function openPath(path: string | null) {
    if (path) revealPath(path);
  }

  async function downConvertProject(path: string, version: string) {
    const result = await act<ActionResult>(`convert-${path}`, () =>
      downConvertAep(path, version),
    );
    flash(result, result.success ? "success" : "error");
  }

  async function purgeProjectAutoSaves(path: string) {
    const result = await act<ActionResult>(`purge-as-${path}`, () =>
      purgeAutoSaves(path),
    );
    flash(result, result.success ? "success" : "error");
    await refreshProjectIndex(
      projectIndex?.scannedMode === "full" ? "full" : "quick",
    );
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

  onMount(() => {
    refresh();
    const interval = setInterval(refreshRenderStatus, 3000);
    const sessionInterval = setInterval(refreshSessionStatus, 5000);
    return () => {
      clearInterval(interval);
      clearInterval(sessionInterval);
    };
  });
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
      onDisable={handleDisableStartup}
    />
    <div class="panel flex flex-col items-center justify-center p-6 text-center">
      <p class="text-xs uppercase tracking-widest text-(--muted)">Status</p>
      {#if loading}
        <div class="mt-2 h-8 w-8 animate-spin rounded-full border-2 border-(--accent)/20 border-t-(--accent)"></div>
      {:else}
        <p class="mt-2 text-2xl font-bold text-(--accent)">Optimal</p>
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

  <ToastBanner {toast} />
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
