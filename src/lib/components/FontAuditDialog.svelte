<script lang="ts">
  import type { FontAuditResult } from "$lib/types";
  import { scale, fade } from "svelte/transition";

  interface Props {
    result: FontAuditResult | null;
    loading: boolean;
    onClose: () => void;
  }

  let { result, loading, onClose }: Props = $props();
</script>

{#if result || loading}
  <div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-md p-4"
    transition:fade={{ duration: 200 }}
    onclick={(e) => e.target === e.currentTarget && onClose()}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === 'Escape' && onClose()}
  >
    <div 
      class="w-full max-w-2xl overflow-hidden rounded-[40px] border border-white/10 bg-[#0a0a0a] shadow-2xl flex flex-col max-h-[90vh]" 
      transition:scale={{ duration: 300, start: 0.95 }}
      onclick={(e) => e.stopPropagation()}
      role="none"
    >
      <!-- Header -->
      <div class="px-8 pt-8 pb-6 border-b border-white/5 bg-gradient-to-br from-indigo-500/5 to-transparent">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-2xl font-bold tracking-tight text-indigo-100">Typography Audit</h2>
            <p class="text-[10px] text-indigo-400/70 mono mt-1 uppercase tracking-widest font-semibold flex items-center gap-2">
                <span class="w-1.5 h-1.5 rounded-full bg-indigo-500 animate-pulse"></span>
                Binary Registry Analysis
            </p>
          </div>
          <button 
            class="rounded-full bg-white/5 p-2 hover:bg-white/10 transition active:scale-95"
            onclick={onClose}
            aria-label="Close dialog"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
          </button>
        </div>
        
        {#if result}
            <div class="mt-4 flex items-center gap-2 px-3 py-1.5 rounded-full bg-white/5 border border-white/5 w-fit">
               <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-(--muted)"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
               <p class="text-[10px] mono text-(--muted) truncate max-w-[400px]">{result.projectPath}</p>
            </div>
        {/if}
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-8 thin-scrollbar">
        {#if loading}
          <div class="flex flex-col items-center justify-center py-24 gap-6">
            <div class="relative">
                <div class="h-16 w-16 animate-spin rounded-full border-2 border-indigo-500/20 border-t-indigo-500"></div>
                <div class="absolute inset-0 flex items-center justify-center text-xs font-bold text-indigo-400 animate-pulse">AEP</div>
            </div>
            <div class="text-center space-y-1">
                <p class="font-bold text-indigo-100 italic">Intercepting Font Tokens</p>
                <p class="text-[10px] uppercase tracking-tighter text-(--muted)">Mapping project resources to system registry...</p>
            </div>
          </div>
        {:else if result}
          <div class="space-y-8">
            <!-- Stats -->
            <div class="grid grid-cols-3 gap-4">
                <div class="p-5 rounded-3xl border border-white/5 bg-white/2 relative overflow-hidden group">
                    <div class="absolute inset-0 bg-indigo-500/5 opacity-0 group-hover:opacity-100 transition"></div>
                    <span class="text-[10px] uppercase font-bold text-(--muted) tracking-widest block mb-2">Total</span>
                    <span class="text-3xl font-bold tracking-tighter">{result.fonts.length}</span>
                </div>
                <div class="p-5 rounded-3xl border border-white/5 bg-white/2 relative overflow-hidden group">
                    <div class="absolute inset-0 {result.missingCount > 0 ? 'bg-amber-500/5' : 'bg-emerald-500/5'} opacity-0 group-hover:opacity-100 transition"></div>
                    <span class="text-[10px] uppercase font-bold text-(--muted) tracking-widest block mb-2">Missing</span>
                    <span class="text-3xl font-bold tracking-tighter {result.missingCount > 0 ? 'text-amber-400' : 'text-emerald-400'}">{result.missingCount}</span>
                </div>
                <div class="p-5 rounded-3xl border border-white/5 bg-white/2 relative overflow-hidden group">
                    <div class="absolute inset-0 bg-indigo-500/5 opacity-0 group-hover:opacity-100 transition"></div>
                    <span class="text-[10px] uppercase font-bold text-(--muted) tracking-widest block mb-2">Health</span>
                    <span class="text-3xl font-bold tracking-tighter">{Math.round(((result.fonts.length - result.missingCount) / (result.fonts.length || 1)) * 100)}%</span>
                </div>
            </div>

            <!-- List -->
            <div class="space-y-3">
              <h3 class="text-xs font-bold uppercase tracking-widest text-(--muted) ml-1 mb-4 flex items-center gap-2">
                  Project Font Map
                  <span class="h-px flex-1 bg-white/5"></span>
              </h3>
              {#each result.fonts as font}
                <div class="flex items-center justify-between p-4 rounded-2xl border border-white/5 bg-white/2 hover:bg-white/5 transition-all group active:scale-[0.99]">
                  <div class="flex items-center gap-4">
                    <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-white/5 border border-white/10 font-serif font-bold text-xl text-indigo-400 group-hover:scale-110 transition shadow-inner">
                      {font.family[0] || '?'}
                    </div>
                    <div>
                      <p class="font-bold text-sm tracking-tight">{font.family}</p>
                      <p class="text-[9px] uppercase tracking-[0.2em] font-bold text-(--muted) mt-0.5">{font.style || 'Regular'}</p>
                    </div>
                  </div>
                  
                  {#if font.isInstalled}
                    <div class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-emerald-500/5 border border-emerald-500/20 text-[10px] font-bold text-emerald-400 uppercase tracking-widest">
                      <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
                      Linked
                    </div>
                  {:else}
                    <div class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-amber-500/5 border border-amber-500/20 text-[10px] font-bold text-amber-400 uppercase tracking-widest">
                      <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>
                      Missing
                    </div>
                  {/if}
                </div>
              {:else}
                <div class="flex flex-col items-center justify-center p-12 text-center border-2 border-dashed border-white/5 rounded-[32px] opacity-40">
                  <p class="text-sm font-bold italic">No Typography DNA Detected</p>
                  <p class="text-[10px] mt-1 uppercase tracking-widest italic opacity-60">Binary project parsing completed with 0 font tokens.</p>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="p-8 border-t border-white/5 bg-white/2 flex items-center justify-end">
        <button 
          class="group relative px-12 py-3 bg-emerald-500 rounded-2xl font-bold text-white overflow-hidden transition-all hover:scale-[1.02] hover:shadow-2xl hover:shadow-emerald-500/40 active:scale-[0.98]"
          onclick={onClose}
        >
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/20 to-white/0 -translate-x-full group-hover:animate-shimmer"></div>
          <span class="relative z-10 uppercase tracking-widest text-[11px]">Conclude Audit</span>
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
