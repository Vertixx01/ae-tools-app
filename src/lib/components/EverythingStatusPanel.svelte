<script lang="ts">
  import { openPath, openUrl } from "@tauri-apps/plugin-opener";
  import type { EverythingStatus } from "$lib/types";

  interface Props {
    status: EverythingStatus | null;
    busy: string | null;
    onInstall: (packageId: string) => void;
    onCheck: () => void;
    checked: boolean;
  }

  let { status, busy, onInstall, onCheck, checked }: Props = $props();

  const downloadOptions = [
    { id: "setup-x64", label: "Download Setup", hint: "Auto-launch 64-bit installer" },
    { id: "zip-x64", label: "Download Portable", hint: "Grab the portable 64-bit archive" }
  ];

  function installToken(id: string) {
    return `everything-install-${id}`;
  }

  function openEsLocation() {
    if (status?.esPath) {
      void openPath(status.esPath);
    }
  }
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-6 md:p-8 group">
  <div class="absolute inset-0 bg-blue-500/5 opacity-0 group-hover:opacity-100 transition duration-500 mix-blend-screen pointer-events-none"></div>
  
  <div class="relative z-10 flex flex-col h-full gap-5">
    <div class="flex items-start justify-between gap-4">
      <div>
        <div class="flex items-center gap-2 mb-2">
           <span class="mono text-[10px] uppercase font-bold tracking-[0.2em] text-blue-400 opacity-80">Search Engine Integration</span>
           {#if checked && status?.available}
              <span class="flex h-1.5 w-1.5 rounded-full bg-blue-400 animate-pulse"></span>
           {/if}
        </div>
        <h2 class="text-2xl font-bold tracking-tight">Voidtools Everything</h2>
      </div>

      <div class="shrink-0 flex items-center gap-2">
         {#if checked && status?.available}
           <span class="inline-flex items-center gap-1.5 rounded-full border border-blue-500/20 bg-blue-500/10 px-3 py-1 text-[9px] font-bold uppercase tracking-widest text-blue-400 shadow-inner">
              <svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="m9 11 3 3L22 4"/></svg>
              Engine Active
           </span>
         {:else if checked}
           <span class="inline-flex items-center gap-1.5 rounded-full border border-rose-500/20 bg-rose-500/10 px-3 py-1 text-[9px] font-bold uppercase tracking-widest text-rose-400 shadow-inner">
              <svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>
              Not Detected
           </span>
         {:else}
           <span class="inline-flex items-center gap-1.5 rounded-full border border-white/5 bg-white/5 px-3 py-1 text-[9px] font-bold uppercase tracking-widest text-(--muted) shadow-inner">
              Pending Check
           </span>
         {/if}
      </div>
    </div>

    <div class="flex-1">
      {#if checked}
        {#if status?.available}
          <div class="rounded-2xl border border-white/5 bg-black/40 p-4 mb-4 flex items-center justify-between gap-4 group/path hover:border-white/10 transition">
             <div class="min-w-0">
                <p class="text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted) mb-1.5">Detected Binary</p>
                <p class="mono truncate text-[11px] text-white/90">{status.esPath}</p>
             </div>
             <button
                class="shrink-0 rounded-full bg-white/5 p-2 text-(--muted) hover:text-white hover:bg-white/10 transition"
                onclick={openEsLocation}
                title="Reveal in Explorer"
             >
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
             </button>
          </div>
          <p class="text-[11px] tracking-wide leading-relaxed text-(--muted) font-medium">Everything is properly configured. Project Indexing will utilize the high-speed <span class="text-white">es.exe</span> search engine for near-instant results.</p>
        {:else}
          <div class="rounded-2xl border border-rose-500/20 bg-rose-500/5 p-4 mb-4">
             <p class="text-sm font-bold text-rose-400 mb-1">Filesystem Acceleration Unavailable</p>
             <p class="text-xs text-rose-200/80 leading-relaxed font-medium">es.exe was not detected. Project scanning will fallback to the slower filesystem walker.</p>
          </div>
          <p class="text-[11px] leading-relaxed text-(--muted) mb-4">Add Everything to your system PATH or download it manually to unlock instant multi-drive scanning capabilities.</p>
        {/if}
      {:else}
         <div class="rounded-2xl border border-amber-500/20 bg-amber-500/5 p-4 mb-4">
             <p class="text-sm font-bold text-amber-500 tracking-tight">Manual Verification Required</p>
             <p class="text-xs text-amber-100/80 leading-relaxed mt-1.5 font-medium">Check status manually to establish if the accelerated search engine is available. This avoids blocking startup while other system scans initialize.</p>
          </div>
      {/if}
    </div>

    <div class="flex flex-col gap-4 border-t border-white/5 pt-4 mt-2">
      <div class="flex flex-wrap items-center gap-3">
        <button
          class="rounded-xl border border-white/10 bg-white/5 px-5 py-2.5 text-[10px] font-bold uppercase tracking-widest transition-all hover:bg-white/10 hover:border-white/20 active:scale-95 disabled:opacity-40"
          onclick={onCheck}
          disabled={busy === "everything-status"}
        >
          {busy === "everything-status" ? "Verifying Engine..." : checked ? "Re-Check Integration" : "Verify Integration"}
        </button>
        
        {#if checked && !status?.available}
          <button
            class="rounded-xl bg-blue-500 px-5 py-2.5 text-[10px] font-bold uppercase tracking-widest text-slate-950 transition-all hover:bg-blue-400 hover:shadow-lg hover:shadow-blue-500/30 active:scale-95 ml-auto flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
            onclick={() => onInstall("setup-x64")}
            disabled={!!busy}
          >
            {#if busy?.startsWith('everything-install')}
                <svg class="animate-spin" xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.21-8.58"/></svg>
                Configuring Env...
            {:else}
                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                Auto Download & Setup
            {/if}
          </button>
        {/if}
      </div>

      {#if checked && !status?.available}
        <div class="rounded-[20px] border border-white/5 bg-black/30 p-4 mt-2">
          <div class="flex items-center justify-between mb-3 border-b border-white/5 pb-2">
            <p class="text-[9px] uppercase font-bold tracking-[0.2em] text-(--muted)">Automated Delivery</p>
            <span class="text-[9px] text-(--muted) font-mono bg-white/5 px-2 py-0.5 rounded-full border border-white/5">%TEMP%</span>
          </div>
          
          <div class="grid gap-2 grid-cols-1 sm:grid-cols-2">
            {#each downloadOptions as option}
              <button
                class="flex flex-col items-start rounded-xl border border-white/5 bg-white/2 px-4 py-3 text-left transition-all hover:border-white/10 hover:bg-white/5 active:scale-[0.98] disabled:opacity-40"
                onclick={() => onInstall(option.id)}
                disabled={!!busy}
              >
                <span class="text-[11px] font-bold text-white mb-0.5">{busy === installToken(option.id) ? "Pulling Payload..." : option.label}</span>
                <span class="text-[9px] text-(--muted) font-medium">{option.hint}</span>
              </button>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
