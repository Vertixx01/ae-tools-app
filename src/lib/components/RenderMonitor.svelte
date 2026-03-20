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

<div class="panel rounded-[28px] p-5 md:p-6">
  <div class="mb-5 flex items-center justify-between gap-4">
    <div>
      <p class="mono text-[11px] uppercase tracking-[0.24em] text-(--muted)">Live Render Monitor</p>
      <h2 class="mt-2 text-2xl font-semibold">After Effects process resources</h2>
    </div>
    <div class="flex items-center gap-2">
      {#if busy}
        <div class="h-2 w-2 animate-pulse rounded-full bg-(--accent)"></div>
      {/if}
      <div class={`rounded-full border px-3 py-1 text-xs ${status?.isActive ? "border-(--accent)/30 bg-(--accent)/10 text-(--accent)" : "border-white/10 bg-white/5 text-(--muted)"}`}>
        {status?.isActive ? "Active" : "Idle"}
      </div>
    </div>
  </div>

  <div class="grid gap-4">
    {#if status?.isActive}
      <div class="grid gap-4 sm:grid-cols-2">
        <div class="rounded-[24px] border border-white/8 bg-white/4 p-5">
          <p class="text-xs uppercase tracking-[0.2em] text-(--muted)">Total CPU usage</p>
          <div class="mt-3 flex items-end gap-2">
            <span class="text-4xl font-bold">{status.totalCpu.toFixed(1)}%</span>
            <div class="mb-1.5 h-1.5 flex-1 rounded-full bg-white/5 overflow-hidden">
               <div class="h-full bg-(--accent) transition-all duration-500" style={`width: ${Math.min(status.totalCpu, 100)}%`}></div>
            </div>
          </div>
        </div>

        <div class="rounded-[24px] border border-white/8 bg-white/4 p-5">
          <p class="text-xs uppercase tracking-[0.2em] text-(--muted)">Memory pressure</p>
          <div class="mt-3 flex items-end gap-2">
            <span class="text-4xl font-bold">{status.totalMemoryMb} <span class="text-lg font-medium text-(--muted)">MB</span></span>
          </div>
        </div>
      </div>

      <div class="space-y-3">
        <p class="text-xs uppercase tracking-[0.2em] text-(--muted)">Active processes</p>
        {#each status.processes as process}
          <div class="flex items-center justify-between gap-4 rounded-2xl border border-white/6 bg-white/4 px-4 py-3">
            <div class="flex flex-col gap-0.5 min-w-0">
               <div class="flex items-center gap-2">
                  <span class="truncate font-medium">{process.name}</span>
                  {#if process.isRendering}
                    <span class="rounded-full bg-(--accent)/20 px-1.5 py-0.5 text-[9px] font-bold uppercase tracking-wider text-(--accent)">Rendering</span>
                  {/if}
               </div>
               <span class="mono text-[10px] text-(--muted) tracking-wider">PID: {process.pid}</span>
            </div>
            <div class="flex items-center gap-4 text-sm">
                <div class="text-right">
                    <p class="font-semibold">{process.cpuUsage.toFixed(1)}%</p>
                    <p class="text-[10px] text-(--muted) uppercase tracking-wider">CPU</p>
                </div>
                <div class="text-right min-w-[60px]">
                    <p class="font-semibold">{process.memoryMb} MB</p>
                    <p class="text-[10px] text-(--muted) uppercase tracking-wider">RAM</p>
                </div>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="flex flex-col items-center justify-center rounded-[24px] border border-dashed border-white/10 p-12 text-center">
        <div class="mb-4 rounded-full bg-white/5 p-4">
           <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-(--muted)"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
        </div>
        <p class="text-sm text-(--muted)">No After Effects processes detected.</p>
        <p class="mt-1 text-xs text-(--muted)/60">The monitor will activate once AE is launched.</p>
      </div>
    {/if}
  </div>
</div>
