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

<div class="panel rounded-[28px] p-5 md:p-6">
  <div class="mb-5 flex items-center justify-between gap-3">
    <div>
      <p class="mono text-[11px] uppercase tracking-[0.24em] text-[color:var(--muted)]">Session mode</p>
      <h2 class="text-2xl font-semibold">Mute noisy startup apps</h2>
    </div>
    <span class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs">
      {status?.active ? "Active" : "Idle"}
    </span>
  </div>

  <p class="text-sm text-[color:var(--muted)]">
    Disable scored startup items while After Effects is open and restore them afterward.
  </p>

  <div class="mt-4 flex flex-wrap gap-3">
    <button
      class="rounded-full bg-[color:var(--accent)] px-4 py-2 text-sm font-semibold text-slate-950 transition hover:scale-[1.01] disabled:opacity-60"
      onclick={onStart}
      disabled={busy === "session-start" || status?.active}
    >
      {busy === "session-start" ? "Muting..." : "Start session mode"}
    </button>
    <button
      class="rounded-full border border-white/10 bg-white/6 px-4 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60"
      onclick={onStop}
      disabled={busy === "session-stop" || !status?.active}
    >
      {busy === "session-stop" ? "Restoring..." : "Stop session mode"}
    </button>
  </div>

  {#if status?.disabledItems.length}
    <div class="mt-4 rounded-2xl border border-white/8 bg-white/4 p-3 text-xs text-[color:var(--muted)]">
      Disabled items:
      <ul class="mt-2 list-disc pl-5">
        {#each status.disabledItems as item}
          <li>{item}</li>
        {/each}
      </ul>
    </div>
  {/if}
</div>
