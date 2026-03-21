<script lang="ts">
  import { onMount } from "svelte";
  import { scale, fade } from "svelte/transition";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { openPath } from "@tauri-apps/plugin-opener";
  
  interface LogEntry {
    message: string;
    timestamp: Date;
  }

  interface Props {
    projectName: string;
    logs: LogEntry[];
    visible: boolean;
    onClose: () => void;
  }

  let { projectName, logs, visible, onClose }: Props = $props();
  let logEnd: HTMLDivElement | null = $state(null);
  let mountTime = $state(Date.now());

  $effect(() => {
    if (visible) mountTime = Date.now();
  });

  const progress = $derived.by(() => {
    let latest = 0;
    for (const log of logs) {
      const match = log.message.match(/PROGRESS:.*\(?\s*(\d+(\.\d+)?)%\)?/);
      if (match) latest = parseFloat(match[1]);
    }
    return latest;
  });

  const finalVideoPath = $derived.by(() => {
    for (const log of logs) {
      // Matches both "Finished rendering to" and "Output: "
      const match = log.message.match(/(?:Finished rendering to|Output:)\s*"(.*)"/i);
      if (match) return match[1];
    }
    return null;
  });

  $effect(() => {
    if (visible && logs.length > 0) {
      logEnd?.scrollIntoView({ behavior: "smooth" });
    }
  });
</script>

{#if visible}
  <div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4"
    transition:fade={{ duration: 200 }}
    onclick={(e) => { 
      if (e.target === e.currentTarget && Date.now() - mountTime > 500) {
        onClose(); 
      }
    }}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === 'Escape' && onClose()}
  >
    <div 
      class="w-full max-w-4xl overflow-hidden rounded-[32px] border border-white/10 bg-[#121212] shadow-2xl" 
      transition:scale={{ duration: 300, start: 0.95 }}
      onclick={(e) => e.stopPropagation()}
      role="none"
    >
      <div class="flex flex-col border-b border-white/5 bg-white/2 px-6 py-5">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-xl font-bold truncate max-w-xl">{projectName}</h2>
            <p class="mt-1 text-xs text-(--muted)">Background Rendering via aerender.exe</p>
          </div>
          <div class="flex items-center gap-4">
            <div class="text-right">
              <span class="text-xl font-bold text-emerald-400">{Math.round(progress)}%</span>
            </div>
            <button 
              class="rounded-full p-2 hover:bg-white/5 transition"
              onclick={onClose}
              aria-label="Close"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
          </div>
        </div>
        
        <div class="mt-4 h-1.5 w-full overflow-hidden rounded-full bg-white/5">
           <div 
             class="h-full bg-emerald-400 transition-all duration-300 {progress === 0 && logs.length > 0 ? 'animate-pulse w-full' : ''}"
             style="width: {progress === 0 && logs.length > 0 ? '100%' : progress + '%'}"
           ></div>
        </div>
      </div>

      <div class="grid lg:grid-cols-2 h-[60vh]">
        <div class="flex flex-col border-r border-white/5 overflow-hidden">
            <div class="flex-1 overflow-y-auto p-4 mono text-[11px] leading-relaxed thin-scrollbar">
                <div class="space-y-1">
                  {#each logs as log}
                    <div class="flex gap-3">
                      <span class="opacity-30 select-none">[{log.timestamp.toLocaleTimeString()}]</span>
                      <p class="break-all {log.message.startsWith('ERR:') ? 'text-red-400' : 'text-emerald-400/90'}">{log.message}</p>
                    </div>
                  {/each}
                  <div bind:this={logEnd}></div>
                </div>
              </div>
        </div>

        <div class="flex flex-col bg-black/20 overflow-hidden">
           <div class="p-4 border-b border-white/5">
             <h3 class="text-sm font-semibold uppercase tracking-widest text-(--muted)">Final Preview</h3>
           </div>
           
           <div class="flex-1 flex flex-col items-center justify-center p-6 text-center">
             {#if finalVideoPath}
               <div class="w-full aspect-video rounded-2xl overflow-hidden bg-black shadow-xl ring-1 ring-white/10">
                 <video 
                   src={convertFileSrc(finalVideoPath)} 
                   controls 
                   class="w-full h-full"
                   autoplay
                 >
                   <track kind="captions" />
                 </video>
               </div>
               <p class="mt-4 text-xs mono text-(--muted) break-all px-4">{finalVideoPath}</p>
             {:else}
                <div class="flex flex-col items-center gap-4 opacity-40">
                  <div class="h-12 w-12 animate-pulse rounded-full border-2 border-white/20 border-t-white/40"></div>
                  <p class="text-sm">Waiting for render to complete...</p>
                </div>
             {/if}
           </div>
        </div>
      </div>

      <div class="border-t border-white/5 bg-white/2 px-6 py-4 flex items-center justify-end gap-3">
        {#if finalVideoPath}
          <button 
            class="rounded-full bg-emerald-500 px-6 py-2 text-sm font-bold text-slate-950 transition hover:bg-emerald-400 hover:scale-[1.02] active:scale-[0.98]"
            onclick={() => openPath(finalVideoPath!)}
          >
            Reveal in Explorer
          </button>
        {/if}
        <button 
          class="rounded-full border border-white/10 bg-white/5 px-6 py-2 text-sm font-semibold text-(--muted) transition hover:bg-white/10"
          onclick={onClose}
        >
          Close Log
        </button>
      </div>
    </div>
  </div>
{/if}
