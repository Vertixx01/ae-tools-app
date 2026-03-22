<script lang="ts">
  interface Props {
    loading: boolean;
    busy: string | null;
    warningMessages: string[];
    errorMessage: string;
    canClearCaches: boolean;
    globalCacheCount: number;
    onRefresh: () => void;
    onClearAllCaches: () => void;
    onPurgeGlobalCaches: () => void;
    onApplyPower: (mode: "stable" | "performance") => void;
  }

  let {
    loading,
    busy,
    warningMessages,
    errorMessage,
    canClearCaches,
    globalCacheCount,
    onRefresh,
    onClearAllCaches,
    onPurgeGlobalCaches,
    onApplyPower,
  }: Props = $props();
</script>

<section class="relative overflow-hidden rounded-[40px] border border-white/5 bg-[#080808] p-8 md:p-12 shadow-2xl">
  <!-- Dynamic Background Glows -->
  <div class="absolute -top-40 -right-40 h-[600px] w-[600px] rounded-full bg-emerald-500/10 blur-[120px] pointer-events-none mix-blend-screen"></div>
  <div class="absolute -bottom-40 -left-40 h-[600px] w-[600px] rounded-full bg-indigo-500/10 blur-[120px] pointer-events-none mix-blend-screen"></div>
  
  <div class="glass-grain opacity-[0.03] transition-opacity duration-1000"></div>

  <!-- Content -->
  <div class="relative z-10 flex flex-col gap-8 md:gap-10">
    
    <!-- Badges / Pills -->
    <div class="flex flex-wrap items-center gap-3">
      <div class="flex items-center gap-2 rounded-full border border-emerald-500/20 bg-emerald-500/5 px-4 py-1.5 shadow-inner">
        <span class="h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
        <span class="mono text-[10px] uppercase font-bold tracking-[0.2em] text-emerald-400">Native Diagnostics Engine</span>
      </div>
      <span class="font-medium rounded-full border border-white/5 bg-white/2 px-4 py-1.5 text-[10px] text-(--muted) uppercase tracking-widest shadow-inner">
        Tauri • Svelte • Rust
      </span>
    </div>

    <!-- Title & Identity -->
    <div class="space-y-5">
      <div class="flex items-center gap-6">
        <div class="flex h-20 w-20 items-center justify-center rounded-[24px] bg-white/5 border border-white/10 shadow-2xl relative overflow-hidden group">
           <div class="absolute inset-0 bg-linear-to-br from-white/10 to-transparent opacity-0 group-hover:opacity-100 transition duration-500"></div>
           <!-- Adjust path if logo_square isn't correct -->
           <img src="/logo_square.png" alt="Aether FX Optimizer" class="h-12 w-12 object-contain filter drop-shadow-lg group-hover:scale-110 transition duration-500" />
        </div>
        <div>
          <h1 class="text-5xl md:text-7xl font-extrabold tracking-tighter text-transparent bg-clip-text bg-linear-to-br from-white via-white/90 to-white/40 leading-tight">
            Aether FX<br><span class="text-emerald-400">Optimizer</span>
          </h1>
        </div>
      </div>
      <p class="max-w-3xl text-sm leading-relaxed text-(--muted) font-medium md:text-base">
        Deep-level diagnostics for Adobe After Effects. Clear versioned caches, pin AE to the high-performance GPU profile, audit expressions natively, and stabilize peak CPU clocks.
      </p>
    </div>

    <!-- Primary Actions -->
    <div class="flex flex-wrap items-center gap-3 pt-2">
      <!-- Main Action -->
      <button 
        class="group relative overflow-hidden rounded-2xl bg-emerald-500 px-7 py-4 text-[11px] font-extrabold uppercase tracking-[0.15em] text-slate-950 transition-all hover:scale-[1.02] hover:shadow-2xl hover:shadow-emerald-500/20 active:scale-[0.98] disabled:opacity-50 disabled:hover:scale-100" 
        onclick={onRefresh} 
        disabled={loading}
      >
        <div class="absolute inset-0 -translate-x-full bg-linear-to-r from-transparent via-white/30 to-transparent group-hover:animate-shimmer"></div>
        <span class="relative z-10 flex items-center gap-2">
           <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" class={loading ? "animate-spin" : ""}><path d="M21 12a9 9 0 1 1-9-9 9.75 9.75 0 0 1 6.74 2.74L21 8M21 3v5h-5"/></svg>
           {loading ? "Scanning System Core..." : "Refresh Diagnostic Scan"}
        </span>
      </button>

      <!-- Secondary Actions -->
      <button 
        class="rounded-2xl border border-white/5 bg-white/2 px-6 py-4 text-[11px] font-bold uppercase tracking-widest transition-all hover:bg-white/5 hover:border-white/10 active:scale-[0.98] disabled:opacity-40" 
        onclick={onClearAllCaches} 
        disabled={!canClearCaches || busy === "cache-all"}
      >
        {busy === "cache-all" ? "Clearing..." : "Purge Local Caches"}
      </button>
      
      <div class="hidden md:block h-8 w-px bg-white/10 mx-2"></div>

      <!-- CPU Power Actions -->
      <div class="flex items-center rounded-2xl border border-white/5 p-1 bg-white/2">
        <button 
          class="rounded-[12px] px-5 py-3 text-[10px] font-bold uppercase tracking-widest transition-all hover:bg-white/5 active:scale-[0.98] disabled:opacity-40 text-indigo-300" 
          onclick={() => onApplyPower("stable")} 
          disabled={busy === "power-stable"}
        >
          {busy === "power-stable" ? "Applying..." : "Apply 99% CPU Cap"}
        </button>
        <div class="h-4 w-px bg-white/10"></div>
        <button 
          class="rounded-[12px] px-5 py-3 text-[10px] font-bold uppercase tracking-widest transition-all hover:bg-white/5 active:scale-[0.98] disabled:opacity-40 text-(--muted)" 
          onclick={() => onApplyPower("performance")} 
          disabled={busy === "power-performance"}
        >
          Restore 100%
        </button>
      </div>

      {#if globalCacheCount > 0}
         <div class="hidden md:block h-8 w-px bg-white/10 mx-2"></div>
        <button 
          class="rounded-2xl border border-rose-500/20 bg-rose-500/5 px-6 py-4 text-[11px] font-bold uppercase tracking-widest text-rose-400 transition-all hover:bg-rose-500/10 active:scale-[0.98] disabled:opacity-40" 
          onclick={onPurgeGlobalCaches} 
          disabled={busy === "purge-global"}
        >
          <span class="flex items-center gap-2">
             <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M3 6h18M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
             {busy === "purge-global" ? "Purging Matrix..." : `Global Media Wipe (${globalCacheCount})`}
          </span>
        </button>
      {/if}
    </div>

    <!-- Error & Warning Feedback -->
    {#if warningMessages.length || errorMessage}
      <div class="mt-4 grid gap-3 max-w-4xl">
        {#each warningMessages as warning}
          <div class="flex items-start gap-3 rounded-2xl border border-amber-500/20 bg-amber-500/5 p-4 shadow-inner">
          <div class="mt-0.5 rounded-full bg-amber-500/20 p-1 text-amber-500">
               <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
             </div>
             <div>
                <p class="text-[11px] font-bold uppercase tracking-widest text-amber-500/80 mb-1">System Warning</p>
                <p class="text-sm font-medium text-amber-100/90 leading-snug">{warning}</p>
             </div>
          </div>
        {/each}

        {#if errorMessage}
          <div class="flex items-start gap-3 rounded-2xl border border-rose-500/20 bg-rose-500/5 p-4 shadow-inner">
             <div class="mt-0.5 rounded-full bg-rose-500/20 p-1 text-rose-500">
               <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>
             </div>
             <div>
                <p class="text-[11px] font-bold uppercase tracking-widest text-rose-500/80 mb-1">Critical Fault</p>
                <p class="text-sm font-medium text-rose-100/90 leading-snug">{errorMessage}</p>
             </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</section>

<style>
</style>
