<script lang="ts">
  import type { ExpressionAuditResult, ExpressionError } from "$lib/types";
  import { fade, scale } from "svelte/transition";

  interface Props {
    result: ExpressionAuditResult | null;
    logs: ExpressionError[];
    loading: boolean;
    onClose: () => void;
  }

  let { result, logs, loading, onClose }: Props = $props();

  let activeTab = $state<"scan" | "history">("scan");

  const displayErrors = $derived.by(() => {
    if (activeTab === "scan") return result?.errors ?? [];
    return logs;
  });

  const riskyCount = $derived(result?.riskyCount ?? 0);
</script>

{#if result || loading || logs.length > 0}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4 backdrop-blur-md"
    transition:fade={{ duration: 200 }}
    onclick={(e) => e.target === e.currentTarget && onClose()}
    onkeydown={(e) => e.key === 'Escape' && onClose()}
    role="button"
    tabindex="0"
  >
    <div
      class="w-full max-w-4xl h-[85vh] overflow-hidden rounded-[40px] flex flex-col shadow-2xl border border-white/10 bg-[#0a0a0a]"
      transition:scale={{ duration: 300, start: 0.95 }}
      onclick={(e) => e.stopPropagation()}
      role="none"
    >
      <!-- Header -->
      <div class="px-8 pt-8 pb-4 border-b border-white/5 bg-gradient-to-br from-rose-500/5 to-transparent flex items-center justify-between">
        <div>
          <h2 class="text-3xl font-extrabold tracking-tight text-rose-50">Linguistics Diagnostic</h2>
          <p class="mt-2 text-[10px] text-rose-400/70 mono uppercase tracking-widest font-semibold flex items-center gap-2">
            <span class="w-1.5 h-1.5 rounded-full bg-rose-500 animate-pulse"></span>
            Expression Runtime Core
          </p>
          {#if result}
            <div class="mt-4 flex items-center gap-2 px-3 py-1.5 rounded-full bg-white/5 border border-white/5 w-fit">
               <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-(--muted)"><path d="M22 12s-3-7-10-7-10 7-10 7 3 7 10 7 10-7 10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
               <p class="text-[10px] mono text-(--muted) truncate max-w-[500px]">{result.projectPath || "Global Registry Scan"}</p>
            </div>
          {/if}
        </div>
        <button 
          class="rounded-full bg-white/5 p-3 hover:bg-white/10 transition active:scale-95 text-(--muted) hover:text-white"
          onclick={onClose}
          aria-label="Close diagnostic"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      </div>

      <!-- Tab Switcher (Modern Style) -->
      <div class="px-8 flex p-1.5 bg-white/5 rounded-none border-b border-white/5">
        <div class="flex p-1 bg-white/4 rounded-[20px] w-fit">
            <button 
              class="px-8 py-3 text-xs font-bold rounded-[16px] transition-all flex items-center gap-2 {activeTab === 'scan' ? 'bg-rose-500 text-slate-950 shadow-lg shadow-rose-500/20' : 'text-(--muted) hover:text-white'}"
              onclick={() => activeTab = 'scan'}
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
              Project Scan
            </button>
            <button 
              class="px-8 py-3 text-xs font-bold rounded-[16px] transition-all flex items-center gap-2 {activeTab === 'history' ? 'bg-rose-500 text-slate-950 shadow-lg shadow-rose-500/20' : 'text-(--muted) hover:text-white'}"
              onclick={() => activeTab = 'history'}
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M12 8v4l3 3"/><circle cx="12" cy="12" r="10"/></svg>
              Runtime Logs ({logs.length})
            </button>
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-8 thin-scrollbar space-y-8">
        {#if loading}
          <div class="flex flex-col items-center justify-center py-28 gap-6 text-center">
            <div class="relative">
                <div class="h-20 w-20 animate-spin rounded-full border-2 border-rose-500/20 border-t-rose-500"></div>
                <div class="absolute inset-0 flex items-center justify-center text-xs font-bold text-rose-400 :global(animate-shimmer)">ESLINT</div>
            </div>
            <div>
                <p class="font-bold text-rose-50 text-lg">Parsing Script Architecture</p>
                <p class="text-[10px] uppercase tracking-widest text-(--muted) opacity-60">Scanning compositions for evaluation bottlenecks and runtime crashes...</p>
            </div>
          </div>
        {:else}
          {#if activeTab === "scan"}
            <!-- Stats Grid -->
            <div class="grid grid-cols-2 gap-6">
              <div class="p-6 rounded-[32px] border border-white/5 bg-white/2 relative overflow-hidden group">
                <div class="absolute inset-0 {(result?.errors.length ?? 0) > 0 ? 'bg-rose-500/5' : 'bg-emerald-500/5'} opacity-0 group-hover:opacity-100 transition"></div>
                <span class="text-[11px] uppercase font-bold text-(--muted) tracking-widest block mb-1">Identified Criticals</span>
                <span class="text-4xl font-extrabold tracking-tighter {(result?.errors.length ?? 0) > 0 ? 'text-rose-400 animate-pulse' : 'text-emerald-400'}">
                  {result?.errors.length ?? 0}
                </span>
                <div class="mt-4 flex flex-wrap gap-2">
                  <span class="px-3 py-1 bg-white/5 rounded-full text-[9px] font-bold uppercase tracking-wider text-rose-300">Hard-fail Logic</span>
                  <span class="px-3 py-1 bg-white/5 rounded-full text-[9px] font-bold uppercase tracking-wider text-rose-300">Reference Errors</span>
                </div>
              </div>
              <div class="p-6 rounded-[32px] border border-white/5 bg-white/2 relative overflow-hidden group">
                 <div class="absolute inset-0 {riskyCount > 0 ? 'bg-amber-500/5' : 'bg-emerald-500/5'} opacity-0 group-hover:opacity-100 transition"></div>
                <span class="text-[11px] uppercase font-bold text-(--muted) tracking-widest block mb-1">Volatile Fragments</span>
                <span class="text-4xl font-extrabold tracking-tighter {riskyCount > 0 ? 'text-amber-400' : 'text-emerald-400'}">
                  {riskyCount}
                </span>
                <p class="mt-4 text-[10px] italic text-(--muted) leading-relaxed">Hardcoded layer strings and dynamic index references detected.</p>
              </div>
            </div>
          {/if}

          <!-- Diagnostic Feed -->
          <div class="space-y-6">
            <h3 class="text-xs font-bold uppercase tracking-widest text-(--muted) ml-1 flex items-center gap-3">
              {activeTab === "scan" ? "Diagnostic Feed" : "System Log Feed"}
              <span class="h-px flex-1 bg-white/5"></span>
            </h3>
            
            {#if displayErrors.length > 0}
              <div class="grid gap-4">
                {#each displayErrors as error}
                  <div class="group rounded-[32px] border border-white/5 bg-white/2 p-6 hover:bg-white/5 transition-all active:scale-[0.99] relative overflow-hidden">
                    <div class="absolute top-0 right-0 w-32 h-32 bg-rose-500/5 rounded-full -translate-y-16 translate-x-16 group-hover:scale-150 transition-all duration-500"></div>
                    
                    <div class="flex flex-col gap-5 relative z-10">
                      <div class="flex flex-wrap items-center justify-between gap-3">
                        <div class="flex items-center gap-3">
                          <span class="rounded-lg bg-rose-500 px-3 py-1 text-[10px] font-bold uppercase tracking-widest text-slate-950 shadow-lg shadow-rose-500/20">ERROR</span>
                          <span class="text-[11px] font-bold mono text-rose-300/60 uppercase">{error.timestamp}</span>
                        </div>
                        {#if error.version}
                            <span class="px-3 py-1 border border-white/10 rounded-lg text-[10px] font-bold mono text-(--muted)">ENGINE: V{error.version}</span>
                        {/if}
                      </div>

                      <div class="space-y-2">
                        <h4 class="text-xl font-bold tracking-tight text-white group-hover:text-rose-100 transition-colors uppercase">{error.composition || 'Main Framework'}</h4>
                        <div class="flex gap-x-6 gap-y-2 flex-wrap text-xs font-semibold text-(--muted)">
                           <div class="flex gap-2 items-center">
                              <span class="opacity-30">LAYER:</span> <span class="text-rose-300/80">{error.layer}</span>
                           </div>
                           <div class="flex gap-2 items-center">
                              <span class="opacity-30">PROPERTY:</span> <span class="text-rose-300/80">{error.property}</span>
                           </div>
                        </div>
                      </div>

                      <div class="rounded-2xl bg-black/60 p-5 border border-white/5 shadow-inner">
                        <div class="flex items-start gap-4">
                           <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="mt-1 text-rose-500"><path d="m21 21-4.3-4.3"/><circle cx="11" cy="11" r="8"/><line x1="11" y1="8" x2="11" y2="12"/><line x1="11" y1="16" x2="11.01" y2="16"/></svg>
                           <p class="mono text-[12px] leading-relaxed text-rose-100/90 whitespace-pre-wrap">{error.message}</p>
                        </div>
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <div class="rounded-[40px] border-2 border-dashed border-white/5 py-24 text-center space-y-6">
                <div class="relative mx-auto flex h-20 w-20 items-center justify-center">
                    <div class="absolute inset-0 bg-emerald-500/20 rounded-full animate-pulse"></div>
                    <div class="relative flex h-16 w-16 items-center justify-center rounded-full bg-emerald-500 text-slate-950 shadow-2xl">
                        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
                    </div>
                </div>
                <div class="space-y-1">
                    <h4 class="text-2xl font-bold tracking-tight text-white">System Integrity Optimal</h4>
                    <p class="text-xs uppercase font-bold tracking-widest text-emerald-400">0 Faults detected in {activeTab === "scan" ? 'project DNA' : 'runtime sequence'}.</p>
                </div>
                <p class="text-[10px] text-(--muted) italic max-w-xs mx-auto">No evaluation failures detected in the active memory buffer.</p>
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="p-8 border-t border-white/5 bg-white/2 flex items-center justify-end">
        <button 
          class="group relative px-12 py-3 bg-rose-500 rounded-2xl font-bold text-white overflow-hidden transition-all hover:scale-[1.02] hover:shadow-2xl hover:shadow-rose-500/40 active:scale-[0.98]"
          onclick={onClose}
        >
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/20 to-white/0 -translate-x-full group-hover:animate-shimmer"></div>
          <span class="relative z-10 uppercase tracking-widest text-[11px]">Conclude Diagnostic</span>
        </button>
      </div>
    </div>
  </div>
{/if}

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
