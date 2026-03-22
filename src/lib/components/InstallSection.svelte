<script lang="ts">
  import type { AfterEffectsInstall, PerformanceMode } from "$lib/types";
  import InstallCard from "./InstallCard.svelte";

  interface Props {
    installs: AfterEffectsInstall[];
    loading: boolean;
    busy: string | null;
    onSetPerformanceMode: (install: AfterEffectsInstall, mode: PerformanceMode) => void;
    onClearInstall: (kind: "cache" | "profile", install: AfterEffectsInstall) => void;
    onOpenPath: (path: string | null) => void;
    onTogglePlugin: (install: AfterEffectsInstall, plugin: any, enable: boolean) => void;
    onInstallScript: (install: AfterEffectsInstall, scriptPath: string) => void;
  }

  let { installs, loading, busy, onSetPerformanceMode, onClearInstall, onOpenPath, onTogglePlugin, onInstallScript }: Props = $props();
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8">
  <div class="mb-6 flex flex-wrap items-start justify-between gap-4 border-b border-white/5 pb-6">
    <div>
      <div class="flex items-center gap-2 mb-2">
         <span class="mono text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) opacity-80">Software Architecture</span>
      </div>
      <h2 class="text-2xl font-bold tracking-tight">After Effects Installations</h2>
      <p class="text-xs text-(--muted) mt-2 max-w-sm leading-relaxed font-medium">Builds, performance profiles, caches, and installed third-party plugins.</p>
    </div>
    <div class="rounded-full border border-white/5 bg-white/5 px-4 py-1.5 text-[10px] uppercase font-bold tracking-widest text-(--muted) shadow-inner flex items-center gap-2">
      <span class="h-1.5 w-1.5 rounded-full bg-white/30"></span>
      {installs.length} Detected
    </div>
  </div>

  <div class="grid gap-4">
    {#if loading}
      <div class="flex flex-col items-center justify-center rounded-[32px] border border-white/5 bg-white/2 py-16 text-center opacity-70">
         <div class="h-8 w-8 animate-spin rounded-full border-2 border-indigo-500/20 border-t-indigo-500 mb-4"></div>
         <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted)">Crawling Registry & Filesystem...</p>
      </div>
    {:else if installs.length}
      {#each installs as install}
        <InstallCard {install} {busy} {onSetPerformanceMode} {onClearInstall} {onOpenPath} {onTogglePlugin} {onInstallScript} />
      {/each}
    {:else}
      <div class="flex flex-col items-center justify-center rounded-[32px] border-2 border-dashed border-white/5 bg-white/2 py-16 text-center opacity-70">
         <p class="text-xs uppercase font-bold tracking-widest text-white">No Instances Found</p>
         <p class="text-[10px] mt-1 text-(--muted) max-w-[200px]">Adobe After Effects is either uninstalled or located outside standard paths.</p>
      </div>
    {/if}
  </div>
</div>
