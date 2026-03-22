<script lang="ts">
  import type { SystemOverview } from "$lib/types";

  interface Props {
    system: SystemOverview | null;
  }

  let { system }: Props = $props();
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8 group shadow-2xl transition-all hover:bg-white/4">
  <div class="absolute inset-0 bg-indigo-500/5 opacity-0 group-hover:opacity-100 transition duration-500 mix-blend-screen pointer-events-none"></div>
  
  <div class="relative z-10 flex flex-col h-full gap-5">
    <div class="flex items-center gap-2 mb-2">
      <span class="mono text-[10px] font-bold uppercase tracking-[0.2em] text-indigo-400 opacity-80">Hardware Profile</span>
      <span class="flex h-1.5 w-1.5 rounded-full bg-indigo-500 animate-pulse"></span>
    </div>

    {#if system}
      <div class="flex flex-col gap-3 flex-1 justify-between">
        <div class="flex flex-col gap-2">
           <div class="flex items-center justify-between rounded-2xl border border-white/5 bg-white/2 px-4 py-3 hover:bg-white/5 transition group/row cursor-default">
             <span class="text-[10px] uppercase font-bold tracking-widest text-(--muted)">CPU</span>
             <span class="text-xs font-semibold text-white truncate max-w-[150px] md:max-w-[200px] group-hover/row:text-indigo-300 transition-colors" title={system.cpu}>{system.cpu}</span>
           </div>
           <div class="flex items-center justify-between rounded-2xl border border-white/5 bg-white/2 px-4 py-3 hover:bg-white/5 transition group/row cursor-default">
             <span class="text-[10px] uppercase font-bold tracking-widest text-(--muted)">GPU</span>
             <span class="text-xs font-semibold text-emerald-400 truncate max-w-[150px] md:max-w-[200px] group-hover/row:brightness-125 transition" title={system.gpu}>{system.gpu}</span>
           </div>
        </div>
        
        <div class="grid grid-cols-2 gap-3 mt-1">
           <div class="flex flex-col gap-1 rounded-[20px] border border-white/5 bg-white/2 p-4 pt-3 relative overflow-hidden group/tile">
             <div class="absolute inset-0 bg-indigo-500/5 opacity-0 group-hover/tile:opacity-100 transition duration-300"></div>
             <span class="relative z-10 text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted)">Memory</span>
             <span class="relative z-10 text-2xl font-bold font-mono tracking-tighter text-indigo-300">{system.ramGb} <span class="text-[10px] text-white/50 tracking-widest font-sans">GB</span></span>
             <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="absolute bottom-2 right-2 text-white/5"><rect x="2" y="2" width="20" height="20" rx="2" ry="2"/><line x1="2" y1="12" x2="22" y2="12"/><line x1="12" y1="2" x2="12" y2="22"/></svg>
           </div>
           <div class="flex flex-col justify-between gap-1 rounded-[20px] border border-white/5 bg-white/2 p-4 pt-3 relative overflow-hidden group/tile">
             <div class="absolute inset-0 bg-white/5 opacity-0 group-hover/tile:opacity-100 transition duration-300"></div>
             <span class="relative z-10 text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted)">BIOS Info</span>
             <div class="relative z-10 space-y-0.5 mt-1">
                <span class="block text-[11px] font-bold text-white truncate max-w-[100px]" title={system.biosVersion}>{system.biosVersion}</span>
                <span class="block text-[9px] text-(--muted) mono">{system.biosDate}</span>
             </div>
           </div>
        </div>

        <div class="mt-2 rounded-[20px] border border-indigo-500/20 bg-indigo-500/5 p-4 shadow-inner">
          <div class="flex items-center gap-2 mb-2">
             <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-indigo-400"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
             <p class="text-[9px] font-bold uppercase tracking-[0.2em] text-indigo-400/80">Active Power Scheme</p>
          </div>
          <p class="mono text-[11px] font-medium leading-tight text-indigo-100 uppercase">{system.powerScheme}</p>
        </div>
      </div>
    {:else}
      <div class="flex-1 flex flex-col items-center justify-center border-2 border-dashed border-white/5 rounded-2xl opacity-50 py-10 min-h-[250px]">
         <div class="h-8 w-8 animate-spin rounded-full border-2 border-indigo-500/20 border-t-indigo-500 mb-3"></div>
         <p class="text-[10px] uppercase tracking-widest font-bold text-(--muted)">Scanning Hardware Matrix...</p>
      </div>
    {/if}
  </div>
</div>
