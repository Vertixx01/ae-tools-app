<script lang="ts">
  import type { ExpressionAuditResult, ExpressionError } from "$lib/types";
  import { fade, scale } from "svelte/transition";

  interface Props {
    result: ExpressionAuditResult | null;
    logs: ExpressionError[];
    loading: boolean;
    onClose: () => void;
  }

  let { result, logs, loading, onClose }: Props = $props();

  let activeTab = $state<"scan" | "history">("scan");

  const displayErrors = $derived.by(() => {
    if (activeTab === "scan") return result?.errors ?? [];
    return logs;
  });

  const riskyCount = $derived(result?.riskyCount ?? 0);
</script>

{#if result || loading || logs.length > 0}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/80 p-4 backdrop-blur-sm"
    transition:fade={{ duration: 200 }}
  >
    <div
      class="panel-strong w-full max-w-4xl max-h-[85vh] overflow-hidden rounded-[32px] flex flex-col shadow-2xl border-white/10"
      transition:scale={{ duration: 300, start: 0.95 }}
    >
      <!-- Header -->
      <div class="flex items-center justify-between border-b border-white/10 p-6 md:p-8">
        <div>
          <h2 class="text-3xl font-bold tracking-tight">Expression Error Scanner</h2>
          <p class="mt-2 text-sm text-(--muted) truncate max-w-lg">
            {result?.projectPath ?? "System-wide diagnostic"}
          </p>
        </div>
        <button 
          class="rounded-full p-2 hover:bg-white/5 transition"
          onclick={onClose}
          aria-label="Close"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      </div>

      <!-- Tabs -->
      <div class="px-8 pt-4 flex gap-6 border-b border-white/5">
        <button 
          class={`pb-3 text-sm font-semibold transition border-b-2 ${activeTab === "scan" ? "text-(--accent) border-(--accent)" : "text-(--muted) border-transparent hover:text-white"}`}
          onclick={() => activeTab = "scan"}
        >
          Project Scan
        </button>
        <button 
          class={`pb-3 text-sm font-semibold transition border-b-2 ${activeTab === "history" ? "text-(--accent) border-(--accent)" : "text-(--muted) border-transparent hover:text-white"}`}
          onclick={() => activeTab = "history"}
        >
          System Log History ({logs.length})
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6 md:p-8 space-y-6">
        {#if loading}
          <div class="flex flex-col items-center justify-center py-20">
            <div class="h-12 w-12 animate-spin rounded-full border-4 border-(--accent)/20 border-t-(--accent)"></div>
            <p class="mt-4 text-(--muted)">Analyzing After Effects logs and project metadata...</p>
          </div>
        {:else}
          {#if activeTab === "scan"}
            <!-- Scan Stats -->
            <div class="grid gap-4 md:grid-cols-2">
              <div class="rounded-3xl border border-white/8 bg-white/4 p-5">
                <p class="text-[10px] uppercase tracking-[0.2em] text-(--muted)">Identified Errors</p>
                <p class="mt-2 text-3xl font-bold {result?.errors.length ? 'text-(--danger)' : 'text-(--accent)'}">
                  {result?.errors.length ?? 0}
                </p>
              </div>
              <div class="rounded-3xl border border-white/8 bg-white/4 p-5">
                <p class="text-[10px] uppercase tracking-[0.2em] text-(--muted)">Risky Patterns</p>
                <p class="mt-2 text-3xl font-bold {riskyCount > 0 ? 'text-(--warn)' : 'text-(--accent)'}">
                  {riskyCount}
                </p>
                <p class="mt-1 text-xs text-(--muted)">Hardcoded layer strings detected</p>
              </div>
            </div>
          {/if}

          <!-- Error List -->
          <div class="space-y-4">
            <h3 class="text-sm font-semibold uppercase tracking-widest text-(--muted)">
              {activeTab === "scan" ? "Detected Problems" : "Runtime History"}
            </h3>
            
            {#if displayErrors.length > 0}
              {#each displayErrors as error}
                <div class="rounded-[24px] border border-white/8 bg-white/4 p-5 hover:bg-white/6 transition">
                  <div class="flex items-start justify-between gap-4">
                    <div class="min-w-0">
                      <div class="flex flex-wrap items-center gap-2">
                        <span class="rounded-full bg-(--danger)/20 px-2 py-0.5 text-[9px] font-bold uppercase tracking-wider text-(--danger)">Error</span>
                        <span class="text-xs text-(--muted)">{error.timestamp}</span>
                        {#if error.version}
                          <span class="text-[10px] mono border border-white/10 rounded-full px-2 py-0.5 text-(--muted)">v{error.version}</span>
                        {/if}
                      </div>
                      <h4 class="mt-3 font-semibold text-lg leading-tight text-white">{error.composition || 'Global Script'}</h4>
                      <div class="mt-2 space-y-1">
                         <p class="text-sm text-(--muted)">
                           <span class="text-white/40">Layer:</span> {error.layer}
                         </p>
                         <p class="text-sm text-(--muted)">
                           <span class="text-white/40">Property:</span> {error.property}
                         </p>
                      </div>
                      <div class="mt-4 rounded-xl bg-black/40 p-3 border border-white/5">
                        <p class="mono text-xs leading-5 text-(--warn)">{error.message}</p>
                      </div>
                    </div>
                  </div>
                </div>
              {/each}
            {:else}
              <div class="rounded-[32px] border border-dashed border-white/10 py-12 text-center">
                <div class="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-white/5 text-(--accent)">
                  <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
                </div>
                <h4 class="mt-4 text-xl font-semibold">Clean Health Bill</h4>
                <p class="mt-2 text-(--muted)">No expression errors detected in this {activeTab === "scan" ? 'scan' : 'session history'}.</p>
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="border-t border-white/10 bg-white/2 p-6 flex justify-end">
        <button
          class="rounded-full border border-white/10 bg-white/6 px-6 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10"
          onclick={onClose}
        >
          Dismiss
        </button>
      </div>
    </div>
  </div>
{/if}
