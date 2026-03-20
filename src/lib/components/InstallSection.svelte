<script lang="ts">
  import type { AfterEffectsInstall } from "$lib/types";
  import InstallCard from "./InstallCard.svelte";

  import type { PerformanceMode } from "$lib/types";

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

<div class="panel rounded-[28px] p-4 md:p-5">
  <div class="mb-4 flex items-center justify-between gap-4">
    <div>
      <p class="mono text-[11px] uppercase tracking-[0.24em] text-(--muted)">After Effects installs</p>
      <h2 class="mt-2 text-2xl font-semibold">Builds, profiles, and cache targets</h2>
    </div>
    <div class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs">{installs.length} detected</div>
  </div>

  <div class="grid gap-4">
    {#if loading}
      <div class="rounded-[24px] border border-white/8 bg-white/4 p-5 text-(--muted)">Scanning Adobe installs and version folders...</div>
    {:else if installs.length}
      {#each installs as install}
        <InstallCard {install} {busy} {onSetPerformanceMode} {onClearInstall} {onOpenPath} {onTogglePlugin} {onInstallScript} />
      {/each}
    {:else}
      <div class="rounded-[24px] border border-white/8 bg-white/4 p-5 text-(--muted)">No installs detected yet.</div>
    {/if}
  </div>
</div>
