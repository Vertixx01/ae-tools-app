<script lang="ts">
  import type { ProjectIndexSnapshot } from "$lib/types";

  interface Props {
    projectIndex: ProjectIndexSnapshot | null;
    busy: string | null;
    onRefresh: (mode: "quick" | "full") => void;
    onOpenPath: (path: string | null) => void;
  }

  let { projectIndex, busy, onRefresh, onOpenPath }: Props = $props();
  const previewProjects = $derived.by(() => projectIndex?.projects.slice(0, 12) ?? []);
</script>

<div class="panel rounded-[28px] p-5 md:p-6">
    <div class="mb-5 flex flex-wrap items-center justify-between gap-4">
      <div>
        <p class="mono text-[11px] uppercase tracking-[0.24em] text-[color:var(--muted)]">
          Project index manager
        </p>
        <h2 class="mt-2 text-2xl font-semibold">Auto-scan AE projects across the PC</h2>
        {#if projectIndex}
          <p class="text-xs text-[color:var(--muted)]">
            Detects .aep/.aepx/.aet files {projectIndex.scannedMode === "full" ? "from every mounted drive" : "in common user folders"} and leans on
            {projectIndex.engine === "everything-es" ? "Everything's es.exe index" : "the filesystem walker"} for speed.
          </p>
        {/if}
      </div>
      <div class="flex flex-wrap gap-2">
        <button
        class="rounded-full border border-white/10 bg-white/6 px-4 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60"
        onclick={() => onRefresh("quick")}
        disabled={busy === "projects-quick" || busy === "projects-full"}
      >
        {busy === "projects-quick" ? "Scanning..." : "Quick scan"}
      </button>
      <button
        class="rounded-full bg-[color:var(--accent)] px-4 py-2 text-sm font-semibold text-slate-950 transition hover:scale-[1.01] disabled:opacity-60"
        onclick={() => onRefresh("full")}
        disabled={busy === "projects-quick" || busy === "projects-full"}
      >
        {busy === "projects-full" ? "Scanning..." : "Full scan"}
      </button>
    </div>
  </div>

  {#if projectIndex}
    <div class="grid gap-3 md:grid-cols-3">
      <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
        <p class="text-[color:var(--muted)]">Found</p>
        <p class="mt-2 text-2xl font-semibold">{projectIndex.totalFound}</p>
      </div>
      <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
        <p class="text-[color:var(--muted)]">Mode</p>
        <p class="mt-2 text-2xl font-semibold uppercase">{projectIndex.scannedMode}</p>
      </div>
      <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
        <p class="text-[color:var(--muted)]">Engine</p>
        <p class="mt-2 text-lg font-semibold uppercase">{projectIndex.engine}</p>
      </div>
    </div>
    <p class="mt-3 text-xs text-[color:var(--muted)]">Roots indexed: {projectIndex.roots.length}</p>
    {#if projectIndex.skippedRoots.length}
      <div class="mt-2 rounded-2xl border border-amber-300/40 bg-[color:rgba(255,216,124,0.12)] p-3 text-xs text-[color:var(--warn)]">
        Skipped {projectIndex.skippedRoots.length} root{projectIndex.skippedRoots.length === 1 ? "" : "s"} (permissions or hidden). {projectIndex.engine === "everything-es" ? "Everything" : "Fallback scan"} will re-attempt when permissions change.
      </div>
    {/if}

    <div class="mt-4 rounded-2xl border border-white/8 bg-white/4 p-3">
      <p class="text-xs uppercase tracking-[0.2em] text-[color:var(--muted)]">Scan roots</p>
      <div class="mt-2 space-y-1 text-xs leading-6 text-[color:var(--muted)]">
        {#each projectIndex.roots as root}
          <p class="mono break-all">{root}</p>
        {/each}
      </div>
    </div>

    <div class="mt-4 grid gap-2">
      {#if previewProjects.length}
        {#each previewProjects as project}
          <button
            class="rounded-2xl border border-white/8 bg-white/4 p-3 text-left transition hover:border-white/20 hover:bg-white/6"
            onclick={() => onOpenPath(project.path)}
          >
            <div class="flex flex-wrap items-center justify-between gap-3">
              <div class="min-w-0">
                <p class="truncate font-medium">{project.name}</p>
                <p class="mono truncate text-xs text-[color:var(--muted)]">{project.path}</p>
              </div>
              <div class="flex items-center gap-2 text-xs">
                <span class="rounded-full border border-white/10 bg-white/6 px-2 py-1 uppercase tracking-[0.14em]">{project.extension}</span>
                <span class="rounded-full border border-white/10 bg-white/6 px-2 py-1">{project.sizeMb} MB</span>
              </div>
            </div>
          </button>
        {/each}
      {:else}
        <div class="rounded-2xl border border-white/8 bg-white/4 p-4 text-[color:var(--muted)]">
          Run a scan to build the project index.
        </div>
      {/if}
    </div>
  {:else}
    <div class="rounded-2xl border border-white/8 bg-white/4 p-4 text-[color:var(--muted)]">
      Run a quick scan to index recent `.aep`, `.aepx`, and `.aet` files from common user
      locations, or a full scan to walk all detected drives.
    </div>
  {/if}
</div>
