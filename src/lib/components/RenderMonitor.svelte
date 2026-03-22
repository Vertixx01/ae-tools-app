<script lang="ts">
  import type { RenderStatus } from "$lib/types";

  interface Props {
    status: RenderStatus | null;
    busy: boolean;
  }

  let { status, busy }: Props = $props();

  const activeProcess = $derived.by(() => 
    status?.processes.find(p => p.isRendering) ?? status?.processes[0]
  );
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8">
  <div class="mb-6 flex flex-wrap items-center justify-between gap-4 border-b border-white/5 pb-5">
    <div>
      <div class="flex items-center gap-2 mb-2">
         <span class="mono text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) opacity-80">Live Telemetry</span>
         {#if status?.isActive}
            <span class="flex h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
         {/if}
      </div>
      <h2 class="text-2xl font-bold tracking-tight">Active Processes</h2>
    </div>
    
    <div class="flex items-center gap-3">
      {#if busy}
        <div class="h-4 w-4 rounded-full border-2 border-(--muted)/30 border-t-(--muted) animate-spin"></div>
      {/if}
      <div class={`flex items-center gap-2 rounded-full border px-4 py-1.5 text-[10px] font-bold uppercase tracking-widest shadow-inner ${status?.isActive ? "border-emerald-500/20 bg-emerald-500/10 text-emerald-400" : "border-white/10 bg-white/5 text-(--muted)"}`}>
        {#if status?.isActive}
          <span class="h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
        {/if}
        {status?.isActive ? "Monitoring" : "Standby"}
      </div>
    </div>
  </div>

  <div class="grid gap-4">
    {#if status?.isActive}
      <div class="grid gap-4 sm:grid-cols-2">
        <div class="relative overflow-hidden rounded-[24px] border border-white/5 bg-black/40 p-5 group hover:bg-white/5 transition">
          <div class="glass-grain opacity-[0.05]"></div>
          <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted)">Allocated CPU</p>
          <div class="mt-3 flex items-end gap-3 z-10 relative">
            <span class="text-5xl font-black tracking-tighter text-emerald-400 font-mono drop-shadow-lg">{status.totalCpu.toFixed(1)}<span class="text-xl text-emerald-500/50">%</span></span>
          </div>
          <div class="mt-4 h-2 w-full rounded-full bg-black/60 border border-white/5 shadow-inner overflow-hidden z-10 relative">
             <div class="h-full bg-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.8)] transition-all duration-500 ease-out" style={`width: ${Math.min(status.totalCpu, 100)}%`}></div>
          </div>
        </div>

        <div class="relative overflow-hidden rounded-[24px] border border-white/5 bg-black/40 p-5 group hover:bg-white/5 transition">
          <div class="glass-grain opacity-[0.05]"></div>
          <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted)">Memory Pressure</p>
          <div class="mt-3 flex items-end gap-3 z-10 relative">
            <span class="text-5xl font-black tracking-tighter text-indigo-400 font-mono drop-shadow-lg">{status.totalMemoryMb}</span>
            <span class="text-[11px] font-bold uppercase tracking-widest text-indigo-500/50 mb-2">MB In Use</span>
          </div>
        </div>
      </div>

      <div class="mt-2 space-y-3">
        <p class="text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) border-b border-white/5 pb-2">Active Child Processes</p>
        {#each status.processes as process}
          <div class="flex flex-wrap items-center justify-between gap-4 rounded-[20px] border border-white/5 bg-white/2 px-5 py-4 transition-all hover:bg-white/5 hover:border-white/10 group/proc">
            <div class="flex flex-col gap-1 min-w-0">
               <div class="flex items-center gap-3">
                  <span class="truncate font-bold text-white text-base group-hover/proc:text-emerald-300 transition-colors">{process.name}</span>
                  {#if process.isRendering}
                    <span class="flex items-center gap-1.5 rounded-[8px] border border-emerald-500/20 bg-emerald-500/10 px-2.5 py-1 text-[9px] font-bold uppercase tracking-widest text-emerald-400 shadow-inner">
                      <span class="h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                      Frame Target
                    </span>
                  {/if}
               </div>
               <span class="mono text-[10px] text-(--muted) font-medium tracking-wide">PID: {process.pid}</span>
            </div>
            <div class="flex items-center gap-6 shrink-0">
                <div class="flex flex-col items-end">
                    <p class="font-bold text-sm text-white font-mono">{process.cpuUsage.toFixed(1)}%</p>
                    <p class="text-[9px] text-(--muted) font-bold uppercase tracking-widest">CPU Sync</p>
                </div>
                <div class="flex flex-col items-end min-w-[60px]">
                    <p class="font-bold text-sm text-white font-mono">{process.memoryMb} <span class="text-[10px] font-sans text-white/50">MB</span></p>
                    <p class="text-[9px] text-(--muted) font-bold uppercase tracking-widest">RAM Cache</p>
                </div>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="flex flex-col items-center justify-center rounded-[32px] border-2 border-dashed border-white/5 bg-transparent py-16 text-center opacity-70">
        <div class="mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-white/2 border border-white/5 shadow-inner text-(--muted)">
           <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
        </div>
        <p class="text-[11px] uppercase font-bold tracking-widest text-white">Telemetry Offline</p>
        <p class="mt-2 text-[10px] text-(--muted) font-medium max-w-[220px] leading-relaxed">The After Effects diagnostic monitor engages automatically upon process detection.</p>
      </div>
    {/if}
  </div>
</div>
