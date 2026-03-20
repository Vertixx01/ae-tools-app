<script lang="ts">
  import type { ProjectIndexSnapshot } from "$lib/types";

  interface Props {
    projectIndex: ProjectIndexSnapshot | null;
    busy: string | null;
    onRefresh: (mode: "quick" | "full") => void;
    onOpenPath: (path: string | null) => void;
    onDownConvert: (path: string, version: string) => void;
    onPurgeAutoSaves: (path: string) => void;
    onAuditFonts: (path: string) => void;
    onAuditExpressions: (path: string) => void;
  }

  let { projectIndex, busy, onRefresh, onOpenPath, onDownConvert, onPurgeAutoSaves, onAuditFonts, onAuditExpressions }: Props = $props();
  const previewProjects = $derived.by(() => projectIndex?.projects.slice(0, 12) ?? []);
</script>

<div class="panel rounded-[28px] p-4 md:p-5">
    <div class="mb-4 flex flex-wrap items-center justify-between gap-4">
      <div>
        <p class="mono text-[11px] uppercase tracking-[0.24em] text-(--muted)">
          Project index manager
        </p>
        <h2 class="mt-2 text-2xl font-semibold">Auto-scan AE projects across the PC</h2>
        {#if projectIndex}
          <p class="text-xs text-(--muted)">
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
        class="rounded-full bg-(--accent) px-4 py-2 text-sm font-semibold text-slate-950 transition hover:scale-[1.01] disabled:opacity-60"
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
        <p class="text-(--muted)">Found</p>
        <p class="mt-2 text-2xl font-semibold">{projectIndex.totalFound}</p>
      </div>
      <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
        <p class="text-(--muted)">Mode</p>
        <p class="mt-2 text-2xl font-semibold uppercase">{projectIndex.scannedMode}</p>
      </div>
      <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
        <p class="text-(--muted)">Engine</p>
        <p class="mt-2 text-lg font-semibold uppercase">{projectIndex.engine}</p>
      </div>
    </div>
    <p class="mt-3 text-xs text-(--muted)">Roots indexed: {projectIndex.roots.length}</p>
    {#if projectIndex.skippedRoots.length}
      <div class="mt-2 rounded-2xl border border-amber-300/40 bg-[rgba(255,216,124,0.12)] p-3 text-xs text-(--warn)">
        Skipped {projectIndex.skippedRoots.length} root{projectIndex.skippedRoots.length === 1 ? "" : "s"} (permissions or hidden). {projectIndex.engine === "everything-es" ? "Everything" : "Fallback scan"} will re-attempt when permissions change.
      </div>
    {/if}

    <div class="mt-4 rounded-2xl border border-white/8 bg-white/4 p-3">
      <p class="text-xs uppercase tracking-[0.2em] text-(--muted)">Scan roots</p>
      <div class="mt-2 space-y-1 text-xs leading-6 text-(--muted)">
        {#each projectIndex.roots as root}
          <p class="mono break-all">{root}</p>
        {/each}
      </div>
    </div>

    <div class="mt-3 grid gap-2 max-h-[800px] overflow-y-auto pr-1">
      {#if previewProjects.length}
        {#each previewProjects as project}
          <div
            class="group relative rounded-2xl border border-white/8 bg-white/4 p-3 transition hover:border-white/20 hover:bg-white/6"
          >
            <div class="flex flex-wrap items-center justify-between gap-4">
              <button 
                class="min-w-0 flex-1 text-left"
                onclick={() => onOpenPath(project.path)}
              >
                <div class="flex items-center gap-2">
                  <p class="truncate font-medium">{project.name}</p>
                  <span class="rounded-full bg-white/10 px-1.5 py-0.5 text-[10px] uppercase tracking-wider text-(--muted)">{project.extension}</span>
                </div>
                <p class="mono truncate text-xs text-(--muted)">{project.path}</p>
              </button>
              
                <div class="flex items-center gap-3">
                <div class="flex flex-col items-end text-xs text-(--muted)">
                   <p>{project.sizeMb} MB</p>
                   <p class="opacity-60">{new Date(project.modified).toLocaleDateString()}</p>
                </div>
                
                <div class="h-8 w-px bg-white/10"></div>

                {#if project.autoSaveCount > 0}
                  <div class="flex flex-col items-end gap-1.5 text-xs">
                    <div class="flex items-center gap-1.5 text-(--accent)">
                      <span class="font-bold">{project.autoSaveCount}</span>
                      <span class="opacity-70 uppercase text-[9px] tracking-wider">A-S</span>
                    </div>
                    <div class="flex items-center gap-2">
                      <button 
                        class="text-[10px] uppercase font-bold text-(--danger) hover:underline disabled:opacity-50"
                        onclick={() => onPurgeAutoSaves(project.path)}
                        disabled={busy === `purge-as-${project.path}`}
                      >
                        Purge {project.autoSaveSizeMb}M
                      </button>
                      <button 
                        class="text-[10px] uppercase font-bold text-(--accent) hover:underline disabled:opacity-50"
                        onclick={() => onAuditFonts(project.path)}
                        disabled={busy === `audit-as-${project.path}`}
                      >
                        Fonts
                      </button>
                      <button 
                        class="text-[10px] uppercase font-bold text-(--accent) hover:underline disabled:opacity-50"
                        onclick={() => onAuditExpressions(project.path)}
                        disabled={busy === `audit-expressions-${project.path}`}
                      >
                        Expr
                      </button>
                    </div>
                  </div>
                  <div class="h-6 w-px bg-white/10"></div>
                {/if}
                
                <div class="flex items-center gap-1.5">
                   <span class="text-[10px] uppercase tracking-wider text-(--muted) mr-1">Save as</span>
                   {#each ["2023", "2022", "2020"] as ver}
                    <button
                      class="rounded-full border border-white/10 bg-white/6 px-2 py-1 text-[10px] font-bold transition hover:bg-(--accent) hover:text-slate-950 disabled:opacity-50"
                      onclick={() => onDownConvert(project.path, ver)}
                      disabled={busy === `convert-${project.path}`}
                    >
                      {ver}
                    </button>
                   {/each}
                </div>
              </div>
            </div>
          </div>
        {/each}
      {:else}
        <div class="rounded-2xl border border-white/8 bg-white/4 p-4 text-(--muted)">
          Run a scan to build the project index.
        </div>
      {/if}
    </div>
  {:else}
    <div class="rounded-2xl border border-white/8 bg-white/4 p-4 text-(--muted)">
      Run a quick scan to index recent `.aep`, `.aepx`, and `.aet` files from common user
      locations, or a full scan to walk all detected drives.
    </div>
  {/if}
</div>
