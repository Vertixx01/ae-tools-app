<script lang="ts">
  import type { Recommendation } from "$lib/types";

  interface Props {
    recommendations: Recommendation[];
  }

  let { recommendations }: Props = $props();
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8">
  <div class="mb-5 flex flex-col gap-2 border-b border-white/5 pb-5">
    <div class="flex items-center gap-2">
       <span class="mono text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) opacity-80">Recommendations</span>
    </div>
    <div class="flex items-center justify-between gap-4">
      <h2 class="text-2xl font-bold tracking-tight">System Optimization Path</h2>
      {#if recommendations.length === 0}
        <span class="flex items-center gap-1.5 rounded-full border border-emerald-500/20 bg-emerald-500/10 px-3 py-1 text-[9px] font-bold uppercase tracking-widest text-emerald-400 shadow-inner">
           <svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
           Optimal
        </span>
      {:else}
        <span class="rounded-full border border-white/5 bg-white/5 px-3 py-1 text-[10px] uppercase font-bold tracking-widest text-(--muted) shadow-inner">
          {recommendations.length} Detected
        </span>
      {/if}
    </div>
  </div>

  <div class="grid gap-3 max-h-[300px] overflow-y-auto pr-2 thin-scrollbar">
    {#if recommendations.length}
      {#each recommendations as rec}
        <article class="group relative overflow-hidden rounded-[24px] border border-white/5 bg-white/2 p-5 transition-all outline-1 outline-transparent hover:outline-white/10 hover:bg-white/5">
          {#if rec.severity === 'high'}
             <div class="absolute inset-0 bg-rose-500/5 opacity-0 group-hover:opacity-100 transition duration-300 pointer-events-none"></div>
          {:else if rec.severity === 'medium'}
             <div class="absolute inset-0 bg-amber-500/5 opacity-0 group-hover:opacity-100 transition duration-300 pointer-events-none"></div>
          {:else}
             <div class="absolute inset-0 bg-indigo-500/5 opacity-0 group-hover:opacity-100 transition duration-300 pointer-events-none"></div>
          {/if}

          <div class="relative z-10 flex items-start justify-between gap-4">
            <div class="min-w-0 pr-4">
              <p class="font-bold text-sm text-white md:text-base tracking-tight">{rec.title}</p>
              <p class="mt-2 text-[11px] leading-relaxed text-(--muted) font-medium">{rec.detail}</p>
            </div>
            
            <div class="shrink-0 flex items-center justify-center">
              {#if rec.severity === 'high'}
                <span class="rounded-[10px] border border-rose-500/20 bg-rose-500/5 px-2.5 py-1 text-[9px] font-bold uppercase tracking-widest text-rose-400 shadow-inner flex items-center gap-1.5">
                  <span class="h-1.5 w-1.5 rounded-full bg-rose-500 animate-pulse"></span>High Priority
                </span>
              {:else if rec.severity === 'medium'}
                <span class="rounded-[10px] border border-amber-500/20 bg-amber-500/5 px-2.5 py-1 text-[9px] font-bold uppercase tracking-widest text-amber-500 shadow-inner">Elevated</span>
              {:else}
                <span class="rounded-[10px] border border-indigo-500/20 bg-indigo-500/5 px-2.5 py-1 text-[9px] font-bold uppercase tracking-widest text-indigo-400 shadow-inner">Routine</span>
              {/if}
            </div>
          </div>
        </article>
      {/each}
    {:else}
      <div class="flex flex-col items-center justify-center rounded-[24px] border-2 border-dashed border-white/5 bg-transparent py-10 text-center opacity-70">
         <div class="h-10 w-10 rounded-full bg-emerald-500/10 flex items-center justify-center mb-3 text-emerald-500">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="20 6 9 17 4 12"/></svg>
         </div>
         <p class="text-xs uppercase font-bold tracking-widest text-white">No Immediate Recommendations</p>
         <p class="text-[10px] mt-1.5 text-(--muted) font-medium max-w-[200px]">System parameters currently meet targeted stability guidelines.</p>
      </div>
    {/if}
  </div>
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
</style>
