<script lang="ts">
  import type { Recommendation } from "$lib/types";

  interface Props {
    recommendations: Recommendation[];
  }

  let { recommendations }: Props = $props();
</script>

<div class="panel rounded-[28px] p-5 md:p-6">
  <p class="mono text-[11px] uppercase tracking-[0.24em] text-[color:var(--muted)]">Recommendations</p>
  <h2 class="mt-2 text-2xl font-semibold">What matters first</h2>
  <div class="mt-5 grid gap-3">
    {#if recommendations.length}
      {#each recommendations as rec}
        <article class="rounded-[22px] border border-white/8 bg-white/4 p-4">
          <div class="flex items-start justify-between gap-3">
            <div>
              <p class="font-semibold">{rec.title}</p>
              <p class="mt-2 text-sm leading-6 text-[color:var(--muted)]">{rec.detail}</p>
            </div>
            <span class="rounded-full border border-white/10 bg-white/6 px-2 py-1 text-[10px] uppercase tracking-[0.16em]">
              {rec.severity}
            </span>
          </div>
        </article>
      {/each}
    {:else}
      <div class="rounded-[24px] border border-white/8 bg-white/4 p-4 text-[color:var(--muted)]">No immediate recommendations.</div>
    {/if}
  </div>
</div>
