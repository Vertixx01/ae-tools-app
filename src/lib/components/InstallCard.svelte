<script lang="ts">
  import type { AfterEffectsInstall, PerformanceMode, PluginEntry } from "$lib/types";

  interface Props {
    install: AfterEffectsInstall;
    busy: string | null;
    onSetPerformanceMode: (install: AfterEffectsInstall, mode: PerformanceMode) => void;
    onClearInstall: (kind: "cache" | "profile", install: AfterEffectsInstall) => void;
    onOpenPath: (path: string | null) => void;
    onTogglePlugin: (install: AfterEffectsInstall, plugin: PluginEntry, enable: boolean) => void;
    onInstallScript: (install: AfterEffectsInstall, scriptPath: string) => void;
  }

  let { install, busy, onSetPerformanceMode, onClearInstall, onOpenPath, onTogglePlugin, onInstallScript }: Props = $props();
  
  import { open } from "@tauri-apps/plugin-dialog";

  async function pickAndInstallScript() {
    const selected = await open({
        multiple: false,
        filters: [{ name: "After Effects Script", extensions: ["jsx", "jsxbin"] }]
    });
    if (selected && !Array.isArray(selected)) {
        onInstallScript(install, selected);
    }
  }

  let expanded = $state(false);
  let healthFilter = $state<"all" | "unsigned" | "duplicates">("all");
  let expandedSources = $state<Record<string, boolean>>({});

  const filteredPlugins = $derived.by(() =>
    install.plugins.filter((plugin) => {
      if (healthFilter === "unsigned" && plugin.hasSignature) return false;
      if (healthFilter === "duplicates" && plugin.duplicateCount < 2) return false;
      return true;
    }),
  );

  const groupedPlugins = $derived.by(() => {
    const groups: Record<string, PluginEntry[]> = {};
    for (const plugin of filteredPlugins) {
      if (!groups[plugin.source]) {
        groups[plugin.source] = [];
      }
      groups[plugin.source].push(plugin);
    }
    return Object.entries(groups)
      .map(([source, items]) => ({
        source,
        count: items.length,
        unsigned: items.filter((entry) => !entry.hasSignature).length,
        duplicates: items.filter((entry) => entry.duplicateCount > 1).length,
        plugins: items.sort((a, b) => a.name.localeCompare(b.name)),
      }))
      .sort((a, b) => a.source.localeCompare(b.source));
  });

  const healthFilterOptions = [
    { id: "all", label: "Show everything" },
    { id: "unsigned", label: "Unsigned only" },
    { id: "duplicates", label: "Duplicates only" },
  ] as const;

  const filterLabel = $derived.by(
    () =>
      healthFilterOptions.find((option) => option.id === healthFilter)?.label ??
      healthFilterOptions[0].label,
  );

  function toggleSource(source: string) {
    expandedSources = { ...expandedSources, [source]: !(expandedSources[source] ?? true) };
  }
</script>

<article class="rounded-[24px] border border-white/8 bg-white/4 p-3 md:p-4">
  <button
    class="flex w-full items-start justify-between gap-4 text-left"
    onclick={() => (expanded = !expanded)}
  >
    <div class="space-y-1">
      <div class="flex flex-wrap items-center gap-2">
        <span class="text-xl font-semibold">{install.displayName}</span>
        <span class="mono rounded-full border border-white/10 bg-white/6 px-2 py-1 text-[10px] uppercase tracking-[0.16em] text-(--accent)">
          v{install.versionHint}
        </span>
        <span class="rounded-full border border-white/10 bg-white/6 px-2 py-1 text-[10px] uppercase tracking-[0.16em]">
          {install.performanceMode}
        </span>
        {#if install.isRunning}
          <span class="rounded-full border border-(--warn)/35 bg-(--warn)/8 px-2 py-1 text-[10px] uppercase tracking-[0.16em] text-(--warn)">
            running
          </span>
        {/if}
      </div>
      <p class="text-sm text-(--muted)">
        {install.installRoot ?? "No install root detected."}
      </p>
    </div>

    <div class="flex items-center gap-3">
      <div class="grid gap-2 text-right text-sm">
        <div class="rounded-2xl border border-white/8 bg-white/4 px-3 py-2">
          <span class="text-(--muted)">Plugins:</span>
          <span class="font-semibold">{install.plugins.length}</span>
        </div>
        <div class="rounded-2xl border border-white/8 bg-white/4 px-3 py-2">
          <span class="text-(--muted)">Caches:</span>
          <span class="font-semibold">{install.cachePaths.length}</span>
        </div>
      </div>
      <span class="mono text-xs text-(--muted)">{expanded ? "[-]" : "[+]"}</span>
    </div>
  </button>

  {#if expanded}
    <div class="mt-4 grid gap-4">
      <div class="grid gap-3 lg:grid-cols-[1fr_0.9fr]">
        <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
          <p class="text-xs uppercase tracking-[0.2em] text-(--muted)">Executable</p>
          <p class="mono mt-2 break-all text-xs leading-6">{install.exePath ?? "Not found"}</p>
        </div>

        <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
          <p class="text-xs uppercase tracking-[0.2em] text-(--muted)">
            Performance mode
          </p>
          <div class="mt-3 flex flex-wrap gap-2">
            <button
              class={`rounded-full px-3 py-2 text-xs font-semibold transition ${install.performanceMode === "balanced" ? "bg-(--accent-2) text-slate-950" : "border border-white/10 bg-white/6 hover:border-white/20 hover:bg-white/10"}`}
              onclick={() => onSetPerformanceMode(install, "balanced")}
              disabled={!install.exePath || busy === `perf-${install.id}`}
            >
              Balanced
            </button>
            <button
              class={`rounded-full px-3 py-2 text-xs font-semibold transition ${install.performanceMode === "gpu" ? "bg-(--accent) text-slate-950" : "border border-white/10 bg-white/6 hover:border-white/20 hover:bg-white/10"}`}
              onclick={() => onSetPerformanceMode(install, "gpu")}
              disabled={!install.exePath || busy === `perf-${install.id}`}
            >
              GPU priority
            </button>
            <button
              class={`rounded-full px-3 py-2 text-xs font-semibold transition ${install.performanceMode === "cpu" ? "bg-(--warn) text-slate-950" : "border border-white/10 bg-white/6 hover:border-white/20 hover:bg-white/10"}`}
              onclick={() => onSetPerformanceMode(install, "cpu")}
              disabled={!install.exePath || busy === `perf-${install.id}`}
            >
              CPU priority
            </button>
          </div>
        </div>
      </div>

      <div class="grid gap-3 lg:grid-cols-2">
        <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
          <p class="text-xs uppercase tracking-[0.2em] text-(--muted)">Plugin roots</p>
          <div class="mt-2 space-y-1 text-xs leading-6 text-(--muted)">
            {#if install.pluginPaths.length}
              {#each install.pluginPaths as folder}
                <p class="mono break-all">{folder}</p>
              {/each}
            {:else}
              <p>No plugin roots detected.</p>
            {/if}
          </div>
        </div>

        <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
          <p class="text-xs uppercase tracking-[0.2em] text-(--muted)">
            Profiles and cache folders
          </p>
          <div class="mt-2 space-y-1 text-xs leading-6 text-(--muted)">
            {#if install.profilePaths.length || install.cachePaths.length}
              {#each [...install.profilePaths, ...install.cachePaths] as folder}
                <p class="mono break-all">{folder}</p>
              {/each}
            {:else}
              <p>No versioned folders found.</p>
            {/if}
          </div>
        </div>
      </div>

      <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
        <div class="flex items-center justify-between gap-4">
          <div>
            <p class="text-xs uppercase tracking-[0.2em] text-[color:var(--muted)]">Installed plugins</p>
            <span class="text-xs text-[color:var(--muted)]">{install.plugins.length} total discovered</span>
          </div>
          <span class="text-xs text-[color:var(--muted)]">
            {filteredPlugins.length} match{filteredPlugins.length === 1 ? "" : "es"}
          </span>
        </div>

        <div class="mt-3 flex flex-wrap gap-2 text-[11px] uppercase tracking-[0.18em]">
          {#each healthFilterOptions as option}
            <button
              class={`rounded-full px-3 py-1 text-xs font-semibold transition ${healthFilter === option.id ? "bg-[color:var(--accent-2)] text-slate-950" : "border border-white/10 bg-white/6 hover:border-white/20 hover:bg-white/10"}`}
              onclick={() => (healthFilter = option.id)}
            >
              {option.label}
            </button>
          {/each}
        </div>

        <p class="mt-2 text-xs text-[color:var(--muted)]">
          {filterLabel} filter engaged.
        </p>

        <div class="mt-4 space-y-3">
          {#if groupedPlugins.length}
            {#each groupedPlugins as group}
              <div class="rounded-2xl border border-white/6 bg-black/5 p-2">
                <button
                  class="flex w-full items-center justify-between gap-3 text-sm font-semibold text-left"
                  onclick={() => toggleSource(group.source)}
                >
                  <div class="flex items-center gap-2">
                    <span class="rounded-full border border-white/10 bg-white/6 px-2 py-0.5 text-[10px] uppercase tracking-[0.14em]">
                      {group.source}
                    </span>
                    <span class="text-[color:var(--muted)]">{group.count} tracked</span>
                  </div>
                  <div class="flex items-center gap-3 text-[10px] uppercase tracking-[0.2em] text-[color:var(--muted)]">
                    <span>unsigned {group.unsigned}</span>
                    <span>duplicates {group.duplicates}</span>
                    <span>{expandedSources[group.source] ?? true ? "collapse" : "expand"}</span>
                  </div>
                </button>
                {#if expandedSources[group.source] ?? true}
                  <div class="mt-3 grid gap-2">
                    {#each group.plugins.slice(0, 24) as plugin}
                      <div class={`flex flex-wrap items-center justify-between gap-3 rounded-2xl border px-3 py-2 text-sm transition ${plugin.isEnabled ? "border-white/6 bg-white/6" : "border-white/4 bg-white/2 opacity-60"}`}>
                        <div class="min-w-0">
                          <div class="flex items-center gap-2">
                            <p class="truncate font-medium">{plugin.name}</p>
                            {#if !plugin.isEnabled}
                              <span class="rounded-full bg-white/10 px-1.5 py-0.5 text-[9px] uppercase tracking-wider text-[color:var(--muted)]">disabled</span>
                            {/if}
                          </div>
                          <p class="mono truncate text-xs text-[color:var(--muted)]">{plugin.path}</p>
                        </div>
                        <div class="flex flex-wrap items-center gap-2 text-xs">
                          <div class="flex items-center gap-1">
                             <span class="rounded-full border border-white/10 bg-white/8 px-2 py-1 uppercase tracking-[0.14em]">{plugin.source}</span>
                            <span class="rounded-full border border-white/10 bg-white/8 px-2 py-1">{plugin.sizeMb} MB</span>
                            <span class={`rounded-full px-2 py-1 text-[10px] uppercase tracking-[0.14em] ${plugin.hasSignature ? "border border-white/10 bg-white/6" : "border border-[color:var(--warn)] bg-[color:rgba(255,216,124,0.06)] text-[color:var(--warn)]"}`}>
                              {plugin.hasSignature ? "signed" : "unsigned"}
                            </span>
                            {#if plugin.duplicateCount > 1}
                              <span class="rounded-full border border-(--warn) bg-(--warn)/10 px-2 py-1 text-[10px] font-bold uppercase tracking-[0.14em] text-(--warn)">
                                Duplicate ({plugin.duplicateCount})
                              </span>
                            {/if}
                          </div>
                          
                          <button
                            class={`rounded-full px-3 py-1 font-semibold transition ${plugin.isEnabled ? "bg-white/10 text-white hover:bg-[color:var(--danger)] hover:text-slate-950" : "bg-[color:var(--accent)] text-slate-950 hover:bg-white/20 hover:text-white"}`}
                            onclick={() => onTogglePlugin(install, plugin, !plugin.isEnabled)}
                            disabled={busy === `toggle-${plugin.id}`}
                          >
                            {busy === `toggle-${plugin.id}` ? "..." : (plugin.isEnabled ? "Disable" : "Enable")}
                          </button>
                        </div>
                      </div>
                    {/each}
                    {#if group.plugins.length > 24}
                      <p class="text-xs text-[color:var(--muted)]">Showing 24 of {group.plugins.length} results for this source.</p>
                    {/if}
                  </div>
                {/if}
              </div>
            {/each}
          {:else}
            <p class="text-sm text-[color:var(--muted)]">No plugin entries match the current filters.</p>
          {/if}
        </div>
      </div>

      <div class="flex flex-wrap gap-2">
        <button
          class="rounded-full border border-white/10 bg-white/6 px-4 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60"
          onclick={() => onClearInstall("cache", install)}
          disabled={!install.cachePaths.length || busy === `cache-${install.id}`}
        >
          {busy === `cache-${install.id}` ? "Clearing..." : "Clear cache"}
        </button>
        <button
          class="rounded-full border border-white/10 bg-white/6 px-4 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60"
          onclick={() => onClearInstall("profile", install)}
          disabled={!install.profilePaths.length || busy === `profile-${install.id}`}
        >
          {busy === `profile-${install.id}` ? "Resetting..." : "Reset profile"}
        </button>
        <button
          class="rounded-full border border-white/10 bg-white/6 px-4 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60"
          onclick={() => onOpenPath(install.installRoot)}
          disabled={!install.installRoot}
        >
          Open install folder
        </button>
        <button
          class="rounded-full bg-[color:var(--accent)] px-4 py-2 text-sm font-semibold text-slate-950 transition hover:bg-white hover:text-slate-950 disabled:opacity-60"
          onclick={pickAndInstallScript}
          disabled={busy === `install-script-${install.id}`}
        >
          {busy === `install-script-${install.id}` ? "Installing..." : "Install Script"}
        </button>
      </div>
    </div>
  {/if}
</article>
