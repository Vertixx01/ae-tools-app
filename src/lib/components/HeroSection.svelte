<script lang="ts">
  interface Props {
    loading: boolean;
    busy: string | null;
    warningMessages: string[];
    errorMessage: string;
    canClearCaches: boolean;
    onRefresh: () => void;
    onClearAllCaches: () => void;
    onApplyPower: (mode: "stable" | "performance") => void;
  }

  let {
    loading,
    busy,
    warningMessages,
    errorMessage,
    canClearCaches,
    onRefresh,
    onClearAllCaches,
    onApplyPower,
  }: Props = $props();
</script>

<section class="panel panel-strong overflow-hidden rounded-[30px] border-white/10 px-6 py-7 md:px-8">
  <div class="space-y-5">
    <div class="flex flex-wrap items-center gap-3">
      <span class="mono rounded-full border border-[color:var(--line-strong)] bg-white/5 px-3 py-1 text-[11px] uppercase tracking-[0.28em] text-[color:var(--accent)]">
        native ae toolkit
      </span>
      <span class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-[color:var(--muted)]">
        Tauri + Svelte + Tailwind + Windows auto-detection
      </span>
    </div>

    <div class="space-y-3">
      <div class="flex items-center gap-4">
        <img src="/logo.png" alt="Aether FX Optimizer Logo" class="h-16 w-16" />
        <h1 class="max-w-4xl text-4xl font-bold leading-none md:text-6xl">Aether FX Optimizer</h1>
      </div>
      <p class="max-w-3xl text-base leading-7 text-[color:var(--muted)] md:text-lg">
        Diagnose Adobe After Effects installs, clear versioned caches, pin AE to the
        high-performance GPU profile, review startup noise, and quickly apply a safer CPU power
        cap when stability matters more than peak clocks.
      </p>
    </div>

    <div class="flex flex-wrap gap-3">
      <button class="rounded-full bg-[color:var(--accent)] px-5 py-3 text-sm font-semibold text-slate-950 transition hover:scale-[1.01] disabled:opacity-60" onclick={onRefresh} disabled={loading}>
        {loading ? "Scanning..." : "Refresh scan"}
      </button>
      <button class="rounded-full border border-white/10 bg-white/6 px-5 py-3 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60" onclick={onClearAllCaches} disabled={!canClearCaches || busy === "cache-all"}>
        {busy === "cache-all" ? "Clearing..." : "Clear all local caches"}
      </button>
      <button class="rounded-full border border-white/10 bg-white/6 px-5 py-3 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60" onclick={() => onApplyPower("stable")} disabled={busy === "power-stable"}>
        {busy === "power-stable" ? "Applying..." : "Apply 99% stability cap"}
      </button>
      <button class="rounded-full border border-white/10 bg-white/6 px-5 py-3 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60" onclick={() => onApplyPower("performance")} disabled={busy === "power-performance"}>
        {busy === "power-performance" ? "Restoring..." : "Restore 100% cap"}
      </button>
    </div>

    {#if warningMessages.length}
      <div class="grid gap-3">
        {#each warningMessages as warning}
          <div class="rounded-2xl border border-[color:var(--danger)]/35 bg-[color:rgba(255,143,124,0.08)] px-4 py-3 text-sm text-[color:var(--danger)]">
            {warning}
          </div>
        {/each}
      </div>
    {/if}

    {#if errorMessage}
      <div class="rounded-2xl border border-[color:var(--danger)]/35 bg-[color:rgba(255,143,124,0.08)] px-4 py-3 text-sm text-[color:var(--danger)]">
        {errorMessage}
      </div>
    {/if}
  </div>
</section>
