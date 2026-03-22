<script lang="ts">
  import type { AfterEffectsInstall, PerformanceMode, PluginEntry } from "$lib/types";
  import { open } from "@tauri-apps/plugin-dialog";

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

<article class="relative overflow-hidden rounded-[24px] border border-white/5 bg-white/2 p-4 md:p-5 transition-all outline-1 outline-transparent hover:outline-white/5 hover:bg-white/4 group/card">
  {#if install.isRunning}
    <div class="absolute inset-0 bg-amber-500/5 opacity-0 group-hover/card:opacity-100 transition duration-500 pointer-events-none mix-blend-screen"></div>
  {/if}

  <button
    class="relative z-10 flex w-full items-start justify-between gap-4 text-left active:scale-[0.99] transition-transform"
    onclick={() => (expanded = !expanded)}
  >
    <div class="space-y-2 flex-1 min-w-0">
      <div class="flex flex-wrap items-center gap-2.5">
        <span class="text-xl font-bold tracking-tight text-white">{install.displayName}</span>
        <span class="mono rounded-full border border-indigo-500/20 bg-indigo-500/10 px-2.5 py-0.5 text-[10px] uppercase font-bold tracking-[0.2em] text-indigo-400">
          Build {install.versionHint}
        </span>
        <span class="rounded-full border border-white/10 bg-white/5 px-2.5 py-0.5 text-[9px] uppercase font-bold tracking-widest text-(--muted)">
          {install.performanceMode}
        </span>
        {#if install.isRunning}
          <span class="flex items-center gap-1.5 rounded-full border border-amber-500/20 bg-amber-500/10 px-2.5 py-0.5 text-[9px] font-bold uppercase tracking-[0.2em] text-amber-500">
            <span class="h-1.5 w-1.5 rounded-full bg-amber-500 animate-pulse"></span>
            Instance Active
          </span>
        {/if}
      </div>
      <div class="flex items-center gap-2">
         <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-(--muted)"><path d="M22 12s-3-7-10-7-10 7-10 7 3 7 10 7 10-7 10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
         <p class="text-[11px] mono text-(--muted) font-medium truncate">{install.installRoot ?? "Orphaned Installation"}</p>
      </div>
    </div>

    <div class="flex flex-col md:flex-row md:items-center gap-4 shrink-0 mt-1 md:mt-0">
      <div class="flex gap-2 text-right">
        <div class="flex flex-col items-center justify-center rounded-[16px] border border-white/5 bg-white/5 min-w-[60px] py-1.5">
          <span class="text-[18px] font-black tracking-tighter text-white leading-none">{install.plugins.length}</span>
          <span class="text-[8px] uppercase tracking-[0.2em] font-bold text-(--muted) mt-0.5">Plugins</span>
        </div>
        <div class="flex flex-col items-center justify-center rounded-[16px] border border-white/5 bg-white/5 min-w-[60px] py-1.5">
          <span class="text-[18px] font-black tracking-tighter text-white leading-none">{install.cachePaths.length}</span>
          <span class="text-[8px] uppercase tracking-[0.2em] font-bold text-(--muted) mt-0.5">Caches</span>
        </div>
      </div>
      <div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-white/10 bg-white/5 text-(--muted) transition-transform {expanded ? 'rotate-180' : ''}">
         <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="m6 9 6 6 6-6"/></svg>
      </div>
    </div>
  </button>

  {#if expanded}
    <div class="relative z-10 mt-6 grid gap-5 border-t border-white/5 pt-6">
      <!-- Top Grid -->
      <div class="grid gap-4 lg:grid-cols-2">
        <div class="rounded-[20px] border border-white/5 bg-white/2 p-4">
          <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) mb-2">Primary Executable</p>
          <div class="rounded-xl bg-black/40 p-3 border border-white/5">
             <p class="mono break-all text-[11px] leading-5 text-(--muted)">{install.exePath ?? "Binary not located"}</p>
          </div>
        </div>

        <div class="rounded-[20px] border border-indigo-500/20 bg-indigo-500/5 p-4 relative overflow-hidden">
          <div class="glass-grain opacity-[0.05]"></div>
          <p class="relative z-10 text-[10px] uppercase font-bold tracking-[0.2em] text-indigo-400/80 mb-3">
            Hardware Performance Strategy
          </p>
          <div class="relative z-10 flex flex-wrap gap-2">
            <button
              class={`rounded-[12px] px-4 py-2.5 text-[10px] font-bold uppercase tracking-widest transition-all ${install.performanceMode === "balanced" ? "bg-indigo-500 text-slate-950 shadow-lg shadow-indigo-500/30" : "border border-white/10 bg-white/5 hover:border-indigo-500/30 hover:bg-indigo-500/10 hover:text-indigo-300"} disabled:opacity-40`}
              onclick={() => onSetPerformanceMode(install, "balanced")}
              disabled={!install.exePath || busy === `perf-${install.id}`}
            >
              Balanced Mode
            </button>
            <button
              class={`rounded-[12px] px-4 py-2.5 text-[10px] font-bold uppercase tracking-widest transition-all ${install.performanceMode === "gpu" ? "bg-emerald-500 text-slate-950 shadow-lg shadow-emerald-500/30" : "border border-white/10 bg-white/5 hover:border-emerald-500/30 hover:bg-emerald-500/10 hover:text-emerald-300"} disabled:opacity-40`}
              onclick={() => onSetPerformanceMode(install, "gpu")}
              disabled={!install.exePath || busy === `perf-${install.id}`}
            >
              Force GPU Priority
            </button>
            <button
              class={`rounded-[12px] px-4 py-2.5 text-[10px] font-bold uppercase tracking-widest transition-all ${install.performanceMode === "cpu" ? "bg-amber-500 text-slate-950 shadow-lg shadow-amber-500/30" : "border border-white/10 bg-white/5 hover:border-amber-500/30 hover:bg-amber-500/10 hover:text-amber-300"} disabled:opacity-40`}
              onclick={() => onSetPerformanceMode(install, "cpu")}
              disabled={!install.exePath || busy === `perf-${install.id}`}
            >
              Force CPU Priority
            </button>
          </div>
        </div>
      </div>

      <!-- Directories -->
      <div class="grid gap-4 lg:grid-cols-2">
        <div class="rounded-[20px] border border-white/5 bg-white/2 p-4">
          <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) mb-3">Detected Plugin Roots</p>
          <div class="space-y-2">
            {#if install.pluginPaths.length}
              {#each install.pluginPaths as folder}
                <div class="rounded-xl bg-black/40 p-3 border border-white/5 flex items-center gap-3 group">
                   <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-indigo-400 shrink-0"><path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"/></svg>
                   <p class="mono break-all text-[10px] leading-relaxed text-(--muted) group-hover:text-white transition-colors">{folder}</p>
                </div>
              {/each}
            {:else}
              <p class="text-xs italic text-(--muted)">No plugin directories located.</p>
            {/if}
          </div>
        </div>

        <div class="rounded-[20px] border border-white/5 bg-white/2 p-4">
          <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) mb-3">
            Profile & Cache Matrix
          </p>
          <div class="space-y-2">
            {#if install.profilePaths.length || install.cachePaths.length}
              {#each [...install.profilePaths, ...install.cachePaths] as folder}
                <div class="rounded-xl bg-black/40 p-3 border border-white/5 flex items-center gap-3 group">
                   <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-rose-400 shrink-0"><path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"/><polyline points="14 2 14 8 20 8"/><path d="M2 15h10"/><path d="m9 18 3-3-3-3"/></svg>
                   <p class="mono break-all text-[10px] leading-relaxed text-(--muted) group-hover:text-white transition-colors">{folder}</p>
                </div>
              {/each}
            {:else}
              <p class="text-xs italic text-(--muted)">No profile structures mapped.</p>
            {/if}
          </div>
        </div>
      </div>

      <!-- Action Footer -->
      <div class="flex flex-wrap items-center gap-2 p-4 rounded-[20px] bg-white/2 border border-white/5">
        <button
          class="rounded-[12px] border border-white/10 bg-white/5 px-5 py-2.5 text-[10px] font-bold uppercase tracking-widest transition hover:border-white/20 hover:bg-white/10 active:scale-95 disabled:opacity-40"
          onclick={() => onClearInstall("cache", install)}
          disabled={!install.cachePaths.length || busy === `cache-${install.id}`}
        >
          {busy === `cache-${install.id}` ? "Clearing..." : "Purge Assigned Cache"}
        </button>
        <button
          class="rounded-[12px] border border-white/10 bg-white/5 px-5 py-2.5 text-[10px] font-bold uppercase tracking-widest transition hover:border-white/20 hover:bg-white/10 active:scale-95 disabled:opacity-40"
          onclick={() => onClearInstall("profile", install)}
          disabled={!install.profilePaths.length || busy === `profile-${install.id}`}
        >
          {busy === `profile-${install.id}` ? "Resetting..." : "Rebuild Profile"}
        </button>
        <div class="h-6 w-px bg-white/10 mx-1"></div>
        <button
          class="rounded-[12px] border border-white/10 bg-white/5 px-5 py-2.5 text-[10px] font-bold uppercase tracking-widest transition hover:border-white/20 hover:bg-white/10 active:scale-95 disabled:opacity-40"
          onclick={() => onOpenPath(install.installRoot)}
          disabled={!install.installRoot}
        >
          Explore Root
        </button>
        <button
          class="rounded-[12px] bg-emerald-500 px-6 py-2.5 text-[10px] font-bold uppercase tracking-widest text-slate-950 transition hover:bg-white hover:shadow-lg active:scale-95 disabled:opacity-40 ml-auto"
          onclick={pickAndInstallScript}
          disabled={busy === `install-script-${install.id}`}
        >
          {busy === `install-script-${install.id}` ? "Injecting..." : "Inject Script"}
        </button>
      </div>

      <!-- Plugins Section -->
      <div class="rounded-[24px] border border-white/5 bg-black/20 p-5 shadow-inner">
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-5 border-b border-white/5 pb-5">
          <div>
            <p class="text-[12px] uppercase font-bold tracking-[0.2em] text-white">Encapsulated Plugins</p>
            <p class="text-[10px] text-(--muted) mt-1 uppercase tracking-widest">{install.plugins.length} active modules discovered</p>
          </div>
          
          <div class="flex items-center gap-1.5 p-1 rounded-full bg-white/5 border border-white/5">
            {#each healthFilterOptions as option}
              <button
                class={`rounded-full px-4 py-1.5 text-[9px] font-bold uppercase tracking-widest transition-all ${healthFilter === option.id ? "bg-white text-slate-950 shadow-md" : "text-(--muted) hover:text-white hover:bg-white/10"}`}
                onclick={() => (healthFilter = option.id)}
              >
                {option.label}
              </button>
            {/each}
          </div>
        </div>

        <div class="space-y-4">
          {#if groupedPlugins.length}
            {#each groupedPlugins as group}
              <div class="rounded-[20px] border border-white/5 bg-white/2 p-3 pb-2 transition-all hover:border-white/10">
                <button
                  class="flex w-full items-center justify-between gap-3 text-left focus:outline-none px-2 py-1"
                  onclick={() => toggleSource(group.source)}
                >
                  <div class="flex items-center gap-3">
                    <span class="rounded-full border border-indigo-500/20 bg-indigo-500/10 px-3 py-1 text-[10px] font-bold uppercase tracking-widest text-indigo-400">
                      {group.source}
                    </span>
                    <span class="text-[10px] uppercase font-bold tracking-widest text-(--muted)">{group.count} {group.count === 1 ? 'module' : 'modules'} limit</span>
                  </div>
                  <div class="flex items-center gap-4 text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted)">
                    {#if group.unsigned > 0}
                        <span class="text-rose-400">Volatile ({group.unsigned})</span>
                    {/if}
                    {#if group.duplicates > 0}
                        <span class="text-amber-400">Duplicated ({group.duplicates})</span>
                    {/if}
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" class="transition-transform {expandedSources[group.source] ?? true ? 'rotate-180' : ''}"><path d="m6 9 6 6 6-6"/></svg>
                  </div>
                </button>
                
                {#if expandedSources[group.source] ?? true}
                  <div class="mt-4 grid gap-2">
                    {#each group.plugins.slice(0, 24) as plugin}
                      <div class={`group/plugin flex flex-col md:flex-row md:items-center justify-between gap-3 rounded-[16px] border px-4 py-3 transition-all ${plugin.isEnabled ? "border-white/5 bg-white/2 hover:bg-white/5" : "border-transparent bg-black/40 opacity-50"}`}>
                        <div class="min-w-0">
                          <div class="flex items-center gap-2.5 mb-1">
                            <p class="truncate text-sm font-bold text-white group-hover/plugin:text-indigo-300 transition-colors">{plugin.name}</p>
                            {#if !plugin.isEnabled}
                              <span class="rounded-full bg-white/10 border border-white/10 px-2 py-0.5 text-[8px] uppercase font-bold tracking-[0.2em] text-(--muted)">Disabled Framework</span>
                            {/if}
                          </div>
                          <p class="mono truncate text-[10px] text-(--muted)">{plugin.path}</p>
                        </div>

                        <div class="flex flex-wrap md:flex-nowrap items-center gap-2 shrink-0">
                          <!-- Size badge -->
                          <span class="rounded-[10px] border border-white/5 bg-white/5 px-2.5 py-1 text-[9px] font-bold uppercase tracking-widest text-(--muted)">{plugin.sizeMb} MB</span>
                          
                          <!-- Signature badge -->
                          <span class={`rounded-[10px] border px-2.5 py-1 text-[9px] font-bold uppercase tracking-widest ${plugin.hasSignature ? "border-emerald-500/20 bg-emerald-500/5 text-emerald-400" : "border-rose-500/20 bg-rose-500/5 text-rose-400"}`}>
                            {plugin.hasSignature ? "Certified" : "Unsigned"}
                          </span>
                          
                          <!-- Duplicate badge -->
                          {#if plugin.duplicateCount > 1}
                            <span class="rounded-[10px] border border-amber-500/20 bg-amber-500/5 px-2.5 py-1 text-[9px] font-bold uppercase tracking-widest text-amber-400 flex items-center gap-1">
                                <span class="h-1.5 w-1.5 rounded-full bg-amber-500 animate-pulse"></span>
                                Overlay ({plugin.duplicateCount})
                            </span>
                          {/if}
                          
                          <div class="h-4 w-px bg-white/10 mx-1 hidden md:block"></div>
                          
                          <!-- Action button -->
                          <button
                            class={`rounded-[12px] px-5 py-2 text-[10px] font-bold uppercase tracking-widest transition-all ${plugin.isEnabled ? "bg-white/5 text-white hover:bg-rose-500 hover:text-slate-950 hover:shadow-lg hover:shadow-rose-500/20" : "bg-emerald-500 text-slate-950 hover:bg-white hover:shadow-lg hover:shadow-emerald-500/20"}`}
                            onclick={() => onTogglePlugin(install, plugin, !plugin.isEnabled)}
                            disabled={busy === `toggle-${plugin.id}`}
                          >
                            {busy === `toggle-${plugin.id}` ? "..." : (plugin.isEnabled ? "Isolate" : "Activate")}
                          </button>
                        </div>
                      </div>
                    {/each}
                    {#if group.plugins.length > 24}
                      <p class="text-[10px] font-bold uppercase tracking-[0.2em] text-(--muted) text-center py-2 italic opacity-60">Truncated: rendering first 24 of {group.plugins.length} active modules.</p>
                    {/if}
                  </div>
                {/if}
              </div>
            {/each}
          {:else}
            <div class="flex items-center justify-center p-8 border border-dashed border-white/10 rounded-[20px] bg-black/20">
               <p class="text-[11px] font-bold uppercase tracking-[0.2em] text-(--muted)">No compliant modules match constraints.</p>
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</article>
