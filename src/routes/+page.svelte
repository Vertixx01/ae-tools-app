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
  } from "$lib/api";
  import HeroSection from "$lib/components/HeroSection.svelte";
  import HealthScoreCard from "$lib/components/HealthScoreCard.svelte";
  import InstallSection from "$lib/components/InstallSection.svelte";
  import ProjectIndexPanel from "$lib/components/ProjectIndexPanel.svelte";
  import RecommendationsPanel from "$lib/components/RecommendationsPanel.svelte";
  import EverythingStatusPanel from "$lib/components/EverythingStatusPanel.svelte";
  import SessionModePanel from "$lib/components/SessionModePanel.svelte";
  import StartupNoisePanel from "$lib/components/StartupNoisePanel.svelte";
  import SystemProfileCard from "$lib/components/SystemProfileCard.svelte";
  import ToastBanner from "$lib/components/ToastBanner.svelte";
  import { getAllCachePaths, getHealthScore } from "$lib/dashboard";
  import type {
    ActionResult,
    AfterEffectsInstall,
    EverythingStatus,
    PerformanceMode,
    ProjectIndexSnapshot,
    ScanSnapshot,
    SessionStatus,
    StartupItem,
  } from "$lib/types";

  type Toast = { tone: "success" | "error"; message: string };

  let snapshot = $state<ScanSnapshot | null>(null);
  let loading = $state(true);
  let busy = $state<string | null>(null);
  let errorMessage = $state("");
  let toast = $state<Toast | null>(null);
  let projectIndex = $state<ProjectIndexSnapshot | null>(null);
  let everythingStatus = $state<EverythingStatus | null>(null);
  let sessionStatus = $state<SessionStatus | null>(null);
  let everythingChecked = $state(false);

  const healthScore = $derived.by(() => getHealthScore(snapshot));
  const topStartup = $derived.by(() => snapshot?.startupItems.slice(0, 8) ?? []);
  const allCachePaths = $derived.by(() => getAllCachePaths(snapshot?.installs ?? []));

  function flash(result: ActionResult, tone: "success" | "error" = "success") {
    toast = { tone, message: result.message };
    setTimeout(() => {
      if (toast?.message === result.message) toast = null;
    }, 3200);
  }

  async function refresh() {
    loading = true;
    errorMessage = "";
    try {
      snapshot = await getScanSnapshot();
      await refreshSessionStatus();
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      loading = false;
    }
  }

  async function refreshSessionStatus() {
    sessionStatus = await getSessionStatus();
  }

  async function refreshEverythingStatus() {
    everythingStatus = await getEverythingStatus();
    everythingChecked = true;
  }

  async function refreshProjectIndex(mode: "quick" | "full") {
    try {
      projectIndex = await act(`projects-${mode}`, () => getProjectIndex(mode));
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function installEverythingPackage(optionId: string) {
    const result = await act(`everything-install-${optionId}`, () => installEverything(optionId));
    flash(result, result.success ? "success" : "error");
    await refreshEverythingStatus();
  }

  async function checkEverythingStatus() {
    await act("everything-status", refreshEverythingStatus);
  }

  async function beginSession() {
    const result = await act("session-start", startSessionMode);
    flash(result, result.success ? "success" : "error");
    await refreshSessionStatus();
  }

  async function endSession() {
    const result = await act("session-stop", stopSessionMode);
    flash(result, result.success ? "success" : "error");
    await refreshSessionStatus();
  }

  async function act<T>(token: string, task: () => Promise<T>) {
    busy = token;
    try {
      return await task();
    } finally {
      busy = null;
    }
  }

  async function clearInstall(kind: "cache" | "profile", install: AfterEffectsInstall) {
    const paths = kind === "cache" ? install.cachePaths : install.profilePaths;
    if (!paths.length) return;
    const result = await act(`${kind}-${install.id}`, () => clearDirectories(paths));
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function setInstallPerformanceMode(
    install: AfterEffectsInstall,
    mode: PerformanceMode,
  ) {
    if (!install.exePath) return;
    const result = await act(`perf-${install.id}`, () =>
      setPerformanceMode(install.exePath!, mode),
    );
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function applyPower(mode: "stable" | "performance") {
    const result = await act(`power-${mode}`, () => applyPowerProfile(mode));
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function disableItem(item: StartupItem) {
    const result = await act(`startup-${item.id}`, () => disableStartupItem(item));
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  async function clearAllCaches() {
    if (!allCachePaths.length) return;
    const result = await act("cache-all", () => clearDirectories(allCachePaths));
    flash(result, result.success ? "success" : "error");
    await refresh();
  }

  function openPath(path: string | null) {
    if (!path) return;
    void revealPath(path);
  }

  onMount(refresh);
</script>

<svelte:head>
  <title>Aether FX Optimizer</title>
</svelte:head>

<div class="min-h-screen px-5 py-5 md:px-8">
  <div class="mx-auto flex max-w-[1460px] flex-col gap-5">
    <section class="grid gap-5 lg:grid-cols-[1.2fr_0.8fr]">
      <HeroSection
        {loading}
        {busy}
        warningMessages={snapshot?.warnings ?? []}
        {errorMessage}
        canClearCaches={allCachePaths.length > 0}
        onRefresh={refresh}
        onClearAllCaches={clearAllCaches}
        onApplyPower={applyPower}
      />

      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-1">
        <HealthScoreCard
          {healthScore}
          recommendationCount={snapshot?.recommendations.length ?? 0}
        />
        <SystemProfileCard system={snapshot?.system ?? null} />
      </div>
    </section>

    <section class="grid gap-5 lg:grid-cols-[1fr_0.85fr]">
      <EverythingStatusPanel
        status={everythingStatus}
        {busy}
        onInstall={installEverythingPackage}
        onCheck={checkEverythingStatus}
        checked={everythingChecked}
      />
      <SessionModePanel
        status={sessionStatus}
        {busy}
        onStart={beginSession}
        onStop={endSession}
      />
    </section>

    <section class="grid gap-5 xl:grid-cols-[1.15fr_0.85fr]">
      <InstallSection
        installs={snapshot?.installs ?? []}
        {loading}
        {busy}
        onSetPerformanceMode={setInstallPerformanceMode}
        onClearInstall={clearInstall}
        onOpenPath={openPath}
      />

      <div class="grid gap-5">
        <ProjectIndexPanel
          {projectIndex}
          {busy}
          onRefresh={refreshProjectIndex}
          onOpenPath={openPath}
        />
        <RecommendationsPanel recommendations={snapshot?.recommendations ?? []} />
        <StartupNoisePanel
          items={topStartup}
          totalCount={snapshot?.startupItems.length ?? 0}
          {busy}
          onDisable={disableItem}
        />
      </div>
    </section>
  </div>

  <ToastBanner {toast} />
</div>
