<script lang="ts">
  import type { SessionStatus } from "$lib/types";

  interface Props {
    status: SessionStatus | null;
    busy: string | null;
    onStart: () => void;
    onStop: () => void;
  }

  let { status, busy, onStart, onStop }: Props = $props();
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8">
  <div class="mb-5 flex flex-col gap-2 border-b border-white/5 pb-5">
    <div class="flex items-center gap-2">
       <span class="mono text-[10px] uppercase font-bold tracking-[0.2em] text-(--muted) opacity-80">Session Mode</span>
       {#if status?.active}
          <span class="flex h-1 w-1 rounded-full bg-indigo-500 animate-pulse"></span>
       {/if}
    </div>
    
    <div class="flex items-center justify-between gap-4">
      <h2 class="text-2xl font-bold tracking-tight">Mute Network & Startup Noise</h2>
      {#if status?.active}
        <span class="flex items-center gap-1.5 rounded-full border border-indigo-500/20 bg-indigo-500/10 px-3 py-1 text-[9px] font-bold uppercase tracking-widest text-indigo-400 shadow-inner">
           <span class="h-1.5 w-1.5 rounded-full bg-indigo-500 animate-pulse"></span>
           Session Active
        </span>
      {:else}
        <span class="rounded-full border border-white/5 bg-white/5 px-3 py-1 text-[9px] font-bold uppercase tracking-widest text-(--muted) shadow-inner">
          Idle State
        </span>
      {/if}
    </div>
  </div>

  <p class="text-[11px] leading-relaxed text-(--muted) font-medium mb-6">
    Temporarily isolate the system by disabling scored startup entries while the engine is live. Target states are safely restored once the session is terminated.
  </p>

  <div class="flex flex-wrap gap-3">
    <button
      class="group flex-1 min-w-[140px] relative overflow-hidden rounded-[16px] bg-indigo-500 px-5 py-3.5 text-[10px] font-bold uppercase tracking-widest text-slate-950 transition-all hover:scale-[1.02] hover:shadow-lg hover:shadow-indigo-500/20 active:scale-[0.98] disabled:opacity-40 disabled:hover:scale-100 flex items-center justify-center gap-2"
      onclick={onStart}
      disabled={busy === "session-start" || status?.active}
    >
      {#if !status?.active && busy !== "session-start"}
         <div class="absolute inset-0 -translate-x-full bg-linear-to-r from-transparent via-white/30 to-transparent group-hover:animate-shimmer"></div>
      {/if}
      {#if busy === "session-start"}
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="animate-spin"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.92-10.44l5.36 5.36"/></svg>
      {/if}
      {busy === "session-start" ? "Init Sequence..." : "Engage Session"}
    </button>
    <button
      class="flex-1 min-w-[140px] rounded-[16px] border border-white/10 bg-white/5 px-5 py-3.5 text-[10px] font-bold uppercase tracking-widest transition-all hover:bg-white/10 hover:border-white/20 active:scale-[0.98] disabled:opacity-40 flex items-center justify-center gap-2"
      onclick={onStop}
      disabled={busy === "session-stop" || !status?.active}
    >
      {#if busy === "session-stop"}
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="animate-spin"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.92-10.44l5.36 5.36"/></svg>
      {/if}
      {busy === "session-stop" ? "Restoring..." : "Terminate Session"}
    </button>
  </div>

  {#if status?.disabledItems && status.disabledItems.length > 0}
    <div class="mt-5 rounded-[20px] border border-amber-500/20 bg-amber-500/5 p-4 shadow-inner">
      <div class="flex items-center gap-2 mb-3 pb-2 border-b border-white/5">
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-amber-500"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/></svg>
        <p class="text-[9px] font-bold uppercase tracking-widest text-amber-500/80">Quarantined Processes</p>
      </div>
      <ul class="space-y-1.5 pl-2 list-none">
        {#each status.disabledItems as item}
          <li class="relative pl-3 text-[10px] font-mono text-amber-100/90 leading-relaxed truncate opacity-80 before:absolute before:left-0 before:top-1.5 before:h-1 before:w-1 before:rounded-full before:bg-amber-500">{item}</li>
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style>
  @keyframes shimmer {
    100% {
      transform: translateX(100%);
    }
  }

  :global(.animate-shimmer) {
    animation: shimmer 1.5s infinite;
  }
</style>
