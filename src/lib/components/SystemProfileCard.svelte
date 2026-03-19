<script lang="ts">
  import type { SystemOverview } from "$lib/types";

  interface Props {
    system: SystemOverview | null;
  }

  let { system }: Props = $props();
</script>

<div class="panel rounded-[24px] border-white/10 p-5">
  <p class="mono text-[11px] uppercase tracking-[0.24em] text-[color:var(--muted)]">System profile</p>
  {#if system}
    <div class="mt-4 grid gap-3 text-sm">
      <div class="flex justify-between gap-4"><span class="text-[color:var(--muted)]">CPU</span><span class="text-right">{system.cpu}</span></div>
      <div class="flex justify-between gap-4"><span class="text-[color:var(--muted)]">GPU</span><span class="text-right">{system.gpu}</span></div>
      <div class="flex justify-between gap-4"><span class="text-[color:var(--muted)]">RAM</span><span>{system.ramGb} GB</span></div>
      <div class="flex justify-between gap-4"><span class="text-[color:var(--muted)]">BIOS</span><span class="text-right">{system.biosVersion}<span class="block text-xs text-[color:var(--muted)]">{system.biosDate}</span></span></div>
      <div class="rounded-2xl border border-white/8 bg-white/4 p-3">
        <p class="text-[color:var(--muted)]">Power scheme</p>
        <p class="mono mt-2 text-xs leading-6">{system.powerScheme}</p>
      </div>
    </div>
  {/if}
</div>
