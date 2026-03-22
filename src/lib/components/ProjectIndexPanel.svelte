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
    onRunAerender: (path: string, name: string) => void;
    onOpenPlugins: (path: string, plugins: string[]) => void;
  }

  let { projectIndex, busy, onRefresh, onOpenPath, onDownConvert, onPurgeAutoSaves, onAuditFonts, onAuditExpressions, onRunAerender, onOpenPlugins }: Props = $props();
  const previewProjects = $derived.by(() => projectIndex?.projects.slice(0, 12) ?? []);

  function formatSize(sizeMb: number) {
    if (sizeMb >= 1048576) {
      return { value: (sizeMb / 1048576).toFixed(2), unit: 'TB' };
    }
    if (sizeMb >= 1024) {
      return { value: (sizeMb / 1024).toFixed(1), unit: 'GB' };
    }
    return { value: Math.round(sizeMb).toString(), unit: 'MB' };
  }

  function formatDate(iso: string) {
    const d = new Date(iso);
    return d.toLocaleDateString('en-GB', { 
      day: '2-digit', 
      month: 'short', 
      year: 'numeric' 
    });
  }
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8">
  <!-- Header Section -->
  <div class="mb-6 flex flex-wrap items-start justify-between gap-4 border-b border-white/5 pb-6">
    <div>
      <div class="flex items-center gap-2 mb-2">
         <span class="mono text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) opacity-80">Project Index Manager</span>
      </div>
      <h2 class="text-2xl font-bold tracking-tight">System-wide After Effects Projects</h2>
      {#if projectIndex}
        <p class="mt-2 text-[11px] font-medium leading-relaxed text-(--muted) max-w-lg">
          Detects .aep, .aepx, and .aet files <span class="text-white">{projectIndex.scannedMode === "full" ? "across all mounted volumes" : "in common user directories"}</span> utilizing 
          <span class="text-white">{projectIndex.engine === "everything-es" ? "Everything (es.exe) index" : "the fallback filesystem walker"}</span> for optimal speed.
        </p>
      {/if}
    </div>
    
    <div class="flex flex-wrap gap-2">
      <button
        class="group relative overflow-hidden rounded-[16px] border border-white/10 bg-white/5 px-5 py-3 text-[10px] font-bold uppercase tracking-widest transition-all hover:bg-white/10 hover:border-white/20 active:scale-[0.98] disabled:opacity-40"
        onclick={() => onRefresh("quick")}
        disabled={busy === "projects-quick" || busy === "projects-full"}
      >
        <span class="relative z-10 flex items-center gap-2">
           <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class={busy === "projects-quick" ? "animate-spin" : ""}><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.92-10.44l5.36 5.36"/></svg>
           {busy === "projects-quick" ? "Scanning..." : "Quick Scan"}
        </span>
      </button>
      <button
        class="group relative overflow-hidden rounded-[16px] bg-indigo-500 px-6 py-3 text-[10px] font-bold uppercase tracking-widest text-slate-950 transition-all hover:scale-[1.02] hover:shadow-lg hover:shadow-indigo-500/20 active:scale-[0.98] disabled:opacity-40 disabled:hover:scale-100"
        onclick={() => onRefresh("full")}
        disabled={busy === "projects-quick" || busy === "projects-full"}
      >
         <div class="absolute inset-0 -translate-x-full bg-linear-to-r from-transparent via-white/30 to-transparent group-hover:animate-shimmer"></div>
        <span class="relative z-10 flex items-center gap-2">
           <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class={busy === "projects-full" ? "animate-spin" : ""}><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.92-10.44l5.36 5.36"/></svg>
           {busy === "projects-full" ? "Deep Scanning..." : "Full System Scan"}
        </span>
      </button>
    </div>
  </div>

  {#if projectIndex}
    <!-- Scan Stats -->
    <div class="grid gap-4 md:grid-cols-3 mb-6">
      <div class="rounded-[20px] border border-white/5 bg-white/2 p-5 relative overflow-hidden group">
        <div class="absolute inset-0 bg-indigo-500/5 opacity-0 group-hover:opacity-100 transition duration-300"></div>
        <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) mb-1">Total Found</p>
        <p class="text-3xl font-black tracking-tighter text-indigo-400">{projectIndex.totalFound}</p>
      </div>
      <div class="rounded-[20px] border border-white/5 bg-white/2 p-5 relative overflow-hidden group">
        <div class="absolute inset-0 bg-white/5 opacity-0 group-hover:opacity-100 transition duration-300"></div>
        <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) mb-1">Search Matrix</p>
        <p class="text-2xl font-black tracking-tighter uppercase text-white">{projectIndex.scannedMode}</p>
      </div>
      <div class="rounded-[20px] border border-white/5 bg-white/2 p-5 relative overflow-hidden group">
        <div class="absolute inset-0 bg-emerald-500/5 opacity-0 group-hover:opacity-100 transition duration-300"></div>
        <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) mb-1">Core Engine</p>
        <p class="text-xl font-black tracking-tighter uppercase {projectIndex.engine === 'everything-es' ? 'text-emerald-400' : 'text-amber-400'}">{projectIndex.engine}</p>
      </div>
    </div>

    <!-- Roots Information -->
    <div class="flex flex-col gap-3 mb-8">
       <div class="flex items-center gap-3">
         <span class="rounded-full border border-white/5 bg-white/5 px-2.5 py-1 text-[9px] font-bold uppercase tracking-[0.2em] text-(--muted)">
           Roots Indexed: {projectIndex.roots.length}
         </span>
       </div>
       {#if projectIndex.skippedRoots.length}
         <div class="flex items-start gap-3 rounded-[16px] border border-amber-500/20 bg-amber-500/5 p-4 shadow-inner">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="mt-0.5 text-amber-500 shrink-0"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
            <div>
               <p class="text-[10px] font-bold uppercase tracking-widest text-amber-500/80 mb-1">Access Restricted</p>
               <p class="text-xs text-amber-100/80 leading-relaxed font-medium">Skipped {projectIndex.skippedRoots.length} root(s) due to permissions or hidden attributes. {projectIndex.engine === "everything-es" ? "The Everything engine" : "Fallback discovery"} automatically re-attempts when system permissions shift.</p>
            </div>
         </div>
       {/if}
       <div class="rounded-[20px] border border-white/5 bg-black/40 p-4 mt-2">
         <p class="text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted) mb-2">Scan Origins</p>
         <div class="space-y-1">
           {#each projectIndex.roots as root}
             <p class="mono text-[10px] text-(--muted) truncate hover:text-white transition cursor-default" title={root}>{root}</p>
           {/each}
         </div>
       </div>
    </div>

    <!-- Project List -->
    <div class="grid gap-3 max-h-[700px] overflow-y-auto pr-2 thin-scrollbar">
      {#if previewProjects.length}
         <div class="flex items-center gap-3 mb-1">
            <span class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted)">Latest Results</span>
            <div class="h-px flex-1 bg-white/5"></div>
         </div>
        {#each previewProjects as project}
          <div class="group/row relative overflow-hidden rounded-[24px] border border-transparent bg-white/2 hover:border-white/10 hover:bg-white/4 p-4 md:p-5 transition-all outline-1 outline-transparent hover:outline-white/5">
            
            <div class="flex flex-col gap-5 relative z-10">
               <!-- Top Identifiers -->
               <div class="flex items-start justify-between gap-4">
                  <button 
                    class="min-w-0 flex-1 text-left focus:outline-none"
                    onclick={() => onOpenPath(project.path)}
                  >
                    <div class="flex items-center gap-3 mb-1.5">
                      <p class="truncate text-lg font-bold text-white group-hover/row:text-indigo-300 transition-colors">{project.name}</p>
                      <span class="rounded-[8px] border border-white/5 bg-white/5 px-2 py-0.5 text-[9px] font-bold uppercase tracking-[0.15em] text-(--muted)">{project.extension}</span>
                    </div>
                    <p class="mono truncate text-[10px] text-(--muted) opacity-70 group-hover/row:opacity-100 transition">{project.path}</p>
                  </button>
                  
                  <div class="flex flex-col items-end gap-1 shrink-0 text-right">
                     <p class="text-sm font-bold text-white font-mono">{formatSize(project.sizeMb).value} <span class="text-[10px] font-sans tracking-widest text-(--muted)">{formatSize(project.sizeMb).unit}</span></p>
                     <p class="text-[10px] text-(--muted) font-medium">{formatDate(project.modified)}</p>
                  </div>
               </div>

               <!-- Properties -->
               {#if project.width || project.height || project.fps || (project.plugins && project.plugins.length > 0) || project.missingFootage > 0}
                 <div class="flex flex-wrap items-center gap-x-6 gap-y-3">
                   <div class="flex items-center gap-4">
                     {#if project.width && project.height}
                       <div class="flex items-center gap-2">
                         <span class="text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted)">Res</span>
                         <span class="text-[11px] font-bold mono text-white">{project.width}×{project.height}</span>
                       </div>
                     {/if}
                     
                     {#if project.fps}
                       <div class="flex items-center gap-2">
                         <span class="text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted)">FPS</span>
                         <span class="text-[11px] font-bold mono text-white">{project.fps}</span>
                       </div>
                     {/if}
                   </div>

                   {#if project.plugins && project.plugins.length > 0}
                     <div class="flex flex-wrap items-center gap-1.5">
                       <span class="text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted) mr-1">Modules</span>
                       {#each project.plugins as plugin}
                         <span class="rounded-[8px] border border-indigo-500/20 bg-indigo-500/5 px-2 py-0.5 text-[9px] font-bold tracking-widest text-indigo-300">
                           {plugin}
                         </span>
                       {/each}
                     </div>
                   {/if}

                   {#if project.missingFootage > 0}
                      <div class="flex items-center gap-1.5 rounded-[8px] border border-rose-500/20 bg-rose-500/5 px-2.5 py-1">
                         <span class="h-1.5 w-1.5 animate-pulse rounded-full bg-rose-500"></span>
                         <span class="text-[9px] font-bold uppercase tracking-wider text-rose-400">
                           {project.missingFootage} Missing
                         </span>
                      </div>
                   {/if}
                 </div>
               {/if}

               <!-- Action Bar -->
               <div class="flex flex-wrap items-center justify-between gap-4 border-t border-white/5 pt-4 mt-1">
                 <div class="flex flex-wrap items-center gap-2">
                    <!-- Analytics Actions -->
                    <button 
                      class="rounded-[12px] border border-white/5 bg-white/2 px-4 py-2 text-[9px] font-bold uppercase tracking-[0.15em] transition-all hover:bg-emerald-500/10 hover:border-emerald-500/30 hover:text-emerald-400 disabled:opacity-40 {project.autoSaveCount > 0 ? 'text-white' : 'text-(--muted)'}"
                      onclick={() => onPurgeAutoSaves(project.path)}
                      disabled={project.autoSaveCount === 0 || busy === `purge-as-${project.path}`}
                    >
                      {#if project.autoSaveCount > 0}
                         Purge {project.autoSaveCount} Auto-Saves
                      {:else}
                         No Auto-Saves
                      {/if}
                    </button>
                    
                    <button 
                      class="rounded-[12px] border border-white/5 bg-white/2 px-4 py-2 text-[9px] font-bold uppercase tracking-[0.15em] text-(--muted) transition-all hover:bg-indigo-500/10 hover:border-indigo-500/30 hover:text-indigo-400 disabled:opacity-40"
                      onclick={() => onAuditFonts(project.path)}
                      disabled={busy === `audit-as-${project.path}`}
                    >
                      Audit Fonts
                    </button>
                    
                    <button 
                      class="rounded-[12px] border border-white/5 bg-white/2 px-4 py-2 text-[9px] font-bold uppercase tracking-[0.15em] text-(--muted) transition-all hover:bg-rose-500/10 hover:border-rose-500/30 hover:text-rose-400 disabled:opacity-40"
                      onclick={() => onAuditExpressions(project.path)}
                      disabled={busy === `audit-expressions-${project.path}`}
                    >
                      Scanner
                    </button>
                    
                    <button 
                      class="rounded-[12px] border border-white/5 bg-white/2 px-4 py-2 text-[9px] font-bold uppercase tracking-[0.15em] text-(--muted) transition-all hover:bg-amber-500/10 hover:border-amber-500/30 hover:text-amber-400 disabled:opacity-40"
                      onclick={() => onOpenPlugins(project.path, project.plugins)}
                      disabled={!project.plugins || project.plugins.length === 0}
                    >
                      Plugins ({project.plugins?.length ?? 0})
                    </button>
                 </div>

                 <div class="flex items-center gap-3">
                    <!-- Save As -->
                    <div class="hidden md:flex items-center gap-1.5 border-l border-white/5 pl-4">
                       <span class="text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted) mr-2 opacity-60">Convert</span>
                       {#each ["2023", "2022", "2020"] as ver}
                        <button
                          class="rounded-[8px] border border-white/5 bg-white/5 px-2 py-1 text-[9px] font-bold text-(--muted) transition-all hover:bg-white/10 hover:text-white disabled:opacity-40"
                          onclick={() => onDownConvert(project.path, ver)}
                          disabled={busy === `convert-${project.path}`}
                        >
                          {ver}
                        </button>
                       {/each}
                    </div>

                    <!-- Render Trigger -->
                    <button 
                      class="group relative overflow-hidden rounded-[16px] bg-emerald-500 px-6 py-2.5 text-[10px] font-bold uppercase tracking-[0.2em] text-slate-950 transition-all hover:scale-[1.02] hover:shadow-lg hover:shadow-emerald-500/20 active:scale-[0.98] disabled:opacity-40 disabled:hover:scale-100 ml-2"
                      onclick={() => {
                        onRunAerender(project.path, project.name);
                      }}
                      disabled={busy === `render-${project.path}`}
                    >
                      <div class="absolute inset-0 -translate-x-full bg-linear-to-r from-transparent via-white/30 to-transparent group-hover:animate-shimmer"></div>
                      <span class="relative z-10 flex items-center gap-2">
                        {#if busy === `render-${project.path}`}
                          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" class="animate-spin"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.92-10.44l5.36 5.36"/></svg>
                          Starting
                        {:else}
                          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><polygon points="5 3 19 12 5 21 5 3"/></svg>
                          Orchestrator
                        {/if}
                      </span>
                    </button>
                 </div>
               </div>
            </div>
          </div>
        {/each}
      {:else}
        <div class="flex flex-col items-center justify-center rounded-[32px] border-2 border-dashed border-white/5 bg-white/2 py-16 text-center opacity-70 mt-4">
           <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-(--muted) mb-3"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><line x1="10" y1="9" x2="8" y2="9"/></svg>
           <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-white">Index Empty</p>
           <p class="text-[10px] mt-1.5 text-(--muted) font-medium max-w-[200px]">Initiate a scan to populate the project matrix.</p>
        </div>
      {/if}
    </div>
  {:else}
    <div class="flex flex-col items-center justify-center rounded-[32px] border-2 border-dashed border-white/5 bg-white/2 py-20 text-center opacity-70">
       <div class="h-12 w-12 rounded-full bg-indigo-500/10 flex items-center justify-center mb-4 text-indigo-400">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
       </div>
       <p class="text-xs uppercase font-bold tracking-widest text-white">System Index Offline</p>
       <p class="text-[10px] mt-2 text-(--muted) font-medium max-w-[280px] leading-relaxed">Execute a Quick Scan to index recent projects from common locations, or a Full Scan for a comprehensive deep-drive walk.</p>
    </div>
  {/if}
</div>

<style>
  .thin-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .thin-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .thin-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }
  .thin-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  @keyframes shimmer {
    100% {
      transform: translateX(100%);
    }
  }

  :global(.animate-shimmer) {
    animation: shimmer 1.5s infinite;
  }
</style>
