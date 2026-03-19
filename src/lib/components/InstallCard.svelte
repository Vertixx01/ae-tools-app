<script lang="ts">
  import type { AfterEffectsInstall, PerformanceMode, PluginEntry } from "$lib/types";

  interface Props {
    install: AfterEffectsInstall;
    busy: string | null;
    onSetPerformanceMode: (install: AfterEffectsInstall, mode: PerformanceMode) => void;
    onClearInstall: (kind: "cache" | "profile", install: AfterEffectsInstall) => void;
    onOpenPath: (path: string | null) => void;
  }

  let { install, busy, onSetPerformanceMode, onClearInstall, onOpenPath }: Props = $props();
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

<article class="rounded-[24px] border border-white/8 bg-white/4 p-4 md:p-5">
  <button
    class="flex w-full items-start justify-between gap-4 text-left"
    onclick={() => (expanded = !expanded)}
  >
    <div class="space-y-2">
      <div class="flex flex-wrap items-center gap-2">
        <span class="text-xl font-semibold">{install.displayName}</span>
        <span class="mono rounded-full border border-white/10 bg-white/6 px-2 py-1 text-[10px] uppercase tracking-[0.16em] text-[color:var(--accent)]">
          v{install.versionHint}
        </span>
        <span class="rounded-full border border-white/10 bg-white/6 px-2 py-1 text-[10px] uppercase tracking-[0.16em]">
          {install.performanceMode}
        </span>
        {#if install.isRunning}
          <span class="rounded-full border border-[color:var(--warn)]/35 bg-[color:rgba(255,216,124,0.08)] px-2 py-1 text-[10px] uppercase tracking-[0.16em] text-[color:var(--warn)]">
            running
          </span>
        {/if}
      </div>
      <p class="text-sm text-[color:var(--muted)]">
        {install.installRoot ?? "No install root detected."}
      </p>
    </div>

    <div class="flex items-center gap-3">
      <div class="grid gap-2 text-right text-sm">
        <div class="rounded-2xl border border-white/8 bg-white/4 px-3 py-2">
          <span class="text-[color:var(--muted)]">Plugins:</span>
          <span class="font-semibold">{install.plugins.length}</span>
        </div>
        <div class="rounded-2xl border border-white/8 bg-white/4 px-3 py-2">
          <span class="text-[color:var(--muted)]">Caches:</span>
          <span class="font-semibold">{install.cachePaths.length}</span>
        </div>
      </div>
      <span class="mono text-xs text-[color:var(--muted)]">{expanded ? "[-]" : "[+]"}</span>
    </div>
  </button>

  {#if expanded}
    <div class="mt-5 grid gap-4">
      <div class="grid gap-3 lg:grid-cols-[1fr_0.9fr]">
        <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
          <p class="text-xs uppercase tracking-[0.2em] text-[color:var(--muted)]">Executable</p>
          <p class="mono mt-2 break-all text-xs leading-6">{install.exePath ?? "Not found"}</p>
        </div>

        <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
          <p class="text-xs uppercase tracking-[0.2em] text-[color:var(--muted)]">
            Performance mode
          </p>
          <div class="mt-3 flex flex-wrap gap-2">
            <button
              class={`rounded-full px-3 py-2 text-xs font-semibold transition ${install.performanceMode === "balanced" ? "bg-[color:var(--accent-2)] text-slate-950" : "border border-white/10 bg-white/6 hover:border-white/20 hover:bg-white/10"}`}
              onclick={() => onSetPerformanceMode(install, "balanced")}
              disabled={!install.exePath || busy === `perf-${install.id}`}
            >
              Balanced
            </button>
            <button
              class={`rounded-full px-3 py-2 text-xs font-semibold transition ${install.performanceMode === "gpu" ? "bg-[color:var(--accent)] text-slate-950" : "border border-white/10 bg-white/6 hover:border-white/20 hover:bg-white/10"}`}
              onclick={() => onSetPerformanceMode(install, "gpu")}
              disabled={!install.exePath || busy === `perf-${install.id}`}
            >
              GPU priority
            </button>
            <button
              class={`rounded-full px-3 py-2 text-xs font-semibold transition ${install.performanceMode === "cpu" ? "bg-[color:var(--warn)] text-slate-950" : "border border-white/10 bg-white/6 hover:border-white/20 hover:bg-white/10"}`}
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
          <p class="text-xs uppercase tracking-[0.2em] text-[color:var(--muted)]">Plugin roots</p>
          <div class="mt-2 space-y-1 text-xs leading-6 text-[color:var(--muted)]">
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
          <p class="text-xs uppercase tracking-[0.2em] text-[color:var(--muted)]">
            Profiles and cache folders
          </p>
          <div class="mt-2 space-y-1 text-xs leading-6 text-[color:var(--muted)]">
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
              <div class="rounded-2xl border border-white/6 bg-black/5 p-3">
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
                    {#each group.plugins.slice(0, 12) as plugin}
                      <div class="flex flex-wrap items-center justify-between gap-3 rounded-2xl border border-white/6 bg-white/6 px-3 py-2 text-sm">
                        <div class="min-w-0">
                          <p class="truncate font-medium">{plugin.name}</p>
                          <p class="mono truncate text-xs text-[color:var(--muted)]">{plugin.path}</p>
                        </div>
                        <div class="flex flex-wrap items-center gap-1 text-xs">
                          <span class="rounded-full border border-white/10 bg-white/8 px-2 py-1 uppercase tracking-[0.14em]">{plugin.source}</span>
                          <span class="rounded-full border border-white/10 bg-white/8 px-2 py-1">{plugin.sizeMb} MB</span>
                          <span class={`rounded-full px-2 py-1 text-[10px] uppercase tracking-[0.14em] ${plugin.hasSignature ? "border border-white/10 bg-white/6" : "border border-[color:var(--warn)] bg-[color:rgba(255,216,124,0.06)] text-[color:var(--warn)]"}`}>
                            {plugin.hasSignature ? "signed" : "unsigned"}
                          </span>
                        </div>
                      </div>
                    {/each}
                    {#if group.plugins.length > 12}
                      <p class="text-xs text-[color:var(--muted)]">Showing 12 of {group.plugins.length} results for this source.</p>
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
      </div>
    </div>
  {/if}
</article>
