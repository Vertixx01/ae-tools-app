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
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4"
    transition:fade={{ duration: 200 }}
    onclick={onClose}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === 'Escape' && onClose()}
  >
    <div 
      class="w-full max-w-2xl overflow-hidden rounded-[32px] border border-white/10 bg-[#121212] shadow-2xl" 
      transition:scale={{ duration: 300, start: 0.95 }}
      onclick={(e) => e.stopPropagation()}
      role="none"
    >
      <div class="flex items-center justify-between border-b border-white/5 bg-white/2 px-6 py-5">
        <div>
          <h2 class="text-xl font-bold">Font Usage Audit</h2>
          <p class="mt-1 text-xs text-(--muted) truncate max-w-md">
            {result?.projectPath ?? 'Analyzing project...'}
          </p>
        </div>
        <button 
          class="rounded-full p-2 hover:bg-white/5 transition"
          onclick={onClose}
          aria-label="Close"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      </div>

      <div class="max-h-[60vh] overflow-y-auto p-6 thin-scrollbar">
        {#if loading}
          <div class="flex flex-col items-center justify-center py-12 gap-4">
            <div class="h-10 w-10 animate-spin rounded-full border-4 border-(--accent)/20 border-t-(--accent)"></div>
            <p class="text-sm animate-pulse text-(--muted)">Analyzing binary markers and system font registry...</p>
          </div>
        {:else if result}
          <div class="grid gap-6">
            <div class="flex items-center justify-between rounded-[24px] bg-white/3 p-5">
               <div class="grid gap-1">
                 <span class="text-xs uppercase tracking-wider text-(--muted)">Fonts Found</span>
                 <span class="text-2xl font-bold">{result.fonts.length}</span>
               </div>
               <div class="h-10 w-px bg-white/10"></div>
               <div class="grid gap-1">
                 <span class="text-xs uppercase tracking-wider text-(--muted)">Missing</span>
                 <span class="text-2xl font-bold {result.missingCount > 0 ? 'text-(--warn)' : 'text-(--accent)'}">{result.missingCount}</span>
               </div>
               <div class="h-10 w-px bg-white/10"></div>
               <div class="grid gap-1">
                 <span class="text-xs uppercase tracking-wider text-(--muted)">Health</span>
                 <span class="text-2xl font-bold">{Math.round(((result.fonts.length - result.missingCount) / (result.fonts.length || 1)) * 100)}%</span>
               </div>
            </div>

            <div class="space-y-3">
              {#each result.fonts as font}
                <div class="flex items-center justify-between rounded-2xl border border-white/5 bg-white/2 px-4 py-3 transition hover:bg-white/4">
                  <div class="flex items-center gap-3">
                    <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-white/5 font-serif font-bold text-lg select-none border border-white/5">
                      {font.family[0] || '?'}
                    </div>
                    <div>
                      <p class="font-medium">{font.family}</p>
                      <p class="text-[10px] uppercase tracking-wider text-(--muted)">{font.style || 'Regular'}</p>
                    </div>
                  </div>
                  
                  {#if font.isInstalled}
                    <div class="flex items-center gap-1.5 text-[11px] font-semibold text-(--accent)">
                      <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
                      INSTALLED
                    </div>
                  {:else}
                    <div class="flex items-center gap-1.5 text-[11px] font-semibold text-(--warn)">
                      <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>
                      MISSING
                    </div>
                  {/if}
                </div>
              {:else}
                <div class="flex flex-col items-center justify-center py-8 text-center opacity-40">
                  <p class="text-sm">No fonts detected in this project.</p>
                  <p class="text-[10px] mt-1 italic">Binary AEP support is experimental.</p>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <div class="border-t border-white/5 bg-white/2 px-6 py-4 flex justify-end">
        <button 
          class="rounded-full bg-white px-6 py-2 text-sm font-semibold text-black transition hover:bg-white/90"
          onclick={onClose}
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
