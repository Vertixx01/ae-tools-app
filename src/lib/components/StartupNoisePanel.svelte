<script lang="ts">
  import type { StartupItem } from "$lib/types";

  interface Props {
    items: StartupItem[];
    totalCount: number;
    busy: string | null;
    onDisable: (item: StartupItem) => void;
  }

  let { items, totalCount, busy, onDisable }: Props = $props();
</script>

<div class="panel rounded-[28px] p-4 md:p-5">
  <div class="mb-4 flex items-center justify-between gap-4">
    <div>
      <p class="mono text-[11px] uppercase tracking-[0.24em] text-[color:var(--muted)]">Startup noise</p>
      <h2 class="mt-2 text-2xl font-semibold">Sorted by likely impact</h2>
    </div>
    <div class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs">{totalCount} items</div>
  </div>

  <div class="grid gap-3 max-h-[360px] overflow-y-auto pr-1">
    {#each items as item}
      <article class="rounded-[22px] border border-white/8 bg-white/4 p-3">
        <div class="flex items-start justify-between gap-3">
          <div class="min-w-0">
            <div class="flex flex-wrap items-center gap-2">
              <h3 class="font-semibold">{item.name}</h3>
              <span class="mono rounded-full border border-white/10 bg-white/6 px-2 py-1 text-[10px] uppercase tracking-[0.16em] text-[color:var(--muted)]">{item.kind}</span>
              <span class="rounded-full border border-white/10 bg-white/6 px-2 py-1 text-[10px] uppercase tracking-[0.16em]">score {item.score}</span>
            </div>
            <p class="mono mt-2 break-all text-xs leading-6 text-[color:var(--muted)]">{item.command || item.location}</p>
          </div>
          <button class="rounded-full border border-white/10 bg-white/6 px-4 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60" onclick={() => onDisable(item)} disabled={busy === `startup-${item.id}`}>
            {busy === `startup-${item.id}` ? "Disabling..." : "Disable"}
          </button>
        </div>
      </article>
    {/each}

    {#if !items.length}
      <div class="rounded-[24px] border border-white/8 bg-white/4 p-4 text-[color:var(--muted)]">No noisy startup items detected.</div>
    {/if}
  </div>
</div>
