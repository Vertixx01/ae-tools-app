<script lang="ts">
  interface Props {
    healthScore: number;
    recommendationCount: number;
  }

  let { healthScore, recommendationCount }: Props = $props();

  const colorProfile = $derived(
    healthScore >= 90 ? { text: 'text-emerald-400', bg: 'bg-emerald-500', glow: 'shadow-emerald-500/50', hoverBg: 'bg-emerald-500/5' } :
    healthScore >= 70 ? { text: 'text-amber-400', bg: 'bg-amber-500', glow: 'shadow-amber-500/50', hoverBg: 'bg-amber-500/5' } :
    { text: 'text-rose-400', bg: 'bg-rose-500', glow: 'shadow-rose-500/50', hoverBg: 'bg-rose-500/5' }
  );
</script>

<div class="relative overflow-hidden rounded-[32px] border border-white/5 bg-white/2 p-7 md:p-8 group shadow-2xl transition-all hover:bg-white/4 active:scale-[0.99]">
  <div class="absolute inset-0 {colorProfile.hoverBg} opacity-0 group-hover:opacity-100 transition duration-500 mix-blend-screen"></div>
  
  <div class="relative z-10 flex flex-col h-full justify-between gap-8 w-full">
    <div class="flex items-center justify-between w-full">
       <div class="flex items-center gap-2">
         <span class="mono text-[10px] font-bold uppercase tracking-[0.2em] text-(--muted) opacity-80">System Health</span>
         {#if healthScore >= 90}
            <span class="flex h-1.5 w-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
         {/if}
       </div>
       {#if recommendationCount > 0}
         <span class="rounded-full border border-white/5 bg-white/5 px-2.5 py-1 text-[9px] font-bold uppercase tracking-widest text-(--muted) shadow-inner whitespace-nowrap">
           {recommendationCount} Recs
         </span>
       {/if}
    </div>

    <div class="flex items-end gap-2 w-full">
      <span class="text-7xl md:text-8xl font-black leading-none tracking-tighter {colorProfile.text} drop-shadow-2xl font-mono">
        {healthScore}
      </span>
      <span class="text-2xl font-bold mb-3 opacity-30 text-white">%</span>
    </div>

    <div class="h-2 w-full overflow-hidden rounded-full bg-black/50 border border-white/5 shadow-inner">
      <div 
        class="h-full rounded-full {colorProfile.bg} shadow-[0_0_12px_currentColor]" 
        style="width: {healthScore}%; box-shadow: 0 0 10px var(--tw-shadow-color);"
      ></div>
    </div>
  </div>
</div>
