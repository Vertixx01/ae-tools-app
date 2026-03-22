<script lang="ts">
  import type { StartupItem } from "$lib/types";

  interface Props {
    items: StartupItem[];
    totalCount: number;
    busy: string | null;
    isAdmin: boolean;
    onDisable: (item: StartupItem) => void;
  }

  let { items, totalCount, busy, isAdmin, onDisable }: Props = $props();
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8">
  <div class="mb-6 flex flex-wrap items-start justify-between gap-4 border-b border-white/5 pb-6">
    <div>
      <div class="flex items-center gap-2 mb-2">
         <span class="mono text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) opacity-80">Startup Noise</span>
         {#if items.length > 0}
            <span class="flex h-1.5 w-1.5 rounded-full bg-amber-500 animate-pulse"></span>
         {/if}
      </div>
      <h2 class="text-2xl font-bold tracking-tight">System Boot Overhead</h2>
    </div>
    <div class="flex flex-col gap-2">
      <div class="rounded-full border border-white/5 bg-white/5 px-4 py-1.5 text-[10px] uppercase font-bold tracking-widest text-(--muted) shadow-inner flex items-center justify-between gap-2">
        <span>{totalCount} total items</span>
      </div>
      {#if !isAdmin}
        <div class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-red-500/10 border border-red-500/20 text-[9px] uppercase font-bold tracking-wider text-red-400">
           <svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>
           Standard User
        </div>
      {/if}
    </div>
  </div>

  <div class="grid gap-3 max-h-[400px] overflow-y-auto pr-2 thin-scrollbar">
    {#each items as item}
      <article class="group relative overflow-hidden rounded-[24px] border border-white/5 bg-white/2 p-4 transition-all hover:bg-white/5 active:scale-[0.99]">
        <div class="absolute inset-0 bg-amber-500/5 opacity-0 group-hover:opacity-100 transition duration-300 pointer-events-none"></div>
        <div class="relative z-10 flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
          <div class="min-w-0 flex-1">
            <div class="flex flex-wrap items-center gap-2 mb-1">
              <h3 class="font-bold text-sm text-white truncate max-w-[200px] md:max-w-[300px]">{item.name}</h3>
              <span class="rounded-full border border-white/5 bg-white/5 px-2.5 py-0.5 text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted)">
                {item.kind}
              </span>
              <span class="flex items-center gap-1.5 rounded-full border border-amber-500/20 bg-amber-500/10 px-2.5 py-0.5 text-[9px] uppercase font-bold tracking-[0.2em] text-amber-500">
                Score {item.score}
              </span>
            </div>
            <p class="mono mt-1.5 truncate text-[11px] leading-relaxed text-(--muted) opacity-70">
              {item.command || item.location}
            </p>
          </div>
          <button 
            class="shrink-0 rounded-[16px] border border-white/5 bg-white/5 px-5 py-2.5 text-[10px] font-bold uppercase tracking-widest transition-all hover:bg-amber-500 hover:text-slate-950 hover:border-amber-500/30 disabled:opacity-40" 
            onclick={() => onDisable(item)} 
            disabled={busy === `startup-${item.id}`}
          >
            {busy === `startup-${item.id}` ? "Disabling..." : "Disable"}
          </button>
        </div>
      </article>
    {/each}

    {#if !items.length && totalCount > 0}
      <div class="flex flex-col items-center justify-center rounded-[32px] border-2 border-dashed border-white/5 bg-white/2 py-12 text-center opacity-70">
         <div class="h-12 w-12 rounded-full bg-emerald-500/10 flex items-center justify-center mb-3 text-emerald-500">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="20 6 9 17 4 12"></polyline></svg>
         </div>
         <p class="text-xs uppercase font-bold tracking-widest text-white">Boot Sector Optimized</p>
         <p class="text-[10px] mt-1 text-(--muted) max-w-[200px]">No high-impact startup items detected affecting Adobe software.</p>
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
