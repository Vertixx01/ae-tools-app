<script lang="ts">
  import { onMount } from "svelte";
  import { scale, fade, slide } from "svelte/transition";
  import type { RenderOptions, ProjectEntry } from "$lib/types";

  interface Props {
    project: ProjectEntry;
    visible: boolean;
    onClose: () => void;
    onStartRender: (options: RenderOptions) => void;
  }

  let { project, visible, onClose, onStartRender }: Props = $props();

  // Tabs
  let activeTab = $state<"basic" | "professional">("basic");

  // Settings - Basic
  let selectedComp = $state("");
  let fileName = $state("");
  let outputPath = $state("");
  let omTemplate = $state("");

  $effect(() => {
    if (!selectedComp) selectedComp = project.compositions?.[0] || "";
    if (!fileName) fileName = project.name.replace(/\.[^/.]+$/, "");
  });

  // Settings - Professional
  let mfr = $state(true);
  let cpuPercent = $state(90);
  let maxMem = $state(80);
  let imageCache = $state(50);
  let priority = $state<RenderOptions["priority"]>("Normal");
  let rsTemplate = $state("");
  let startFrame = $state<number | undefined>(undefined);
  let endFrame = $state<number | undefined>(undefined);
  let sound = $state(true);
  let continueOnMissing = $state(true);
  let reuse = $state(true);

  onMount(() => {
    // Attempt to guess default output path if project path is available
    if (project.path) {
      const lastSlash =
        project.path.lastIndexOf("\\") !== -1
          ? project.path.lastIndexOf("\\")
          : project.path.lastIndexOf("/");
      const dir = project.path.substring(0, lastSlash);
      outputPath = dir + "\\renders\\" + fileName + ".mp4";
    }
  });

  function start() {
    const options: RenderOptions = {
      projectPath: project.path,
      comp: selectedComp || undefined,
      outputPath: outputPath || undefined,
      omTemplate: omTemplate || undefined,
      rsTemplate: rsTemplate || undefined,
      mfr,
      cpuPercent,
      maxMem,
      imageCache,
      priority,
      startFrame,
      endFrame,
      sound,
      continueOnMissing,
      reuse,
    };
    onStartRender(options);
  }

  const priorityOptions: RenderOptions["priority"][] = [
    "Low",
    "BelowNormal",
    "Normal",
    "AboveNormal",
    "High",
  ];
  const templates = [
    "(Default)",
    "H.264",
    "ProRes 422",
    "ProRes 4444",
    "Lossless",
    "QuickTime",
  ];
</script>

{#if visible}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-md p-4"
    transition:fade={{ duration: 200 }}
    onclick={(e) => e.target === e.currentTarget && onClose()}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === "Escape" && onClose()}
  >
    <div
      class="w-full max-w-3xl overflow-hidden rounded-[40px] border border-white/10 bg-[#0a0a0a] shadow-2xl flex flex-col max-h-[90vh]"
      transition:scale={{ duration: 300, start: 0.95 }}
      onclick={(e) => e.stopPropagation()}
      role="none"
    >
      <!-- Header -->
      <div
        class="px-8 pt-8 pb-6 border-b border-white/5 bg-gradient-to-br from-emerald-500/5 to-transparent"
      >
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-2xl font-bold tracking-tight">
              Render Orchestrator
            </h2>
            <p
              class="text-xs text-emerald-400/70 mono mt-1 uppercase tracking-widest font-semibold"
            >
              Optimized Core Engine
            </p>
          </div>
          <button
            class="rounded-full bg-white/5 p-2 hover:bg-white/10 transition active:scale-95"
            onclick={onClose}
            aria-label="Close dialog"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2.5"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><line x1="18" y1="6" x2="6" y2="18"></line><line
                x1="6"
                y1="6"
                x2="18"
                y2="18"
              ></line></svg
            >
          </button>
        </div>

        <!-- Tab Switcher -->
        <div class="mt-8 flex p-1.5 bg-white/5 rounded-2xl w-fit">
          <button
            class="px-6 py-2 text-sm font-bold rounded-xl transition-all {activeTab ===
            'basic'
              ? 'bg-emerald-500 text-slate-950 shadow-lg shadow-emerald-500/20'
              : 'text-(--muted) hover:text-white'}"
            onclick={() => (activeTab = "basic")}
          >
            Standard
          </button>
          <button
            class="px-6 py-2 text-sm font-bold rounded-xl transition-all {activeTab ===
            'professional'
              ? 'bg-emerald-500 text-slate-950 shadow-lg shadow-emerald-500/20'
              : 'text-(--muted) hover:text-white'}"
            onclick={() => (activeTab = "professional")}
          >
            Engine Protocol
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-8 thin-scrollbar space-y-8">
        {#if activeTab === "basic"}
          <div transition:fade={{ duration: 150 }} class="space-y-6">
            <div class="space-y-3">
              <label
                class="block text-xs font-bold uppercase tracking-widest text-(--muted) ml-1"
                for="comp-select">Target Composition</label
              >
              <div class="relative group">
                <select
                  id="comp-select"
                  bind:value={selectedComp}
                  class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500/40 transition appearance-none cursor-pointer"
                >
                  <option value="" class="bg-slate-900"
                    >Render Queue (Default)</option
                  >
                  {#each project.compositions || [] as comp}
                    <option value={comp} class="bg-slate-900">{comp}</option>
                  {/each}
                </select>
                <div
                  class="absolute right-5 top-1/2 -translate-y-1/2 pointer-events-none opacity-50"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"><path d="m6 9 6 6 6-6" /></svg
                  >
                </div>
              </div>
            </div>

            <div class="space-y-3">
              <label
                class="block text-xs font-bold uppercase tracking-widest text-(--muted) ml-1"
                for="output-template">Output Profile</label
              >
              <div class="grid grid-cols-2 lg:grid-cols-3 gap-2">
                {#each templates as t}
                  <button
                    class="px-4 py-3 rounded-xl border text-sm font-medium transition-all {(
                      t === '(Default)' ? omTemplate === '' : omTemplate === t
                    )
                      ? 'bg-emerald-500/10 border-emerald-500/50 text-emerald-400'
                      : 'bg-white/4 border-white/5 text-(--muted) hover:border-white/20'}"
                    onclick={() => (omTemplate = t === "(Default)" ? "" : t)}
                  >
                    {t}
                  </button>
                {/each}
              </div>
            </div>

            <div class="space-y-3">
              <label
                class="block text-xs font-bold uppercase tracking-widest text-(--muted) ml-1"
                for="output-path">Destination</label
              >
              <input
                id="output-path"
                type="text"
                bind:value={outputPath}
                placeholder="Auto-generated path..."
                class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-xs mono focus:outline-none focus:ring-2 focus:ring-emerald-500/40 transition"
              />
              <p class="text-[10px] text-(--muted) italic opacity-60 ml-1">
                Subfolder /renders will be used if left empty.
              </p>
            </div>
          </div>
        {:else}
          <div transition:fade={{ duration: 150 }} class="space-y-8 pb-4">
            <!-- Multi-Frame Rendering -->
            <div
              class="p-6 rounded-3xl border border-emerald-500/20 bg-emerald-500/5 space-y-5"
            >
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-bold text-emerald-400">
                    Multi-Frame Core Logic
                  </h3>
                  <p class="text-[11px] text-emerald-400/60 mt-0.5">
                    Accelerate render using all available CPU cores.
                  </p>
                </div>
                <!-- Toggle -->
                <button
                  class="w-12 h-6 rounded-full relative transition {mfr
                    ? 'bg-emerald-500'
                    : 'bg-white/10'}"
                  onclick={() => (mfr = !mfr)}
                  aria-label="Toggle Multi-Frame Rendering"
                >
                  <div
                    class="absolute top-1 left-1 w-4 h-4 rounded-full bg-white transition-all {mfr
                      ? 'translate-x-6'
                      : ''}"
                  ></div>
                </button>
              </div>

              {#if mfr}
                <div transition:slide class="pt-2 space-y-4">
                  <div class="flex items-center justify-between text-xs mb-1">
                    <span class="text-(--muted) font-bold uppercase"
                      >CPU Allocation Pool</span
                    >
                    <span class="text-emerald-400 font-bold">{cpuPercent}%</span
                    >
                  </div>
                  <input
                    type="range"
                    min="10"
                    max="100"
                    bind:value={cpuPercent}
                    class="w-full h-1.5 bg-white/10 rounded-full appearance-none cursor-pointer accent-emerald-500"
                  />
                  <div
                    class="flex justify-between text-[10px] text-(--muted) opacity-50 px-1"
                  >
                    <span>Battery Saver</span>
                    <span>Extreme Performance</span>
                  </div>
                </div>
              {/if}
            </div>

            <div class="grid grid-cols-2 gap-6">
              <div class="space-y-3">
                <label
                  class="block text-xs font-bold uppercase tracking-widest text-(--muted) ml-1"
                  for="resource-priority"
                  >Resource Priority</label
                >
                <select
                  id="resource-priority"
                  bind:value={priority}
                  class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500/40 transition appearance-none cursor-pointer"
                >
                  {#each priorityOptions as p}
                    <option value={p} class="bg-slate-900">{p}</option>
                  {/each}
                </select>
              </div>

              <div class="space-y-3">
                <label
                  class="block text-xs font-bold uppercase tracking-widest text-(--muted) ml-1"
                  for="ae-reuse-toggle"
                  >AE Master Instance</label
                >
                <button
                  id="ae-reuse-toggle"
                  class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-sm flex items-center justify-between cursor-pointer hover:bg-white/10 transition"
                  onclick={() => (reuse = !reuse)}
                >
                  <span class="text-(--muted)">Reuse Running AE</span>
                  <div
                    class="w-3 h-3 rounded-full border-2 {reuse
                      ? 'bg-emerald-500 border-emerald-500'
                      : 'border-white/20'}"
                  ></div>
                </button>
              </div>
            </div>

            <div class="space-y-4">
              <label
                class="block text-xs font-bold uppercase tracking-widest text-(--muted) ml-1"
                for="mem-alloc-range"
                >Memory Matrix (% Limit)</label
              >
              <div class="grid grid-cols-2 gap-4">
                <div class="bg-white/4 p-4 rounded-2xl border border-white/5">
                  <div class="flex justify-between items-center mb-3">
                    <span class="text-[10px] uppercase font-bold text-(--muted)"
                      >Allocatable</span
                    >
                    <span class="text-xs font-bold">{maxMem}%</span>
                  </div>
                  <input
                    id="mem-alloc-range"
                    type="range"
                    min="10"
                    max="100"
                    bind:value={maxMem}
                    class="w-full h-1 bg-white/10 rounded-full appearance-none cursor-pointer accent-white"
                  />
                </div>
                <div class="bg-white/4 p-4 rounded-2xl border border-white/5">
                  <div class="flex justify-between items-center mb-3">
                    <span class="text-[10px] uppercase font-bold text-(--muted)"
                      >Cache Buffer</span
                    >
                    <span class="text-xs font-bold">{imageCache}%</span>
                  </div>
                  <input
                    type="range"
                    min="10"
                    max="100"
                    bind:value={imageCache}
                    class="w-full h-1 bg-white/10 rounded-full appearance-none cursor-pointer accent-white"
                  />
                </div>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-6">
              <div class="space-y-3">
                <label class="block text-xs font-bold uppercase tracking-widest text-(--muted) ml-1" for="rs-custom">Custom RS Template</label>
                <div class="relative group">
                    <input 
                        id="rs-custom"
                        list="rs-templates"
                        type="text" 
                        bind:value={rsTemplate} 
                        placeholder="e.g. Best Settings"
                        class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500/40 transition hover:bg-white/8" 
                    />
                    <datalist id="rs-templates">
                        <option value="Best Settings">Best Settings (Standard)</option>
                        <option value="Draft Settings">Draft (Fast Preview)</option>
                        <option value="Multi-Machine Settings">Multi-Machine Cluster</option>
                    </datalist>
                    <div class="absolute right-5 top-1/2 -translate-y-1/2 pointer-events-none opacity-20 group-hover:opacity-40 transition-opacity">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5-5"/></svg>
                    </div>
                </div>
              </div>
              <div class="space-y-3">
                <label class="block text-xs font-bold uppercase tracking-widest text-(--muted) ml-1" for="om-custom">Custom OM Template</label>
                <div class="relative group">
                    <input 
                        id="om-custom"
                        list="om-templates"
                        type="text" 
                        bind:value={omTemplate} 
                        placeholder="e.g. Lossless"
                        class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500/40 transition hover:bg-white/8" 
                    />
                    <datalist id="om-templates">
                        <option value="Lossless">Universal Lossless</option>
                        <option value="H.264">Web Optimized (H.264)</option>
                        <option value="ProRes 422">ProRes 422 (Industry Std)</option>
                    </datalist>
                    <div class="absolute right-5 top-1/2 -translate-y-1/2 pointer-events-none opacity-20 group-hover:opacity-40 transition-opacity">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5-5"/></svg>
                    </div>
                </div>
              </div>
            </div>

            <div class="flex flex-col gap-6 pt-2">
                <div class="grid grid-cols-2 gap-6">
                  <!-- Audio Setting Tile -->
                  <button
                    class="group/toggle relative flex flex-col items-center justify-center gap-4 p-8 rounded-[32px] border transition-all duration-300 active:scale-[0.98] min-h-[140px] {sound
                      ? 'bg-indigo-500/10 border-indigo-500/30 shadow-lg shadow-indigo-500/5'
                      : 'bg-white/2 border-white/5 hover:border-white/10'}"
                    onclick={() => (sound = !sound)}
                  >
                    <div
                      class="w-12 h-12 rounded-2xl flex items-center justify-center transition-all duration-500 {sound
                        ? 'bg-indigo-500 text-slate-950 shadow-xl shadow-indigo-500/30'
                        : 'bg-white/5 text-white/40'}"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M11 5L6 9H2v6h4l5 4V5z" /><path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"/></svg>
                    </div>
                    <div class="text-center">
                      <p class="text-[11px] font-black uppercase tracking-[.15em] {sound ? 'text-indigo-400' : 'text-(--muted)'}">Audio Engine</p>
                      <p class="text-[10px] font-bold opacity-30 mt-1 uppercase">{sound ? 'Broadcasting' : 'Muted'}</p>
                    </div>
                  </button>

                  <!-- Safeguard Setting Tile -->
                  <button
                    class="group/toggle relative flex flex-col items-center justify-center gap-4 p-8 rounded-[32px] border transition-all duration-300 active:scale-[0.98] min-h-[140px] {continueOnMissing
                      ? 'bg-amber-500/10 border-amber-500/30 shadow-lg shadow-amber-500/5'
                      : 'bg-white/2 border-white/5 hover:border-white/10'}"
                    onclick={() => (continueOnMissing = !continueOnMissing)}
                  >
                    <div
                      class="w-12 h-12 rounded-2xl flex items-center justify-center transition-all duration-500 {continueOnMissing
                        ? 'bg-amber-500 text-slate-950 shadow-xl shadow-amber-500/30'
                        : 'bg-white/5 text-white/40'}"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><polyline points="13 2 13 9 20 9" /></svg>
                    </div>
                    <div class="text-center">
                      <p class="text-[11px] font-black uppercase tracking-[.15em] {continueOnMissing ? 'text-amber-500' : 'text-(--muted)'}">Safeguard</p>
                      <p class="text-[10px] font-bold opacity-30 mt-1 uppercase">{continueOnMissing ? 'Active' : 'Strict'}</p>
                    </div>
                  </button>
                </div>

                <!-- Frame Range Row (Full Width) -->
                <div class="bg-white/2 border border-white/5 rounded-[32px] p-6 flex items-center justify-between group/frames hover:bg-white/4 transition-all">
                  <div class="flex items-center gap-6">
                    <div class="w-12 h-12 rounded-2xl bg-white/5 flex items-center justify-center text-white/20">
                      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 16V4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12"/><path d="M11 10 7 14 11 18" /><path d="m15 10 4 4-4 4"/></svg>
                    </div>
                    <div class="text-left">
                      <p class="text-[11px] font-black uppercase tracking-[.2em] text-(--muted)">Sequence Range</p>
                      <p class="text-[10px] font-bold opacity-30 mt-1">SPECIFIED WORK AREA</p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3 bg-black/40 px-6 py-4 rounded-2xl border border-white/5 shadow-inner">
                    <input type="number" bind:value={startFrame} placeholder="0" class="w-16 bg-transparent text-[14px] font-black mono text-center focus:outline-none placeholder:text-white/10" />
                    <span class="opacity-20 text-[11px] font-mono mx-1">/</span>
                    <input type="number" bind:value={endFrame} placeholder="END" class="w-16 bg-transparent text-[14px] font-black mono text-center focus:outline-none placeholder:text-white/10" />
                  </div>
                </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="p-8 border-t border-white/5 bg-white/2 flex items-center justify-between"
      >
        <button
          class="text-sm font-semibold text-(--muted) hover:text-white transition"
          onclick={onClose}
        >
          Abort Protocol
        </button>
        <button
          class="group relative px-10 py-4 bg-emerald-500 rounded-2xl font-bold text-slate-950 overflow-hidden transition-all hover:scale-[1.02] hover:shadow-2xl hover:shadow-emerald-500/40 active:scale-[0.98]"
          onclick={start}
        >
          <div
            class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/20 to-white/0 -translate-x-full group-hover:animate-shimmer"
          ></div>
          <span class="relative z-10 flex items-center gap-3">
            Initialize Render Sequence
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2.5"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><path d="M5 12h14" /><path d="m12 5 7 7-7 7" /></svg
            >
          </span>
        </button>
      </div>
    </div>
  </div>
{/if}

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

  @keyframes shimmer {
    100% {
      transform: translateX(100%);
    }
  }

  :global(.animate-shimmer) {
    animation: shimmer 1.5s infinite;
  }
</style>
